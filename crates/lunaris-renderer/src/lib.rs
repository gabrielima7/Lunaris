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

pub mod animation;
pub mod camera;
pub mod debug_draw;
pub mod decal;
pub mod facial;
pub mod foliage;
pub mod gi;
pub mod gpu;
pub mod ik;
pub mod instancing;
pub mod lod;
pub mod material;
pub mod mesh;
pub mod motion_matching;
pub mod particles;
pub mod pipeline2d;
pub mod pipeline3d;
pub mod postprocess;
pub mod raytracing;
pub mod root_motion;
pub mod shadows;
pub mod ssr;
pub mod sss;
pub mod terrain;
pub mod texture;
pub mod vfx_graph;
pub mod volumetric;
pub mod water;

pub use animation::{AnimationClip, AnimationStateMachine, Skeleton, SkeletalAnimator};
pub use camera::{Camera2D, Camera3D, CameraUniform};
pub use debug_draw::{DebugDraw, DebugDraw2D, DebugShape};
pub use gpu::{GraphicsConfig, GraphicsContext, GpuInfo, Vertex2D, Vertex3D};
pub use lod::{CullingSystem, Frustum, LodGroup};
pub use material::{BlendMode, Material, MaterialId, ShaderId, ShaderSource};
pub use mesh::{Mesh, MeshId, MeshManager, Model};
pub use particles::{EmitterConfig, Particle, ParticleEmitter, ParticleSystem};
pub use pipeline2d::{Render2D, RenderStats, SpriteBatch, SpriteInstance};
pub use pipeline3d::{Light, LightType, MeshInstance, Render3D, RenderStats3D};
pub use postprocess::{Bloom, ColorGrading, PostProcessStack, ToneMapping};
pub use terrain::{Heightmap, Terrain, TerrainConfig};
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
