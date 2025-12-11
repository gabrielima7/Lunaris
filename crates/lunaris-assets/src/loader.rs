//! Asset loaders

use crate::{AssetHandle, AssetId, AssetState, AssetType};
use lunaris_core::Result;
use std::path::Path;

/// Trait for loading assets of a specific type
pub trait AssetLoader: Send + Sync {
    /// The type of asset this loader produces
    type Asset: Send + Sync + 'static;

    /// Get the extensions this loader handles
    fn extensions(&self) -> &[&str];

    /// Load an asset from bytes
    fn load(&self, path: &Path, bytes: &[u8]) -> Result<Self::Asset>;
}

/// Built-in texture loader
#[derive(Debug, Default)]
pub struct TextureLoader;

impl AssetLoader for TextureLoader {
    type Asset = TextureAsset;

    fn extensions(&self) -> &[&str] {
        &["png", "jpg", "jpeg", "bmp", "tga", "webp"]
    }

    fn load(&self, path: &Path, bytes: &[u8]) -> Result<Self::Asset> {
        // In real implementation, would decode image
        tracing::debug!("Loading texture: {:?} ({} bytes)", path, bytes.len());
        Ok(TextureAsset {
            width: 256,
            height: 256,
            format: TextureFormat::Rgba8,
            data: bytes.to_vec(),
        })
    }
}

/// Texture asset data
#[derive(Debug, Clone)]
pub struct TextureAsset {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Pixel format
    pub format: TextureFormat,
    /// Raw pixel data
    pub data: Vec<u8>,
}

/// Texture formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    /// RGBA 8-bit
    Rgba8,
    /// RGB 8-bit
    Rgb8,
    /// Grayscale
    R8,
}

/// Built-in audio loader
#[derive(Debug, Default)]
pub struct AudioLoader;

impl AssetLoader for AudioLoader {
    type Asset = AudioAsset;

    fn extensions(&self) -> &[&str] {
        &["wav", "ogg", "mp3"]
    }

    fn load(&self, path: &Path, bytes: &[u8]) -> Result<Self::Asset> {
        tracing::debug!("Loading audio: {:?} ({} bytes)", path, bytes.len());
        Ok(AudioAsset {
            sample_rate: 44100,
            channels: 2,
            samples: Vec::new(), // Would decode audio
        })
    }
}

/// Audio asset data
#[derive(Debug, Clone)]
pub struct AudioAsset {
    /// Sample rate
    pub sample_rate: u32,
    /// Number of channels
    pub channels: u16,
    /// Audio samples
    pub samples: Vec<f32>,
}

/// Built-in JSON loader
#[derive(Debug, Default)]
pub struct JsonLoader;

impl AssetLoader for JsonLoader {
    type Asset = serde_json::Value;

    fn extensions(&self) -> &[&str] {
        &["json"]
    }

    fn load(&self, path: &Path, bytes: &[u8]) -> Result<Self::Asset> {
        tracing::debug!("Loading JSON: {:?}", path);
        serde_json::from_slice(bytes)
            .map_err(|e| lunaris_core::Error::Asset(e.to_string()))
    }
}

/// Built-in script loader
#[derive(Debug, Default)]
pub struct ScriptLoader;

impl AssetLoader for ScriptLoader {
    type Asset = ScriptAsset;

    fn extensions(&self) -> &[&str] {
        &["lua"]
    }

    fn load(&self, path: &Path, bytes: &[u8]) -> Result<Self::Asset> {
        tracing::debug!("Loading script: {:?}", path);
        let source = String::from_utf8(bytes.to_vec())
            .map_err(|e| lunaris_core::Error::Asset(e.to_string()))?;
        Ok(ScriptAsset { source })
    }
}

/// Script asset data
#[derive(Debug, Clone)]
pub struct ScriptAsset {
    /// Lua source code
    pub source: String,
}
