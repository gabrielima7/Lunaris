//! # Lunaris Assets
//!
//! Asset loading, caching, and hot-reloading for the Lunaris Game Engine.
//!
//! ## Features
//!
//! - Async asset loading
//! - Asset caching with reference counting
//! - Hot-reloading in development
//! - Custom asset types
//! - Asset dependencies

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod handle;
pub mod loader;
pub mod manager;

pub use handle::{AssetHandle, AssetId, AssetState};
pub use loader::AssetLoader;
pub use manager::AssetManager;

use lunaris_core::Result;

/// Asset types supported by the engine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetType {
    /// Texture/Image
    Texture,
    /// Audio clip
    Audio,
    /// 3D Model/Mesh
    Model,
    /// Shader
    Shader,
    /// Font
    Font,
    /// Scene/Level
    Scene,
    /// Script
    Script,
    /// Material
    Material,
    /// Animation
    Animation,
    /// Generic binary data
    Binary,
    /// JSON data
    Json,
}

impl AssetType {
    /// Get asset type from file extension
    #[must_use]
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "png" | "jpg" | "jpeg" | "bmp" | "tga" | "webp" => Some(Self::Texture),
            "wav" | "ogg" | "mp3" | "flac" => Some(Self::Audio),
            "gltf" | "glb" | "obj" | "fbx" => Some(Self::Model),
            "wgsl" | "glsl" | "hlsl" => Some(Self::Shader),
            "ttf" | "otf" => Some(Self::Font),
            "scene" | "level" => Some(Self::Scene),
            "lua" => Some(Self::Script),
            "mat" | "material" => Some(Self::Material),
            "anim" => Some(Self::Animation),
            "bin" => Some(Self::Binary),
            "json" => Some(Self::Json),
            _ => None,
        }
    }
}

/// Initialize the asset subsystem
///
/// # Errors
///
/// Returns an error if initialization fails
pub fn init() -> Result<()> {
    tracing::info!("Asset subsystem initialized");
    Ok(())
}
