//! # Lunaris Runtime
//!
//! The main runtime and application framework for the Lunaris Game Engine.
//!
//! Provides window management, event handling, and the game loop.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod ai;
pub mod audio;
pub mod crowd;
pub mod example_game;
pub mod network;
pub mod perception;
pub mod plugin;
pub mod save;
pub mod window;

pub use ai::{BehaviorContext, NavAgent, NavMesh, NavPath};
pub use audio::{AudioListener, AudioSource, AudioSystem};
pub use example_game::ExampleGame;
pub use network::{NetworkClient, NetworkServer, NetworkConfig};
pub use plugin::{Plugin, PluginApp, PluginId, PluginManager};
pub use save::{SaveData, SaveSystem};
pub use window::{AppRunner, Application, Window, WindowConfig, WindowState};

use lunaris_core::Result;

/// Runtime configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Application name
    pub name: String,
    /// Window configuration
    pub window: WindowConfig,
    /// Target frame rate (0 = unlimited)
    pub target_fps: u32,
    /// Enable hot reload
    pub hot_reload: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            name: String::from("Lunaris Application"),
            window: WindowConfig::default(),
            target_fps: 60,
            hot_reload: cfg!(debug_assertions),
        }
    }
}

/// Initialize all engine subsystems
///
/// # Errors
///
/// Returns an error if any subsystem fails to initialize
pub fn init() -> Result<()> {
    lunaris_core::init()?;
    tracing::info!("Lunaris Engine v{}", lunaris_core::VERSION);
    tracing::info!("Runtime initialized");
    Ok(())
}

/// Convenience macro to run a game
#[macro_export]
macro_rules! run_game {
    ($game:expr) => {{
        let config = $crate::WindowConfig::default();
        $crate::AppRunner::new($game, config).run()
    }};
    ($game:expr, $config:expr) => {{
        $crate::AppRunner::new($game, $config).run()
    }};
}
