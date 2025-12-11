//! Virtual Texturing
//!
//! Streaming of infinite textures, UDIM support, megatextures.

use glam::{Vec2, IVec2};
use std::collections::{HashMap, HashSet};

/// Virtual texture system
pub struct VirtualTexturing {
    pub atlases: Vec<TextureAtlas>,
    pub page_table: PageTable,
    pub feedback: FeedbackBuffer,
    pub cache: PageCache,
    pub settings: VTSettings,
}

/// Texture atlas
pub struct TextureAtlas {
    pub size: u32,
    pub page_size: u32,
    pub pages: Vec<AtlasPage>,
    pub free_pages: Vec<usize>,
}

/// Atlas page
pub struct AtlasPage {
    pub virtual_id: Option<(u64, u32, IVec2)>, // texture_id, mip, tile_coord
    pub last_used: u32,
}

/// Page table
pub struct PageTable {
    pub entries: HashMap<(u64, u32, IVec2), PageTableEntry>,
    pub texture_infos: HashMap<u64, VirtualTextureInfo>,
}

/// Page table entry
pub struct PageTableEntry {
    pub atlas_index: usize,
    pub page_index: usize,
    pub loaded: bool,
}

/// Virtual texture info
pub struct VirtualTextureInfo {
    pub id: u64,
    pub width: u32,
    pub height: u32,
    pub mip_levels: u32,
    pub page_size: u32,
    pub path: String,
}

/// Feedback buffer
pub struct FeedbackBuffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<FeedbackEntry>,
    pub requested_pages: HashSet<(u64, u32, IVec2)>,
}

/// Feedback entry
#[derive(Clone, Copy, Default)]
pub struct FeedbackEntry {
    pub texture_id: u64,
    pub mip_level: u32,
    pub tile_x: u16,
    pub tile_y: u16,
}

/// Page cache
pub struct PageCache {
    pub max_pages: usize,
    pub loaded_pages: Vec<CachedPage>,
    pub pending_loads: Vec<PageLoadRequest>,
    pub loads_per_frame: u32,
}

/// Cached page
pub struct CachedPage {
    pub texture_id: u64,
    pub mip: u32,
    pub coord: IVec2,
    pub data: Vec<u8>,
}

/// Page load request
pub struct PageLoadRequest {
    pub texture_id: u64,
    pub mip: u32,
    pub coord: IVec2,
    pub priority: f32,
}

/// VT settings
pub struct VTSettings {
    pub page_size: u32,
    pub atlas_size: u32,
    pub max_anisotropy: u32,
    pub mip_bias: f32,
    pub feedback_scale: u32,
    pub max_loads_per_frame: u32,
}

impl Default for VTSettings {
    fn default() -> Self {
        Self { page_size: 128, atlas_size: 4096, max_anisotropy: 16, mip_bias: -0.5, feedback_scale: 16, max_loads_per_frame: 4 }
    }
}

impl VirtualTexturing {
    pub fn new() -> Self {
        Self {
            atlases: vec![TextureAtlas::new(4096, 128)],
            page_table: PageTable { entries: HashMap::new(), texture_infos: HashMap::new() },
            feedback: FeedbackBuffer::new(120, 68),
            cache: PageCache { max_pages: 1024, loaded_pages: Vec::new(), pending_loads: Vec::new(), loads_per_frame: 4 },
            settings: VTSettings::default(),
        }
    }

    pub fn register_texture(&mut self, id: u64, width: u32, height: u32, path: &str) {
        let mips = (width.max(height) as f32).log2().ceil() as u32;
        self.page_table.texture_infos.insert(id, VirtualTextureInfo { id, width, height, mip_levels: mips, page_size: self.settings.page_size, path: path.into() });
    }

    pub fn update(&mut self, frame: u32) {
        // Process feedback
        self.feedback.requested_pages.clear();
        for entry in &self.feedback.data {
            if entry.texture_id != 0 {
                self.feedback.requested_pages.insert((entry.texture_id, entry.mip_level, IVec2::new(entry.tile_x as i32, entry.tile_y as i32)));
            }
        }

        // Queue missing pages for loading
        for (tex_id, mip, coord) in &self.feedback.requested_pages {
            let key = (*tex_id, *mip, *coord);
            if !self.page_table.entries.contains_key(&key) {
                let camera_dist = (coord.as_vec2().length()) / 100.0;
                let priority = 1.0 / (1.0 + camera_dist + *mip as f32);
                self.cache.pending_loads.push(PageLoadRequest { texture_id: *tex_id, mip: *mip, coord: *coord, priority });
            }
        }

        // Sort by priority
        self.cache.pending_loads.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());

        // Process loads
        for _ in 0..self.settings.max_loads_per_frame {
            if let Some(request) = self.cache.pending_loads.pop() {
                self.load_page(request, frame);
            }
        }

        // Evict old pages
        self.evict_pages(frame);
    }

    fn load_page(&mut self, request: PageLoadRequest, frame: u32) {
        // Find free page in atlas
        let atlas = &mut self.atlases[0];
        if let Some(page_idx) = atlas.free_pages.pop() {
            atlas.pages[page_idx] = AtlasPage { virtual_id: Some((request.texture_id, request.mip, request.coord)), last_used: frame };
            self.page_table.entries.insert((request.texture_id, request.mip, request.coord), PageTableEntry { atlas_index: 0, page_index: page_idx, loaded: true });
        }
    }

    fn evict_pages(&mut self, frame: u32) {
        let threshold = frame.saturating_sub(60);
        for atlas in &mut self.atlases {
            for (idx, page) in atlas.pages.iter_mut().enumerate() {
                if page.virtual_id.is_some() && page.last_used < threshold {
                    if let Some(key) = page.virtual_id {
                        self.page_table.entries.remove(&key);
                    }
                    page.virtual_id = None;
                    atlas.free_pages.push(idx);
                }
            }
        }
    }

    pub fn sample(&self, texture_id: u64, uv: Vec2, mip: u32) -> Option<(usize, usize, Vec2)> {
        let info = self.page_table.texture_infos.get(&texture_id)?;
        let scale = 1.0 / (1 << mip) as f32;
        let pages_x = (info.width as f32 * scale / self.settings.page_size as f32).ceil() as i32;
        let pages_y = (info.height as f32 * scale / self.settings.page_size as f32).ceil() as i32;
        
        let tile_x = (uv.x * pages_x as f32).floor() as i32;
        let tile_y = (uv.y * pages_y as f32).floor() as i32;
        let coord = IVec2::new(tile_x.clamp(0, pages_x - 1), tile_y.clamp(0, pages_y - 1));
        
        let entry = self.page_table.entries.get(&(texture_id, mip, coord))?;
        let local_uv = Vec2::new(uv.x * pages_x as f32 - tile_x as f32, uv.y * pages_y as f32 - tile_y as f32);
        
        Some((entry.atlas_index, entry.page_index, local_uv))
    }
}

impl TextureAtlas {
    pub fn new(size: u32, page_size: u32) -> Self {
        let pages_per_side = size / page_size;
        let total_pages = (pages_per_side * pages_per_side) as usize;
        let pages = vec![AtlasPage { virtual_id: None, last_used: 0 }; total_pages];
        let free_pages = (0..total_pages).collect();
        Self { size, page_size, pages, free_pages }
    }
}

impl FeedbackBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, data: vec![FeedbackEntry::default(); (width * height) as usize], requested_pages: HashSet::new() }
    }
}
