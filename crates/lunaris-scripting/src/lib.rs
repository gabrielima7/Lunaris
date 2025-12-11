//! # Lunaris Scripting
//!
//! Sandboxed Lua scripting system for the Lunaris Game Engine.
//!
//! This crate provides:
//! - Secure Lua 5.4 sandbox with resource limits
//! - Capability-based API exposure
//! - Script context management
//! - Safe Rust/Lua interop
//!
//! ## Security Model
//!
//! Scripts are isolated and cannot:
//! - Access the file system directly
//! - Execute system commands
//! - Access raw memory
//! - Make network requests
//!
//! ## Example
//!
//! ```no_run
//! use lunaris_scripting::{ScriptEngine, SandboxConfig};
//!
//! let config = SandboxConfig::default();
//! let engine = ScriptEngine::new(config).unwrap();
//!
//! engine.run_script(r#"
//!     print("Hello from Lua!")
//! "#).unwrap();
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod capabilities;
pub mod error;
pub mod game_api;
pub mod sandbox;

pub use error::{ScriptError, ScriptResult};
pub use sandbox::{SandboxConfig, ScriptContext, ScriptEngine};
