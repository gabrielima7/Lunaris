//! Lua sandbox implementation
//!
//! Provides a secure, resource-limited Lua execution environment.

use crate::capabilities::{CapabilitySet, TrustLevel};
use crate::error::{ScriptError, ScriptResult};
use mlua::{Function, Lua, Result as LuaResult, StdLib, Table, Value};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Configuration for the Lua sandbox
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Maximum instructions before timeout (0 = unlimited)
    pub max_instructions: u64,
    /// Maximum memory in bytes (0 = unlimited)
    pub max_memory: usize,
    /// Maximum stack depth
    pub max_stack_depth: u32,
    /// Trust level for scripts
    pub trust_level: TrustLevel,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            max_instructions: 10_000_000, // 10M instructions per execution
            max_memory: 64 * 1024 * 1024, // 64 MB
            max_stack_depth: 256,
            trust_level: TrustLevel::Untrusted,
        }
    }
}

impl SandboxConfig {
    /// Create a config for trusted scripts (development)
    #[must_use]
    pub fn trusted() -> Self {
        Self {
            max_instructions: 0,
            max_memory: 0,
            max_stack_depth: 512,
            trust_level: TrustLevel::Trusted,
        }
    }

    /// Create a config for verified scripts
    #[must_use]
    pub fn verified() -> Self {
        Self {
            max_instructions: 50_000_000,
            max_memory: 128 * 1024 * 1024,
            max_stack_depth: 256,
            trust_level: TrustLevel::Verified,
        }
    }
}

/// A script execution context
#[derive(Debug)]
pub struct ScriptContext {
    /// Unique context ID
    pub id: lunaris_core::id::Id,
    /// Capability set for this context
    pub capabilities: CapabilitySet,
    /// Instruction counter
    instruction_count: Arc<AtomicU64>,
}

impl ScriptContext {
    /// Create a new script context
    #[must_use]
    pub fn new(trust_level: TrustLevel) -> Self {
        Self {
            id: lunaris_core::id::Id::new(),
            capabilities: CapabilitySet::new(trust_level),
            instruction_count: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Get the current instruction count
    #[must_use]
    pub fn instructions(&self) -> u64 {
        self.instruction_count.load(Ordering::Relaxed)
    }

    /// Reset the instruction counter
    pub fn reset_counter(&self) {
        self.instruction_count.store(0, Ordering::Relaxed);
    }
}

/// The main script engine
pub struct ScriptEngine {
    lua: Lua,
    config: SandboxConfig,
    context: ScriptContext,
}

impl ScriptEngine {
    /// Create a new script engine with the given configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the Lua state cannot be initialized
    pub fn new(config: SandboxConfig) -> ScriptResult<Self> {
        // Only load safe standard libraries
        let libs = StdLib::TABLE
            | StdLib::STRING
            | StdLib::MATH
            | StdLib::COROUTINE;

        let lua = Lua::new_with(libs, mlua::LuaOptions::default())
            .map_err(|e| ScriptError::SandboxInit(e.to_string()))?;

        // Set memory limit if configured
        if config.max_memory > 0 {
            lua.set_memory_limit(config.max_memory)?;
        }

        let context = ScriptContext::new(config.trust_level);

        let mut engine = Self { lua, config, context };
        engine.setup_sandbox()?;

        Ok(engine)
    }

    /// Set up the sandbox environment
    fn setup_sandbox(&mut self) -> ScriptResult<()> {
        // Remove potentially dangerous globals
        self.remove_dangerous_globals()?;

        // Set up instruction limit hook if configured
        if self.config.max_instructions > 0 {
            self.setup_instruction_limit()?;
        }

        // Register safe API functions based on capabilities
        self.register_safe_apis()?;

        Ok(())
    }

    /// Remove dangerous global functions
    fn remove_dangerous_globals(&self) -> ScriptResult<()> {
        let globals = self.lua.globals();

        // Functions that must never be available
        let dangerous = [
            "dofile",
            "loadfile",
            "load",
            "loadstring",
            "rawequal",
            "rawget",
            "rawset",
            "rawlen",
            "collectgarbage",
            "getfenv",
            "setfenv",
            "getmetatable",
            "setmetatable",
            "newproxy",
        ];

        for name in dangerous {
            globals.set(name, Value::Nil)?;
        }

        // Remove os library entirely (if it was loaded)
        globals.set("os", Value::Nil)?;
        // Remove io library entirely
        globals.set("io", Value::Nil)?;
        // Remove debug library entirely
        globals.set("debug", Value::Nil)?;
        // Remove package library
        globals.set("package", Value::Nil)?;

        Ok(())
    }

    /// Set up instruction counting hook
    fn setup_instruction_limit(&mut self) -> ScriptResult<()> {
        let max_instructions = self.config.max_instructions;
        let counter = Arc::clone(&self.context.instruction_count);

        self.lua.set_hook(
            mlua::HookTriggers::new().every_nth_instruction(1000),
            move |_lua, _debug| {
                let count = counter.fetch_add(1000, Ordering::Relaxed);
                if count >= max_instructions {
                    Err(mlua::Error::RuntimeError(format!(
                        "Instruction limit exceeded: {count} >= {max_instructions}"
                    )))
                } else {
                    Ok(())
                }
            },
        )?;

        Ok(())
    }

    /// Register safe API functions
    fn register_safe_apis(&self) -> ScriptResult<()> {
        let globals = self.lua.globals();

        // Create lunaris namespace
        let lunaris = self.lua.create_table()?;

        // Always available: safe print that goes through tracing
        let safe_print = self.lua.create_function(|_, args: mlua::Variadic<Value>| {
            let output: Vec<String> = args
                .iter()
                .map(|v| format!("{v:?}"))
                .collect();
            tracing::info!(target: "lua", "{}", output.join("\t"));
            Ok(())
        })?;
        globals.set("print", safe_print)?;

        // Add version info
        lunaris.set("version", lunaris_core::VERSION)?;

        // Time API (always available)
        let time_table = self.lua.create_table()?;
        time_table.set(
            "now",
            self.lua.create_function(|_, ()| {
                Ok(std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs_f64())
                    .unwrap_or(0.0))
            })?,
        )?;
        lunaris.set("time", time_table)?;

        // Math extensions
        let math_ext = self.lua.create_table()?;
        math_ext.set(
            "lerp",
            self.lua
                .create_function(|_, (a, b, t): (f64, f64, f64)| Ok(a + (b - a) * t))?,
        )?;
        math_ext.set(
            "clamp",
            self.lua.create_function(|_, (x, min, max): (f64, f64, f64)| {
                Ok(x.max(min).min(max))
            })?,
        )?;
        lunaris.set("math", math_ext)?;

        globals.set("lunaris", lunaris)?;

        Ok(())
    }

    /// Execute a Lua script
    ///
    /// # Errors
    ///
    /// Returns an error if the script fails to compile or execute
    pub fn run_script(&self, source: &str) -> ScriptResult<()> {
        self.context.reset_counter();
        self.lua
            .load(source)
            .exec()
            .map_err(ScriptError::from)
    }

    /// Execute a Lua script and return a value
    ///
    /// # Errors
    ///
    /// Returns an error if the script fails or the return type doesn't match
    pub fn eval<T: for<'lua> mlua::FromLua<'lua>>(&self, source: &str) -> ScriptResult<T> {
        self.context.reset_counter();
        self.lua
            .load(source)
            .eval()
            .map_err(ScriptError::from)
    }

    /// Get a global function from the Lua state
    ///
    /// # Errors
    ///
    /// Returns an error if the function doesn't exist
    pub fn get_function(&self, name: &str) -> ScriptResult<Function> {
        self.lua
            .globals()
            .get::<_, Function>(name)
            .map_err(ScriptError::from)
    }

    /// Get the current script context
    #[must_use]
    pub const fn context(&self) -> &ScriptContext {
        &self.context
    }

    /// Get the sandbox configuration
    #[must_use]
    pub const fn config(&self) -> &SandboxConfig {
        &self.config
    }
}

impl std::fmt::Debug for ScriptEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScriptEngine")
            .field("config", &self.config)
            .field("context", &self.context)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_script_execution() {
        let engine = ScriptEngine::new(SandboxConfig::default()).unwrap();
        engine.run_script("local x = 1 + 1").unwrap();
    }

    #[test]
    fn eval_returns_value() {
        let engine = ScriptEngine::new(SandboxConfig::default()).unwrap();
        let result: i32 = engine.eval("return 42").unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn dangerous_functions_removed() {
        let engine = ScriptEngine::new(SandboxConfig::default()).unwrap();

        // These should fail because the functions are removed
        assert!(engine.run_script("dofile('test.lua')").is_err());
        assert!(engine.run_script("os.execute('ls')").is_err());
        assert!(engine.run_script("io.open('test.txt')").is_err());
    }

    #[test]
    fn lunaris_api_available() {
        let engine = ScriptEngine::new(SandboxConfig::default()).unwrap();

        // Version should be available
        let version: String = engine.eval("return lunaris.version").unwrap();
        assert!(!version.is_empty());

        // Math extensions should work
        let lerp: f64 = engine.eval("return lunaris.math.lerp(0, 10, 0.5)").unwrap();
        assert!((lerp - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn instruction_limit_enforced() {
        let config = SandboxConfig {
            max_instructions: 1000,
            ..Default::default()
        };
        let engine = ScriptEngine::new(config).unwrap();

        // Infinite loop should be stopped
        let result = engine.run_script("while true do end");
        assert!(result.is_err());
    }
}
