//! Plugin system for extending the engine
//!
//! Allows modular extension of engine functionality.

use lunaris_core::Result;
use std::any::Any;

/// Plugin identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PluginId {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
}

impl PluginId {
    /// Create a new plugin ID
    #[must_use]
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }
}

/// Plugin trait for engine extensions
pub trait Plugin: Any + Send + Sync {
    /// Get the plugin ID
    fn id(&self) -> PluginId;

    /// Get plugin dependencies
    fn dependencies(&self) -> Vec<PluginId> {
        Vec::new()
    }

    /// Called when the plugin is loaded
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails
    fn on_load(&mut self, app: &mut PluginApp) -> Result<()>;

    /// Called when the plugin is unloaded
    fn on_unload(&mut self, app: &mut PluginApp);

    /// Called every frame
    fn update(&mut self, _app: &mut PluginApp, _delta_time: f32) {}

    /// Get plugin as Any for downcasting
    fn as_any(&self) -> &dyn Any;

    /// Get plugin as Any mut
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Application interface for plugins
pub struct PluginApp {
    /// Registered systems
    systems: Vec<Box<dyn FnMut(f32)>>,
    /// Plugin state storage
    state: std::collections::HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl Default for PluginApp {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginApp {
    /// Create a new plugin app
    #[must_use]
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
            state: std::collections::HashMap::new(),
        }
    }

    /// Register a system
    pub fn add_system(&mut self, system: impl FnMut(f32) + 'static) {
        self.systems.push(Box::new(system));
    }

    /// Store plugin state
    pub fn set_state<T: Any + Send + Sync>(&mut self, key: &str, value: T) {
        self.state.insert(key.to_string(), Box::new(value));
    }

    /// Get plugin state
    #[must_use]
    pub fn get_state<T: Any + Send + Sync>(&self, key: &str) -> Option<&T> {
        self.state.get(key)?.downcast_ref::<T>()
    }

    /// Get mutable plugin state
    pub fn get_state_mut<T: Any + Send + Sync>(&mut self, key: &str) -> Option<&mut T> {
        self.state.get_mut(key)?.downcast_mut::<T>()
    }

    /// Run all systems
    pub fn run_systems(&mut self, delta_time: f32) {
        for system in &mut self.systems {
            system(delta_time);
        }
    }
}

/// Plugin manager
pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    app: PluginApp,
    load_order: Vec<PluginId>,
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginManager {
    /// Create a new plugin manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            app: PluginApp::new(),
            load_order: Vec::new(),
        }
    }

    /// Register a plugin
    ///
    /// # Errors
    ///
    /// Returns an error if the plugin fails to load
    pub fn register<P: Plugin + 'static>(&mut self, mut plugin: P) -> Result<()> {
        let id = plugin.id();
        tracing::info!("Loading plugin: {} v{}", id.name, id.version);

        // Check dependencies
        for dep in plugin.dependencies() {
            if !self.load_order.contains(&dep) {
                return Err(lunaris_core::Error::Plugin(format!(
                    "Missing dependency: {} v{}",
                    dep.name, dep.version
                )));
            }
        }

        // Load the plugin
        plugin.on_load(&mut self.app)?;

        self.load_order.push(id);
        self.plugins.push(Box::new(plugin));

        Ok(())
    }

    /// Unload all plugins
    pub fn unload_all(&mut self) {
        for plugin in self.plugins.iter_mut().rev() {
            tracing::info!("Unloading plugin: {}", plugin.id().name);
            plugin.on_unload(&mut self.app);
        }
        self.plugins.clear();
        self.load_order.clear();
    }

    /// Update all plugins
    pub fn update(&mut self, delta_time: f32) {
        for plugin in &mut self.plugins {
            plugin.update(&mut self.app, delta_time);
        }
        self.app.run_systems(delta_time);
    }

    /// Get a plugin by type
    #[must_use]
    pub fn get<P: Plugin + 'static>(&self) -> Option<&P> {
        for plugin in &self.plugins {
            if let Some(p) = plugin.as_any().downcast_ref::<P>() {
                return Some(p);
            }
        }
        None
    }

    /// Get a mutable plugin by type
    pub fn get_mut<P: Plugin + 'static>(&mut self) -> Option<&mut P> {
        for plugin in &mut self.plugins {
            if let Some(p) = plugin.as_any_mut().downcast_mut::<P>() {
                return Some(p);
            }
        }
        None
    }

    /// Get the number of loaded plugins
    #[must_use]
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }
}

/// Built-in physics plugin
pub struct PhysicsPlugin {
    gravity: lunaris_core::math::Vec3,
}

impl Default for PhysicsPlugin {
    fn default() -> Self {
        Self {
            gravity: lunaris_core::math::Vec3::new(0.0, -9.81, 0.0),
        }
    }
}

impl Plugin for PhysicsPlugin {
    fn id(&self) -> PluginId {
        PluginId::new("lunaris-physics", "0.1.0")
    }

    fn on_load(&mut self, _app: &mut PluginApp) -> Result<()> {
        tracing::info!("Physics plugin loaded with gravity: {:?}", self.gravity);
        Ok(())
    }

    fn on_unload(&mut self, _app: &mut PluginApp) {
        tracing::info!("Physics plugin unloaded");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Built-in audio plugin
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn id(&self) -> PluginId {
        PluginId::new("lunaris-audio", "0.1.0")
    }

    fn on_load(&mut self, _app: &mut PluginApp) -> Result<()> {
        tracing::info!("Audio plugin loaded");
        Ok(())
    }

    fn on_unload(&mut self, _app: &mut PluginApp) {
        tracing::info!("Audio plugin unloaded");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
