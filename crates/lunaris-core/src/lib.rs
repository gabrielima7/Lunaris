//! # Lunaris Core
//!
//! Core utilities, types, and foundational abstractions for the Lunaris Game Engine.
//!
//! This crate provides:
//! - Common error types and result aliases
//! - Logging and tracing infrastructure
//! - Platform abstraction layer
//! - Core math extensions (on top of glam)
//! - Resource handles and identifiers

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(unsafe_code)]

pub mod error;
pub mod game;
pub mod id;
pub mod input;
pub mod math;
pub mod time;

pub use error::{Error, Result};
pub use game::{Game, GameConfig};
pub use input::{Input, Key, MouseButton};
pub use math::{Color, Rect, Transform2D, Transform3D, Vec2, Vec3};

/// Lunaris Engine version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize core systems (logging, etc.)
///
/// # Errors
///
/// Returns an error if initialization fails (e.g., logging already initialized)
pub fn init() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .try_init()
        .map_err(|e| Error::Init(e.to_string()))?;

    tracing::info!("Lunaris Engine v{VERSION} initialized");
    Ok(())
}
