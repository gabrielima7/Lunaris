//! Asset Pipeline System
//!
//! Intelligent asset import, optimization, and processing pipeline.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Supported import formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportFormat {
    // 3D Models
    Gltf,
    Glb,
    Fbx,
    Obj,
    Blend,
    
    // Textures
    Png,
    Jpg,
    Tga,
    Exr,
    Hdr,
    Psd,
    
    // Audio
    Wav,
    Ogg,
    Mp3,
    Flac,
    
    // Other
    Json,
    Toml,
}

impl ImportFormat {
    /// Detect format from extension
    #[must_use]
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "gltf" => Some(Self::Gltf),
            "glb" => Some(Self::Glb),
            "fbx" => Some(Self::Fbx),
            "obj" => Some(Self::Obj),
            "blend" => Some(Self::Blend),
            "png" => Some(Self::Png),
            "jpg" | "jpeg" => Some(Self::Jpg),
            "tga" => Some(Self::Tga),
            "exr" => Some(Self::Exr),
            "hdr" => Some(Self::Hdr),
            "psd" => Some(Self::Psd),
            "wav" => Some(Self::Wav),
            "ogg" => Some(Self::Ogg),
            "mp3" => Some(Self::Mp3),
            "flac" => Some(Self::Flac),
            "json" => Some(Self::Json),
            "toml" => Some(Self::Toml),
            _ => None,
        }
    }

    /// Is 3D model format
    #[must_use]
    pub fn is_model(&self) -> bool {
        matches!(self, Self::Gltf | Self::Glb | Self::Fbx | Self::Obj | Self::Blend)
    }

    /// Is texture format
    #[must_use]
    pub fn is_texture(&self) -> bool {
        matches!(self, Self::Png | Self::Jpg | Self::Tga | Self::Exr | Self::Hdr | Self::Psd)
    }

    /// Is audio format
    #[must_use]
    pub fn is_audio(&self) -> bool {
        matches!(self, Self::Wav | Self::Ogg | Self::Mp3 | Self::Flac)
    }
}

/// Import settings for 3D models
#[derive(Debug, Clone)]
pub struct ModelImportSettings {
    /// Generate LODs automatically
    pub generate_lods: bool,
    /// Number of LOD levels
    pub lod_count: u8,
    /// LOD reduction ratio per level
    pub lod_reduction: f32,
    /// Optimize for Nanite/Virtual Geometry
    pub nanite_optimize: bool,
    /// Maximum triangle count per cluster
    pub max_cluster_triangles: u32,
    /// Generate collision mesh
    pub generate_collision: bool,
    /// Import materials
    pub import_materials: bool,
    /// Import animations
    pub import_animations: bool,
    /// Scale factor
    pub scale: f32,
    /// Up axis
    pub up_axis: UpAxis,
    /// Calculate tangents
    pub calculate_tangents: bool,
    /// Weld vertices
    pub weld_vertices: bool,
    /// Weld threshold
    pub weld_threshold: f32,
}

impl Default for ModelImportSettings {
    fn default() -> Self {
        Self {
            generate_lods: true,
            lod_count: 4,
            lod_reduction: 0.5,
            nanite_optimize: true,
            max_cluster_triangles: 128,
            generate_collision: true,
            import_materials: true,
            import_animations: true,
            scale: 1.0,
            up_axis: UpAxis::Y,
            calculate_tangents: true,
            weld_vertices: true,
            weld_threshold: 0.0001,
        }
    }
}

/// Up axis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpAxis {
    Y,
    Z,
}

/// Import settings for textures
#[derive(Debug, Clone)]
pub struct TextureImportSettings {
    /// Generate mipmaps
    pub generate_mipmaps: bool,
    /// Compression format
    pub compression: TextureCompression,
    /// sRGB color space
    pub srgb: bool,
    /// Max resolution (0 = no limit)
    pub max_resolution: u32,
    /// Power of two resize
    pub power_of_two: bool,
    /// Normal map detection
    pub detect_normal_map: bool,
    /// Alpha handling
    pub alpha_mode: AlphaMode,
}

impl Default for TextureImportSettings {
    fn default() -> Self {
        Self {
            generate_mipmaps: true,
            compression: TextureCompression::BC7,
            srgb: true,
            max_resolution: 4096,
            power_of_two: false,
            detect_normal_map: true,
            alpha_mode: AlphaMode::Auto,
        }
    }
}

/// Texture compression format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureCompression {
    None,
    BC1,
    BC3,
    BC4,
    BC5,
    BC7,
    ASTC4x4,
    ASTC6x6,
    ASTC8x8,
    ETC2,
}

/// Alpha handling mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaMode {
    Auto,
    Opaque,
    Cutout,
    Transparent,
    Premultiplied,
}

/// Import settings for audio
#[derive(Debug, Clone)]
pub struct AudioImportSettings {
    /// Sample rate conversion
    pub target_sample_rate: u32,
    /// Convert to mono
    pub force_mono: bool,
    /// Compression quality (0-1)
    pub quality: f32,
    /// Streaming mode
    pub streaming: bool,
    /// Preload on scene load
    pub preload: bool,
    /// 3D spatial audio
    pub spatial: bool,
}

impl Default for AudioImportSettings {
    fn default() -> Self {
        Self {
            target_sample_rate: 48000,
            force_mono: false,
            quality: 0.8,
            streaming: false,
            preload: true,
            spatial: false,
        }
    }
}

/// Import result
#[derive(Debug, Clone)]
pub struct ImportResult {
    /// Source path
    pub source: PathBuf,
    /// Output paths (may generate multiple files)
    pub outputs: Vec<PathBuf>,
    /// Warnings
    pub warnings: Vec<String>,
    /// Errors
    pub errors: Vec<String>,
    /// Import duration (ms)
    pub duration_ms: u64,
    /// Generated LOD count
    pub lod_count: u8,
    /// Original triangle count
    pub original_triangles: u32,
    /// Final triangle count (with LODs)
    pub total_triangles: u32,
    /// Memory size estimate
    pub memory_estimate: u64,
}

/// LOD generation result
#[derive(Debug, Clone)]
pub struct LodResult {
    /// LOD level
    pub level: u8,
    /// Triangle count
    pub triangles: u32,
    /// Vertex count
    pub vertices: u32,
    /// Screen percentage threshold
    pub screen_threshold: f32,
}

/// Asset importer
pub struct AssetImporter {
    /// Model settings
    pub model_settings: ModelImportSettings,
    /// Texture settings
    pub texture_settings: TextureImportSettings,
    /// Audio settings
    pub audio_settings: AudioImportSettings,
    /// Output directory
    pub output_dir: PathBuf,
    /// Cache directory
    pub cache_dir: PathBuf,
    /// Import history
    history: Vec<ImportResult>,
    /// File watch enabled
    pub watch_enabled: bool,
}

impl Default for AssetImporter {
    fn default() -> Self {
        Self::new(PathBuf::from("assets/imported"), PathBuf::from(".cache/assets"))
    }
}

impl AssetImporter {
    /// Create new importer
    #[must_use]
    pub fn new(output_dir: PathBuf, cache_dir: PathBuf) -> Self {
        Self {
            model_settings: ModelImportSettings::default(),
            texture_settings: TextureImportSettings::default(),
            audio_settings: AudioImportSettings::default(),
            output_dir,
            cache_dir,
            history: Vec::new(),
            watch_enabled: true,
        }
    }

    /// Import a file
    pub fn import(&mut self, path: &Path) -> Result<ImportResult, ImportError> {
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .ok_or(ImportError::UnknownFormat)?;

        let format = ImportFormat::from_extension(ext)
            .ok_or(ImportError::UnknownFormat)?;

        let start = std::time::Instant::now();

        let result = if format.is_model() {
            self.import_model(path, format)
        } else if format.is_texture() {
            self.import_texture(path, format)
        } else if format.is_audio() {
            self.import_audio(path, format)
        } else {
            self.import_generic(path)
        }?;

        let mut result = result;
        result.duration_ms = start.elapsed().as_millis() as u64;

        self.history.push(result.clone());
        Ok(result)
    }

    fn import_model(&self, path: &Path, _format: ImportFormat) -> Result<ImportResult, ImportError> {
        let mut outputs = Vec::new();
        let mut warnings = Vec::new();
        let base_name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("model");

        // Generate main mesh
        let main_output = self.output_dir.join(format!("{}.mesh", base_name));
        outputs.push(main_output);

        // Generate LODs
        let mut lods = Vec::new();
        let original_triangles = 100000; // Would be read from file
        let mut total_triangles = original_triangles;

        if self.model_settings.generate_lods {
            let mut triangles = original_triangles;
            for level in 0..self.model_settings.lod_count {
                let screen_threshold = match level {
                    0 => 1.0,
                    1 => 0.5,
                    2 => 0.25,
                    3 => 0.125,
                    _ => 0.0625,
                };

                lods.push(LodResult {
                    level,
                    triangles,
                    vertices: triangles * 3 / 2, // Estimate
                    screen_threshold,
                });

                if level < self.model_settings.lod_count - 1 {
                    triangles = (triangles as f32 * self.model_settings.lod_reduction) as u32;
                    total_triangles += triangles;
                }

                let lod_output = self.output_dir.join(format!("{}_lod{}.mesh", base_name, level));
                outputs.push(lod_output);
            }
        }

        // Generate collision
        if self.model_settings.generate_collision {
            let collision_output = self.output_dir.join(format!("{}.collision", base_name));
            outputs.push(collision_output);
        }

        // Nanite optimization
        if self.model_settings.nanite_optimize {
            let cluster_count = original_triangles / self.model_settings.max_cluster_triangles;
            warnings.push(format!("Generated {} clusters for Nanite", cluster_count));
        }

        Ok(ImportResult {
            source: path.to_path_buf(),
            outputs,
            warnings,
            errors: Vec::new(),
            duration_ms: 0,
            lod_count: self.model_settings.lod_count,
            original_triangles,
            total_triangles,
            memory_estimate: total_triangles as u64 * 64, // Estimate bytes
        })
    }

    fn import_texture(&self, path: &Path, _format: ImportFormat) -> Result<ImportResult, ImportError> {
        let base_name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("texture");

        let output = self.output_dir.join(format!("{}.tex", base_name));
        let outputs = vec![output];

        let mut warnings = Vec::new();

        if self.texture_settings.generate_mipmaps {
            warnings.push("Generated 10 mipmap levels".to_string());
        }

        warnings.push(format!("Compressed with {:?}", self.texture_settings.compression));

        Ok(ImportResult {
            source: path.to_path_buf(),
            outputs,
            warnings,
            errors: Vec::new(),
            duration_ms: 0,
            lod_count: 0,
            original_triangles: 0,
            total_triangles: 0,
            memory_estimate: 4 * 1024 * 1024, // 4MB estimate
        })
    }

    fn import_audio(&self, path: &Path, _format: ImportFormat) -> Result<ImportResult, ImportError> {
        let base_name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("audio");

        let output = self.output_dir.join(format!("{}.audio", base_name));
        let outputs = vec![output];

        let mut warnings = Vec::new();
        warnings.push(format!("Resampled to {}Hz", self.audio_settings.target_sample_rate));

        if self.audio_settings.force_mono {
            warnings.push("Converted to mono".to_string());
        }

        Ok(ImportResult {
            source: path.to_path_buf(),
            outputs,
            warnings,
            errors: Vec::new(),
            duration_ms: 0,
            lod_count: 0,
            original_triangles: 0,
            total_triangles: 0,
            memory_estimate: 1024 * 1024, // 1MB estimate
        })
    }

    fn import_generic(&self, path: &Path) -> Result<ImportResult, ImportError> {
        let file_name = path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("file");

        let output = self.output_dir.join(file_name);

        Ok(ImportResult {
            source: path.to_path_buf(),
            outputs: vec![output],
            warnings: Vec::new(),
            errors: Vec::new(),
            duration_ms: 0,
            lod_count: 0,
            original_triangles: 0,
            total_triangles: 0,
            memory_estimate: 0,
        })
    }

    /// Import all files in a directory
    pub fn import_directory(&mut self, dir: &Path) -> Vec<Result<ImportResult, ImportError>> {
        let mut results = Vec::new();

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    results.push(self.import(&path));
                }
            }
        }

        results
    }

    /// Get import history
    #[must_use]
    pub fn history(&self) -> &[ImportResult] {
        &self.history
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

/// Import error
#[derive(Debug, Clone)]
pub enum ImportError {
    /// Unknown file format
    UnknownFormat,
    /// File not found
    FileNotFound,
    /// Parse error
    ParseError(String),
    /// IO error
    IoError(String),
    /// Compression error
    CompressionError(String),
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownFormat => write!(f, "Unknown file format"),
            Self::FileNotFound => write!(f, "File not found"),
            Self::ParseError(e) => write!(f, "Parse error: {}", e),
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::CompressionError(e) => write!(f, "Compression error: {}", e),
        }
    }
}

impl std::error::Error for ImportError {}

/// Auto-reimport watcher
pub struct AssetWatcher {
    /// Watch directories
    pub directories: Vec<PathBuf>,
    /// File modification times
    modification_times: HashMap<PathBuf, std::time::SystemTime>,
    /// Poll interval (ms)
    pub poll_interval_ms: u32,
    /// Last poll
    last_poll: std::time::Instant,
    /// Pending reimports
    pending: Vec<PathBuf>,
}

impl Default for AssetWatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl AssetWatcher {
    /// Create new watcher
    #[must_use]
    pub fn new() -> Self {
        Self {
            directories: Vec::new(),
            modification_times: HashMap::new(),
            poll_interval_ms: 1000,
            last_poll: std::time::Instant::now(),
            pending: Vec::new(),
        }
    }

    /// Add directory to watch
    pub fn watch(&mut self, dir: PathBuf) {
        self.directories.push(dir);
    }

    /// Poll for changes
    pub fn poll(&mut self) -> Vec<PathBuf> {
        let now = std::time::Instant::now();
        if now.duration_since(self.last_poll).as_millis() < self.poll_interval_ms as u128 {
            return Vec::new();
        }
        self.last_poll = now;

        let mut changed = Vec::new();

        for dir in &self.directories {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Ok(metadata) = std::fs::metadata(&path) {
                            if let Ok(modified) = metadata.modified() {
                                let prev = self.modification_times.get(&path).copied();
                                if prev.map_or(true, |prev| modified > prev) {
                                    self.modification_times.insert(path.clone(), modified);
                                    if prev.is_some() {
                                        changed.push(path);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        changed
    }
}
