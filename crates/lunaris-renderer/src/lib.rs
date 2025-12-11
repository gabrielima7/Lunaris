//! # Lunaris Renderer
//!
//! GPU-accelerated rendering system for the Lunaris Game Engine.
//!
//! Built on wgpu for cross-platform graphics support (Vulkan, Metal, DX12, WebGPU).
//!
//! ## Features
//!
//! - 2D sprite rendering with batching
//! - 3D mesh rendering with PBR materials
//! - Camera systems (orthographic and perspective)
//! - Texture and sprite atlas management
//! - Animation system

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod camera;
pub mod gpu;
pub mod material;
pub mod texture;

pub use camera::{Camera2D, Camera3D, CameraUniform};
pub use gpu::{GraphicsConfig, GraphicsContext, GpuInfo, Vertex2D, Vertex3D};
pub use material::{BlendMode, Material, MaterialId, ShaderId, ShaderSource};
pub use texture::{AnimationPlayer, Sprite, SpriteAnimation, SpriteAtlas, TextureId, TextureInfo};

use lunaris_core::Result;

/// Initialize the renderer subsystem
///
/// # Errors
///
/// Returns an error if GPU initialization fails
pub fn init() -> Result<()> {
    tracing::info!("Renderer subsystem initialized");
    Ok(())
}
