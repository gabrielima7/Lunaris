//! 2D Render Pipeline
//!
//! Handles sprite batching and efficient 2D rendering.

use super::{Camera2D, CameraUniform, Vertex2D};
use crate::texture::TextureId;
use lunaris_core::math::{Color, Rect, Vec2};
use std::collections::HashMap;
use wgpu::*;
use wgpu::util::DeviceExt;

/// Maximum sprites per batch
pub const MAX_SPRITES_PER_BATCH: usize = 10000;

/// Sprite instance for batching
#[derive(Debug, Clone, Copy)]
pub struct SpriteInstance {
    /// Position in world space
    pub position: Vec2,
    /// Size in pixels
    pub size: Vec2,
    /// Rotation in radians
    pub rotation: f32,
    /// UV coordinates
    pub uv: Rect,
    /// Tint color
    pub color: Color,
    /// Z-order (higher = in front)
    pub z_order: f32,
    /// Texture ID
    pub texture: TextureId,
}

impl SpriteInstance {
    /// Create a simple sprite
    #[must_use]
    pub fn new(position: Vec2, size: Vec2, texture: TextureId) -> Self {
        Self {
            position,
            size,
            rotation: 0.0,
            uv: Rect::new(0.0, 0.0, 1.0, 1.0),
            color: Color::WHITE,
            z_order: 0.0,
            texture,
        }
    }

    /// Set rotation
    #[must_use]
    pub const fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    /// Set color tint
    #[must_use]
    pub const fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set UV coordinates
    #[must_use]
    pub const fn with_uv(mut self, uv: Rect) -> Self {
        self.uv = uv;
        self
    }
}

/// Sprite batch for efficient rendering
pub struct SpriteBatch {
    /// Sprites to render
    sprites: Vec<SpriteInstance>,
    /// Vertex buffer
    vertex_buffer: Option<Buffer>,
    /// Index buffer
    index_buffer: Option<Buffer>,
    /// Vertices
    vertices: Vec<Vertex2D>,
    /// Indices
    indices: Vec<u16>,
    /// Is dirty (needs rebuild)
    dirty: bool,
}

impl Default for SpriteBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl SpriteBatch {
    /// Create a new sprite batch
    #[must_use]
    pub fn new() -> Self {
        Self {
            sprites: Vec::with_capacity(MAX_SPRITES_PER_BATCH),
            vertex_buffer: None,
            index_buffer: None,
            vertices: Vec::with_capacity(MAX_SPRITES_PER_BATCH * 4),
            indices: Vec::with_capacity(MAX_SPRITES_PER_BATCH * 6),
            dirty: false,
        }
    }

    /// Clear all sprites
    pub fn clear(&mut self) {
        self.sprites.clear();
        self.dirty = true;
    }

    /// Add a sprite
    pub fn add(&mut self, sprite: SpriteInstance) {
        self.sprites.push(sprite);
        self.dirty = true;
    }

    /// Add many sprites
    pub fn add_many(&mut self, sprites: impl IntoIterator<Item = SpriteInstance>) {
        self.sprites.extend(sprites);
        self.dirty = true;
    }

    /// Sort sprites by z-order and texture (for batching)
    pub fn sort(&mut self) {
        self.sprites.sort_by(|a, b| {
            a.z_order
                .partial_cmp(&b.z_order)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.texture.0.raw().cmp(&b.texture.0.raw()))
        });
    }

    /// Build vertex and index buffers
    pub fn build(&mut self, device: &Device) {
        if !self.dirty {
            return;
        }

        self.vertices.clear();
        self.indices.clear();

        for (i, sprite) in self.sprites.iter().enumerate() {
            let base_index = (i * 4) as u16;

            // Calculate rotated corners
            let half_size = sprite.size * 0.5;
            let cos = sprite.rotation.cos();
            let sin = sprite.rotation.sin();

            let corners = [
                Vec2::new(-half_size.x, -half_size.y), // top-left
                Vec2::new(half_size.x, -half_size.y),  // top-right
                Vec2::new(half_size.x, half_size.y),   // bottom-right
                Vec2::new(-half_size.x, half_size.y),  // bottom-left
            ];

            let uvs = [
                [sprite.uv.x, sprite.uv.y],
                [sprite.uv.x + sprite.uv.width, sprite.uv.y],
                [sprite.uv.x + sprite.uv.width, sprite.uv.y + sprite.uv.height],
                [sprite.uv.x, sprite.uv.y + sprite.uv.height],
            ];

            for (j, corner) in corners.iter().enumerate() {
                let rotated = Vec2::new(
                    corner.x * cos - corner.y * sin,
                    corner.x * sin + corner.y * cos,
                );
                let position = sprite.position + rotated;

                self.vertices.push(Vertex2D {
                    position: [position.x, position.y],
                    tex_coords: uvs[j],
                    color: [sprite.color.r, sprite.color.g, sprite.color.b, sprite.color.a],
                });
            }

            // Two triangles per sprite
            self.indices.extend_from_slice(&[
                base_index,
                base_index + 1,
                base_index + 2,
                base_index,
                base_index + 2,
                base_index + 3,
            ]);
        }

        // Create buffers
        self.vertex_buffer = Some(device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Sprite Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: BufferUsages::VERTEX,
        }));

        self.index_buffer = Some(device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Sprite Index Buffer"),
            contents: bytemuck::cast_slice(&self.indices),
            usage: BufferUsages::INDEX,
        }));

        self.dirty = false;
    }

    /// Get number of sprites
    #[must_use]
    pub fn len(&self) -> usize {
        self.sprites.len()
    }

    /// Check if empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sprites.is_empty()
    }
}

/// 2D Render pipeline
pub struct Render2D {
    /// Sprite batches per texture
    batches: HashMap<u64, SpriteBatch>,
    /// Default batch for untextured sprites
    default_batch: SpriteBatch,
    /// Camera uniform buffer
    camera_buffer: Option<Buffer>,
    /// Render pipeline
    pipeline: Option<RenderPipeline>,
    /// Current camera
    camera: Camera2D,
}

impl Default for Render2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Render2D {
    /// Create a new 2D renderer
    #[must_use]
    pub fn new() -> Self {
        Self {
            batches: HashMap::new(),
            default_batch: SpriteBatch::new(),
            camera_buffer: None,
            pipeline: None,
            camera: Camera2D::default(),
        }
    }

    /// Initialize the renderer with a device
    pub fn init(&mut self, device: &Device, format: TextureFormat) {
        // Create shader module
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("2D Shader"),
            source: ShaderSource::Wgsl(super::material::builtin::SPRITE_2D.into()),
        });

        // Camera uniform buffer
        let camera_uniform = CameraUniform::from_camera_2d(&self.camera);
        self.camera_buffer = Some(device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        }));

        // Bind group layout for camera
        let camera_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("2D Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        self.pipeline = Some(device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("2D Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex2D::layout()],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        }));

        tracing::info!("2D Renderer initialized");
    }

    /// Set the camera
    pub fn set_camera(&mut self, camera: Camera2D) {
        self.camera = camera;
    }

    /// Get the camera
    #[must_use]
    pub fn camera(&self) -> &Camera2D {
        &self.camera
    }

    /// Begin a new frame
    pub fn begin_frame(&mut self) {
        self.default_batch.clear();
        for batch in self.batches.values_mut() {
            batch.clear();
        }
    }

    /// Draw a sprite
    pub fn draw_sprite(&mut self, sprite: SpriteInstance) {
        let texture_id = sprite.texture.0.raw();
        let batch = self.batches.entry(texture_id).or_insert_with(SpriteBatch::new);
        batch.add(sprite);
    }

    /// Draw a rectangle
    pub fn draw_rect(&mut self, position: Vec2, size: Vec2, color: Color) {
        self.default_batch.add(SpriteInstance {
            position,
            size,
            rotation: 0.0,
            uv: Rect::new(0.0, 0.0, 1.0, 1.0),
            color,
            z_order: 0.0,
            texture: TextureId(lunaris_core::id::Id::NULL),
        });
    }

    /// End frame and submit draw commands
    pub fn end_frame(&mut self, device: &Device, queue: &Queue) {
        // Update camera buffer
        if let Some(buffer) = &self.camera_buffer {
            let camera_uniform = CameraUniform::from_camera_2d(&self.camera);
            queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
        }

        // Build all batches
        self.default_batch.build(device);
        for batch in self.batches.values_mut() {
            batch.sort();
            batch.build(device);
        }
    }

    /// Get draw statistics
    #[must_use]
    pub fn stats(&self) -> RenderStats {
        let mut total_sprites = self.default_batch.len();
        let mut batch_count = if self.default_batch.is_empty() { 0 } else { 1 };
        
        for batch in self.batches.values() {
            total_sprites += batch.len();
            if !batch.is_empty() {
                batch_count += 1;
            }
        }

        RenderStats {
            sprites: total_sprites,
            batches: batch_count,
            draw_calls: batch_count,
        }
    }
}

/// Render statistics
#[derive(Debug, Clone, Copy, Default)]
pub struct RenderStats {
    /// Total sprites rendered
    pub sprites: usize,
    /// Number of batches
    pub batches: usize,
    /// Number of draw calls
    pub draw_calls: usize,
}
