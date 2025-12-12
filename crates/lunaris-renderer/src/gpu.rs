//! GPU Rendering abstraction layer
//!
//! Provides a modern, cross-platform rendering backend using wgpu.

use lunaris_core::{math::Color, Result};
use std::sync::Arc;
use wgpu::*;

/// Graphics backend configuration
#[derive(Debug, Clone)]
pub struct GraphicsConfig {
    /// Power preference for GPU selection
    pub power_preference: PowerPreference,
    /// Enable GPU validation (debug mode)
    pub validation: bool,
    /// Present mode (VSync, Mailbox, etc.)
    pub present_mode: PresentMode,
    /// Maximum frames in flight
    pub max_frames_in_flight: u32,
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        Self {
            power_preference: PowerPreference::HighPerformance,
            validation: cfg!(debug_assertions),
            present_mode: PresentMode::AutoVsync,
            max_frames_in_flight: 2,
        }
    }
}

/// The main graphics context
pub struct GraphicsContext {
    instance: Instance,
    adapter: Adapter,
    device: Arc<Device>,
    queue: Arc<Queue>,
    surface: Option<Surface<'static>>,
    surface_config: Option<SurfaceConfiguration>,
    clear_color: Color,
}

impl GraphicsContext {
    /// Create a new graphics context
    ///
    /// # Errors
    ///
    /// Returns an error if GPU initialization fails
    pub async fn new(config: &GraphicsConfig) -> Result<Self> {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Dx12Compiler::Fxc,
            flags: if config.validation {
                InstanceFlags::debugging()
            } else {
                InstanceFlags::empty()
            },
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: config.power_preference,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| lunaris_core::Error::Renderer("No suitable GPU adapter found".into()))?;

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Lunaris Device"),
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                    memory_hints: MemoryHints::Performance,
                },
                None,
            )
            .await
            .map_err(|e| lunaris_core::Error::Renderer(e.to_string()))?;

        tracing::info!(
            "GPU initialized: {} ({:?})",
            adapter.get_info().name,
            adapter.get_info().backend
        );

        Ok(Self {
            instance,
            adapter,
            device: Arc::new(device),
            queue: Arc::new(queue),
            surface: None,
            surface_config: None,
            clear_color: Color::BLACK,
        })
    }

    /// Get the device
    #[must_use]
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    /// Get the queue
    #[must_use]
    pub fn queue(&self) -> &Arc<Queue> {
        &self.queue
    }

    /// Set the clear color
    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    /// Get GPU info
    #[must_use]
    pub fn gpu_info(&self) -> GpuInfo {
        let info = self.adapter.get_info();
        GpuInfo {
            name: info.name,
            vendor: info.vendor,
            device_type: format!("{:?}", info.device_type),
            backend: format!("{:?}", info.backend),
        }
    }

    /// Begin a new frame
    pub fn begin_frame(&self) -> Option<FrameContext> {
        let surface = self.surface.as_ref()?;
        let surface_config = self.surface_config.as_ref()?;

        let output = surface.get_current_texture().ok()?;
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        Some(FrameContext {
            output,
            view,
            width: surface_config.width,
            height: surface_config.height,
        })
    }

    /// End the current frame and present
    pub fn end_frame(&self, frame: FrameContext) {
        frame.output.present();
    }

    /// Create a render pass
    pub fn create_render_pass<'a>(
        &self,
        encoder: &'a mut CommandEncoder,
        view: &'a TextureView,
    ) -> RenderPass<'a> {
        encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Main Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(wgpu::Color {
                        r: self.clear_color.r as f64,
                        g: self.clear_color.g as f64,
                        b: self.clear_color.b as f64,
                        a: self.clear_color.a as f64,
                    }),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        })
    }
}

/// GPU information
#[derive(Debug, Clone)]
pub struct GpuInfo {
    /// GPU name
    pub name: String,
    /// Vendor ID
    pub vendor: u32,
    /// Device type (Discrete, Integrated, etc.)
    pub device_type: String,
    /// Backend (Vulkan, Metal, DX12, etc.)
    pub backend: String,
}

/// Context for a single frame
pub struct FrameContext {
    output: SurfaceTexture,
    /// The texture view to render to
    pub view: TextureView,
    /// Frame width
    pub width: u32,
    /// Frame height
    pub height: u32,
}

/// Vertex format for 2D sprites
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex2D {
    /// Position (x, y)
    pub position: [f32; 2],
    /// Texture coordinates (u, v)
    pub tex_coords: [f32; 2],
    /// Color (r, g, b, a)
    pub color: [f32; 4],
}

impl Vertex2D {
    /// Vertex buffer layout
    pub fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x2,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x2,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as BufferAddress,
                    shader_location: 2,
                    format: VertexFormat::Float32x4,
                },
            ],
        }
    }
}

/// Vertex format for 3D meshes
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex3D {
    /// Position (x, y, z)
    pub position: [f32; 3],
    /// Normal (x, y, z)
    pub normal: [f32; 3],
    /// Texture coordinates (u, v)
    pub tex_coords: [f32; 2],
}

impl Vertex3D {
    /// Vertex buffer layout
    pub fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as BufferAddress,
                    shader_location: 2,
                    format: VertexFormat::Float32x2,
                },
            ],
        }
    }
}

