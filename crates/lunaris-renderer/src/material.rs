//! Material and shader system

use lunaris_core::{id::Id, math::Color};
use std::collections::HashMap;

/// Shader handle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderId(pub Id);

/// Material handle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialId(pub Id);

/// Shader stage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderStage {
    /// Vertex shader
    Vertex,
    /// Fragment shader
    Fragment,
    /// Compute shader
    Compute,
}

/// Shader source
#[derive(Debug, Clone)]
pub struct ShaderSource {
    /// Shader name
    pub name: String,
    /// WGSL source code
    pub source: String,
    /// Entry point functions
    pub entry_points: HashMap<ShaderStage, String>,
}

impl ShaderSource {
    /// Create a new shader source
    #[must_use]
    pub fn new(name: impl Into<String>, source: impl Into<String>) -> Self {
        let mut entry_points = HashMap::new();
        entry_points.insert(ShaderStage::Vertex, "vs_main".to_string());
        entry_points.insert(ShaderStage::Fragment, "fs_main".to_string());

        Self {
            name: name.into(),
            source: source.into(),
            entry_points,
        }
    }
}

/// Material property value
#[derive(Debug, Clone)]
pub enum MaterialProperty {
    /// Float value
    Float(f32),
    /// Vector2
    Vec2([f32; 2]),
    /// Vector3
    Vec3([f32; 3]),
    /// Vector4 / Color
    Vec4([f32; 4]),
    /// Integer
    Int(i32),
    /// Texture reference
    Texture(super::texture::TextureId),
}

/// Blend mode for rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BlendMode {
    /// No blending (opaque)
    #[default]
    Opaque,
    /// Alpha blending
    Alpha,
    /// Additive blending
    Additive,
    /// Multiply blending
    Multiply,
    /// Pre-multiplied alpha
    Premultiplied,
}

/// Material definition
#[derive(Debug, Clone)]
pub struct Material {
    /// Material name
    pub name: String,
    /// Shader to use
    pub shader: ShaderId,
    /// Properties
    pub properties: HashMap<String, MaterialProperty>,
    /// Blend mode
    pub blend_mode: BlendMode,
    /// Double-sided rendering
    pub double_sided: bool,
    /// Depth write enabled
    pub depth_write: bool,
    /// Depth test enabled
    pub depth_test: bool,
}

impl Material {
    /// Create a new material with a shader
    #[must_use]
    pub fn new(name: impl Into<String>, shader: ShaderId) -> Self {
        Self {
            name: name.into(),
            shader,
            properties: HashMap::new(),
            blend_mode: BlendMode::Opaque,
            double_sided: false,
            depth_write: true,
            depth_test: true,
        }
    }

    /// Set a float property
    pub fn set_float(&mut self, name: impl Into<String>, value: f32) {
        self.properties
            .insert(name.into(), MaterialProperty::Float(value));
    }

    /// Set a color property
    pub fn set_color(&mut self, name: impl Into<String>, color: Color) {
        self.properties
            .insert(name.into(), MaterialProperty::Vec4([color.r, color.g, color.b, color.a]));
    }

    /// Set a texture property
    pub fn set_texture(&mut self, name: impl Into<String>, texture: super::texture::TextureId) {
        self.properties
            .insert(name.into(), MaterialProperty::Texture(texture));
    }

    /// Get a property
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&MaterialProperty> {
        self.properties.get(name)
    }
}

/// Built-in shaders
pub mod builtin {
    /// 2D sprite shader (WGSL)
    pub const SPRITE_2D: &str = r#"
struct CameraUniform {
    view_proj: mat4x4<f32>,
    position: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 0.0, 1.0);
    out.tex_coords = in.tex_coords;
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    return tex_color * in.color;
}
"#;

    /// 3D PBR shader (WGSL)
    pub const PBR_3D: &str = r#"
struct CameraUniform {
    view_proj: mat4x4<f32>,
    position: vec4<f32>,
}

struct ModelUniform {
    model: mat4x4<f32>,
    normal: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> model: ModelUniform;

@group(2) @binding(0)
var t_albedo: texture_2d<f32>;
@group(2) @binding(1)
var s_albedo: sampler;
@group(2) @binding(2)
var t_normal: texture_2d<f32>;
@group(2) @binding(3)
var t_metallic_roughness: texture_2d<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) tangent: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_pos = model.model * vec4<f32>(in.position, 1.0);
    out.clip_position = camera.view_proj * world_pos;
    out.world_position = world_pos.xyz;
    out.world_normal = (model.normal * vec4<f32>(in.normal, 0.0)).xyz;
    out.tex_coords = in.tex_coords;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let albedo = textureSample(t_albedo, s_albedo, in.tex_coords).rgb;
    let normal = normalize(in.world_normal);
    
    // Simple directional light
    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.3));
    let ndotl = max(dot(normal, light_dir), 0.0);
    
    let ambient = 0.1;
    let diffuse = ndotl * 0.9;
    
    let final_color = albedo * (ambient + diffuse);
    return vec4<f32>(final_color, 1.0);
}
"#;

    /// Unlit shader (WGSL)
    pub const UNLIT: &str = r#"
struct CameraUniform {
    view_proj: mat4x4<f32>,
    position: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) tangent: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 1.0);
    out.tex_coords = in.tex_coords;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
"#;
}
