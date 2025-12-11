//! Render Graph System
//!
//! Frame graph for optimized render pass scheduling and resource management.

use std::collections::HashMap;

/// Resource handle
pub type ResourceHandle = u64;

/// Pass handle
pub type PassHandle = u64;

/// Texture usage flags
#[derive(Debug, Clone, Copy)]
pub struct TextureUsage {
    pub render_target: bool,
    pub shader_read: bool,
    pub shader_write: bool,
    pub copy_src: bool,
    pub copy_dst: bool,
}

impl Default for TextureUsage {
    fn default() -> Self {
        Self {
            render_target: false,
            shader_read: true,
            shader_write: false,
            copy_src: false,
            copy_dst: false,
        }
    }
}

/// Texture description
#[derive(Debug, Clone)]
pub struct TextureDesc {
    /// Width
    pub width: u32,
    /// Height
    pub height: u32,
    /// Depth
    pub depth: u32,
    /// Format
    pub format: TextureFormat,
    /// Mip levels
    pub mip_levels: u32,
    /// Sample count
    pub samples: u32,
    /// Usage
    pub usage: TextureUsage,
    /// Name (for debugging)
    pub name: String,
}

/// Texture format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    R8Unorm,
    R8Snorm,
    R16Float,
    R32Float,
    Rg8Unorm,
    Rg16Float,
    Rg32Float,
    Rgba8Unorm,
    Rgba8Srgb,
    Rgba16Float,
    Rgba32Float,
    Bgra8Unorm,
    Bgra8Srgb,
    Rgb10A2Unorm,
    Rg11B10Float,
    Depth16,
    Depth24,
    Depth32Float,
    Depth24Stencil8,
    Depth32FloatStencil8,
}

/// Buffer description
#[derive(Debug, Clone)]
pub struct BufferDesc {
    /// Size in bytes
    pub size: u64,
    /// Usage
    pub usage: BufferUsage,
    /// Name
    pub name: String,
}

/// Buffer usage flags
#[derive(Debug, Clone, Copy, Default)]
pub struct BufferUsage {
    pub vertex: bool,
    pub index: bool,
    pub uniform: bool,
    pub storage: bool,
    pub indirect: bool,
    pub copy_src: bool,
    pub copy_dst: bool,
}

/// Resource type
#[derive(Debug, Clone)]
pub enum ResourceType {
    Texture(TextureDesc),
    Buffer(BufferDesc),
    Imported,
}

/// Resource in graph
#[derive(Debug, Clone)]
pub struct GraphResource {
    /// Handle
    pub handle: ResourceHandle,
    /// Type
    pub resource_type: ResourceType,
    /// Is imported (external)
    pub imported: bool,
    /// First use pass
    pub first_use: Option<PassHandle>,
    /// Last use pass
    pub last_use: Option<PassHandle>,
    /// Physical resource index
    pub physical_index: Option<usize>,
}

/// Pass type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassType {
    Render,
    Compute,
    Copy,
    Present,
}

/// Render pass
#[derive(Debug, Clone)]
pub struct RenderPass {
    /// Handle
    pub handle: PassHandle,
    /// Name
    pub name: String,
    /// Type
    pub pass_type: PassType,
    /// Read resources
    pub reads: Vec<ResourceHandle>,
    /// Write resources
    pub writes: Vec<ResourceHandle>,
    /// Color attachments
    pub color_attachments: Vec<ResourceHandle>,
    /// Depth attachment
    pub depth_attachment: Option<ResourceHandle>,
    /// Is enabled
    pub enabled: bool,
    /// Order (after compilation)
    pub order: usize,
}

/// Render graph
pub struct RenderGraph {
    /// Resources
    resources: HashMap<ResourceHandle, GraphResource>,
    /// Passes
    passes: HashMap<PassHandle, RenderPass>,
    /// Next handle
    next_handle: u64,
    /// Compiled order
    compiled_order: Vec<PassHandle>,
    /// Is compiled
    compiled: bool,
}

impl Default for RenderGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderGraph {
    /// Create new render graph
    #[must_use]
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            passes: HashMap::new(),
            next_handle: 1,
            compiled_order: Vec::new(),
            compiled: false,
        }
    }

    /// Create transient texture
    pub fn create_texture(&mut self, desc: TextureDesc) -> ResourceHandle {
        let handle = self.next_handle;
        self.next_handle += 1;

        let resource = GraphResource {
            handle,
            resource_type: ResourceType::Texture(desc),
            imported: false,
            first_use: None,
            last_use: None,
            physical_index: None,
        };

        self.resources.insert(handle, resource);
        self.compiled = false;
        handle
    }

    /// Create transient buffer
    pub fn create_buffer(&mut self, desc: BufferDesc) -> ResourceHandle {
        let handle = self.next_handle;
        self.next_handle += 1;

        let resource = GraphResource {
            handle,
            resource_type: ResourceType::Buffer(desc),
            imported: false,
            first_use: None,
            last_use: None,
            physical_index: None,
        };

        self.resources.insert(handle, resource);
        self.compiled = false;
        handle
    }

    /// Import external resource
    pub fn import(&mut self, name: &str) -> ResourceHandle {
        let handle = self.next_handle;
        self.next_handle += 1;

        let resource = GraphResource {
            handle,
            resource_type: ResourceType::Imported,
            imported: true,
            first_use: None,
            last_use: None,
            physical_index: None,
        };

        self.resources.insert(handle, resource);
        handle
    }

    /// Add render pass
    pub fn add_render_pass(&mut self, name: &str) -> PassHandle {
        let handle = self.next_handle;
        self.next_handle += 1;

        let pass = RenderPass {
            handle,
            name: name.to_string(),
            pass_type: PassType::Render,
            reads: Vec::new(),
            writes: Vec::new(),
            color_attachments: Vec::new(),
            depth_attachment: None,
            enabled: true,
            order: 0,
        };

        self.passes.insert(handle, pass);
        self.compiled = false;
        handle
    }

    /// Add compute pass
    pub fn add_compute_pass(&mut self, name: &str) -> PassHandle {
        let handle = self.next_handle;
        self.next_handle += 1;

        let pass = RenderPass {
            handle,
            name: name.to_string(),
            pass_type: PassType::Compute,
            reads: Vec::new(),
            writes: Vec::new(),
            color_attachments: Vec::new(),
            depth_attachment: None,
            enabled: true,
            order: 0,
        };

        self.passes.insert(handle, pass);
        self.compiled = false;
        handle
    }

    /// Pass reads resource
    pub fn pass_read(&mut self, pass: PassHandle, resource: ResourceHandle) {
        if let Some(p) = self.passes.get_mut(&pass) {
            p.reads.push(resource);
        }
        if let Some(r) = self.resources.get_mut(&resource) {
            r.last_use = Some(pass);
            if r.first_use.is_none() {
                r.first_use = Some(pass);
            }
        }
        self.compiled = false;
    }

    /// Pass writes resource
    pub fn pass_write(&mut self, pass: PassHandle, resource: ResourceHandle) {
        if let Some(p) = self.passes.get_mut(&pass) {
            p.writes.push(resource);
        }
        if let Some(r) = self.resources.get_mut(&resource) {
            if r.first_use.is_none() {
                r.first_use = Some(pass);
            }
            r.last_use = Some(pass);
        }
        self.compiled = false;
    }

    /// Set color attachment
    pub fn set_color_attachment(&mut self, pass: PassHandle, slot: usize, resource: ResourceHandle) {
        if let Some(p) = self.passes.get_mut(&pass) {
            while p.color_attachments.len() <= slot {
                p.color_attachments.push(0);
            }
            p.color_attachments[slot] = resource;
        }
        self.pass_write(pass, resource);
    }

    /// Set depth attachment
    pub fn set_depth_attachment(&mut self, pass: PassHandle, resource: ResourceHandle) {
        if let Some(p) = self.passes.get_mut(&pass) {
            p.depth_attachment = Some(resource);
        }
        self.pass_write(pass, resource);
    }

    /// Compile graph
    pub fn compile(&mut self) {
        if self.compiled {
            return;
        }

        // Topological sort passes
        let mut order = Vec::new();
        let mut visited = std::collections::HashSet::new();

        for &handle in self.passes.keys() {
            self.visit_pass(handle, &mut visited, &mut order);
        }

        // Assign order
        for (i, &handle) in order.iter().enumerate() {
            if let Some(pass) = self.passes.get_mut(&handle) {
                pass.order = i;
            }
        }

        self.compiled_order = order;
        self.compiled = true;

        // Allocate physical resources (lifetime analysis)
        self.allocate_resources();
    }

    fn visit_pass(
        &self,
        handle: PassHandle,
        visited: &mut std::collections::HashSet<PassHandle>,
        order: &mut Vec<PassHandle>,
    ) {
        if visited.contains(&handle) {
            return;
        }
        visited.insert(handle);

        if let Some(pass) = self.passes.get(&handle) {
            // Visit dependencies (passes that write to our reads)
            for &read in &pass.reads {
                if let Some(resource) = self.resources.get(&read) {
                    if let Some(writer) = self.find_writer(read, handle) {
                        self.visit_pass(writer, visited, order);
                    }
                }
            }
        }

        order.push(handle);
    }

    fn find_writer(&self, resource: ResourceHandle, before: PassHandle) -> Option<PassHandle> {
        for (handle, pass) in &self.passes {
            if *handle != before && pass.writes.contains(&resource) {
                return Some(*handle);
            }
        }
        None
    }

    fn allocate_resources(&mut self) {
        // Simple allocation - would use more sophisticated aliasing
        let mut physical_count = 0usize;

        for resource in self.resources.values_mut() {
            if !resource.imported {
                resource.physical_index = Some(physical_count);
                physical_count += 1;
            }
        }
    }

    /// Get execution order
    #[must_use]
    pub fn execution_order(&self) -> &[PassHandle] {
        &self.compiled_order
    }

    /// Get pass  
    #[must_use]
    pub fn get_pass(&self, handle: PassHandle) -> Option<&RenderPass> {
        self.passes.get(&handle)
    }

    /// Get statistics
    #[must_use]
    pub fn stats(&self) -> RenderGraphStats {
        let transient = self.resources.values().filter(|r| !r.imported).count();
        
        RenderGraphStats {
            pass_count: self.passes.len(),
            resource_count: self.resources.len(),
            transient_resources: transient,
            imported_resources: self.resources.len() - transient,
        }
    }
}

/// Render graph statistics
#[derive(Debug, Clone)]
pub struct RenderGraphStats {
    pub pass_count: usize,
    pub resource_count: usize,
    pub transient_resources: usize,
    pub imported_resources: usize,
}

/// Standard render graph builder
pub struct StandardRenderGraph;

impl StandardRenderGraph {
    /// Build deferred rendering graph
    #[must_use]
    pub fn deferred() -> RenderGraph {
        let mut graph = RenderGraph::new();

        // GBuffer pass
        let gbuffer_albedo = graph.create_texture(TextureDesc {
            width: 1920,
            height: 1080,
            depth: 1,
            format: TextureFormat::Rgba8Srgb,
            mip_levels: 1,
            samples: 1,
            usage: TextureUsage { render_target: true, shader_read: true, ..Default::default() },
            name: "GBuffer.Albedo".into(),
        });

        let gbuffer_normal = graph.create_texture(TextureDesc {
            width: 1920,
            height: 1080,
            depth: 1,
            format: TextureFormat::Rgba16Float,
            mip_levels: 1,
            samples: 1,
            usage: TextureUsage { render_target: true, shader_read: true, ..Default::default() },
            name: "GBuffer.Normal".into(),
        });

        let depth = graph.create_texture(TextureDesc {
            width: 1920,
            height: 1080,
            depth: 1,
            format: TextureFormat::Depth32Float,
            mip_levels: 1,
            samples: 1,
            usage: TextureUsage { render_target: true, shader_read: true, ..Default::default() },
            name: "Depth".into(),
        });

        let hdr = graph.create_texture(TextureDesc {
            width: 1920,
            height: 1080,
            depth: 1,
            format: TextureFormat::Rgba16Float,
            mip_levels: 1,
            samples: 1,
            usage: TextureUsage { render_target: true, shader_read: true, ..Default::default() },
            name: "HDR".into(),
        });

        // GBuffer pass
        let gbuffer_pass = graph.add_render_pass("GBuffer");
        graph.set_color_attachment(gbuffer_pass, 0, gbuffer_albedo);
        graph.set_color_attachment(gbuffer_pass, 1, gbuffer_normal);
        graph.set_depth_attachment(gbuffer_pass, depth);

        // Lighting pass
        let lighting_pass = graph.add_render_pass("Lighting");
        graph.pass_read(lighting_pass, gbuffer_albedo);
        graph.pass_read(lighting_pass, gbuffer_normal);
        graph.pass_read(lighting_pass, depth);
        graph.set_color_attachment(lighting_pass, 0, hdr);

        // Post-process pass
        let post_pass = graph.add_render_pass("PostProcess");
        graph.pass_read(post_pass, hdr);

        graph.compile();
        graph
    }
}
