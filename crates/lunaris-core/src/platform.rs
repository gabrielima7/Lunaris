//! Platform Abstraction Layer
//!
//! Cross-platform support for all major platforms and devices.

use std::collections::HashMap;

/// Target platform
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    // Desktop
    Windows,
    Linux,
    MacOS,
    
    // Mobile
    Ios,
    Android,
    
    // Console
    PlayStation5,
    XboxSeriesX,
    NintendoSwitch,
    
    // Web
    WebAssembly,
    WebGPU,
    
    // VR/AR
    MetaQuest,
    AppleVisionPro,
    SteamVR,
    PSVR2,
    
    // Cloud Gaming
    GeForceNow,
    XCloud,
    PlayStationNow,
    
    // Embedded
    RaspberryPi,
    SteamDeck,
}

impl Platform {
    /// Get all supported platforms
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
            Self::Windows, Self::Linux, Self::MacOS,
            Self::Ios, Self::Android,
            Self::PlayStation5, Self::XboxSeriesX, Self::NintendoSwitch,
            Self::WebAssembly, Self::WebGPU,
            Self::MetaQuest, Self::AppleVisionPro, Self::SteamVR, Self::PSVR2,
            Self::GeForceNow, Self::XCloud, Self::PlayStationNow,
            Self::RaspberryPi, Self::SteamDeck,
        ]
    }

    /// Is desktop platform
    #[must_use]
    pub fn is_desktop(&self) -> bool {
        matches!(self, Self::Windows | Self::Linux | Self::MacOS)
    }

    /// Is mobile platform
    #[must_use]
    pub fn is_mobile(&self) -> bool {
        matches!(self, Self::Ios | Self::Android)
    }

    /// Is console platform
    #[must_use]
    pub fn is_console(&self) -> bool {
        matches!(self, Self::PlayStation5 | Self::XboxSeriesX | Self::NintendoSwitch)
    }

    /// Is VR platform
    #[must_use]
    pub fn is_vr(&self) -> bool {
        matches!(self, Self::MetaQuest | Self::AppleVisionPro | Self::SteamVR | Self::PSVR2)
    }

    /// Is web platform
    #[must_use]
    pub fn is_web(&self) -> bool {
        matches!(self, Self::WebAssembly | Self::WebGPU)
    }

    /// Get graphics backend
    #[must_use]
    pub fn graphics_backend(&self) -> GraphicsBackend {
        match self {
            Self::Windows => GraphicsBackend::DirectX12,
            Self::Linux | Self::Android | Self::SteamDeck => GraphicsBackend::Vulkan,
            Self::MacOS | Self::Ios | Self::AppleVisionPro => GraphicsBackend::Metal,
            Self::PlayStation5 | Self::PSVR2 => GraphicsBackend::GNM,
            Self::XboxSeriesX | Self::XCloud => GraphicsBackend::DirectX12,
            Self::NintendoSwitch => GraphicsBackend::NVN,
            Self::WebAssembly | Self::WebGPU => GraphicsBackend::WebGPU,
            Self::MetaQuest => GraphicsBackend::Vulkan,
            Self::SteamVR => GraphicsBackend::Vulkan,
            Self::GeForceNow | Self::PlayStationNow => GraphicsBackend::Vulkan,
            Self::RaspberryPi => GraphicsBackend::Vulkan,
        }
    }

    /// Get audio backend
    #[must_use]
    pub fn audio_backend(&self) -> AudioBackend {
        match self {
            Self::Windows | Self::XboxSeriesX | Self::XCloud => AudioBackend::XAudio2,
            Self::Linux | Self::SteamDeck | Self::RaspberryPi => AudioBackend::PulseAudio,
            Self::MacOS | Self::Ios | Self::AppleVisionPro => AudioBackend::CoreAudio,
            Self::Android | Self::MetaQuest => AudioBackend::AAudio,
            Self::PlayStation5 | Self::PSVR2 | Self::PlayStationNow => AudioBackend::Tempest,
            Self::NintendoSwitch => AudioBackend::NintendoAudio,
            Self::WebAssembly | Self::WebGPU => AudioBackend::WebAudio,
            Self::SteamVR | Self::GeForceNow => AudioBackend::PulseAudio,
        }
    }
}

/// Graphics backend
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsBackend {
    Vulkan,
    DirectX12,
    Metal,
    WebGPU,
    GNM,      // PlayStation
    NVN,      // Nintendo
    OpenGLES, // Fallback mobile
}

/// Audio backend
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioBackend {
    XAudio2,
    CoreAudio,
    PulseAudio,
    AAudio,
    Tempest,
    NintendoAudio,
    WebAudio,
}

/// Platform capabilities
#[derive(Debug, Clone)]
pub struct PlatformCapabilities {
    /// Platform
    pub platform: Platform,
    /// Max texture size
    pub max_texture_size: u32,
    /// Max render targets
    pub max_render_targets: u32,
    /// Ray tracing support
    pub ray_tracing: bool,
    /// Hardware ray tracing
    pub hardware_rt: bool,
    /// Mesh shaders support
    pub mesh_shaders: bool,
    /// Variable rate shading
    pub vrs: bool,
    /// Max memory (MB)
    pub max_memory_mb: u32,
    /// Max VRAM (MB)
    pub max_vram_mb: u32,
    /// Compute shader support
    pub compute: bool,
    /// Touch input
    pub touch: bool,
    /// Gamepad support
    pub gamepad: bool,
    /// Keyboard/mouse
    pub keyboard_mouse: bool,
    /// Motion controls
    pub motion: bool,
    /// Haptics
    pub haptics: bool,
    /// HDR support
    pub hdr: bool,
    /// Max FPS
    pub max_fps: u32,
}

impl PlatformCapabilities {
    /// Get capabilities for platform
    #[must_use]
    pub fn for_platform(platform: Platform) -> Self {
        match platform {
            Platform::Windows => Self {
                platform,
                max_texture_size: 16384,
                max_render_targets: 8,
                ray_tracing: true,
                hardware_rt: true,
                mesh_shaders: true,
                vrs: true,
                max_memory_mb: 32768,
                max_vram_mb: 24576,
                compute: true,
                touch: false,
                gamepad: true,
                keyboard_mouse: true,
                motion: false,
                haptics: true,
                hdr: true,
                max_fps: 240,
            },
            Platform::PlayStation5 => Self {
                platform,
                max_texture_size: 16384,
                max_render_targets: 8,
                ray_tracing: true,
                hardware_rt: true,
                mesh_shaders: true,
                vrs: true,
                max_memory_mb: 16384,
                max_vram_mb: 16384,
                compute: true,
                touch: true,
                gamepad: true,
                keyboard_mouse: true,
                motion: true,
                haptics: true,
                hdr: true,
                max_fps: 120,
            },
            Platform::XboxSeriesX => Self {
                platform,
                max_texture_size: 16384,
                max_render_targets: 8,
                ray_tracing: true,
                hardware_rt: true,
                mesh_shaders: true,
                vrs: true,
                max_memory_mb: 16384,
                max_vram_mb: 16384,
                compute: true,
                touch: false,
                gamepad: true,
                keyboard_mouse: true,
                motion: false,
                haptics: true,
                hdr: true,
                max_fps: 120,
            },
            Platform::NintendoSwitch => Self {
                platform,
                max_texture_size: 8192,
                max_render_targets: 4,
                ray_tracing: false,
                hardware_rt: false,
                mesh_shaders: false,
                vrs: false,
                max_memory_mb: 4096,
                max_vram_mb: 4096,
                compute: true,
                touch: true,
                gamepad: true,
                keyboard_mouse: false,
                motion: true,
                haptics: true,
                hdr: false,
                max_fps: 60,
            },
            Platform::Ios => Self {
                platform,
                max_texture_size: 8192,
                max_render_targets: 4,
                ray_tracing: true,
                hardware_rt: true,
                mesh_shaders: true,
                vrs: false,
                max_memory_mb: 6144,
                max_vram_mb: 6144,
                compute: true,
                touch: true,
                gamepad: true,
                keyboard_mouse: false,
                motion: true,
                haptics: true,
                hdr: true,
                max_fps: 120,
            },
            Platform::Android => Self {
                platform,
                max_texture_size: 8192,
                max_render_targets: 4,
                ray_tracing: true,
                hardware_rt: false,
                mesh_shaders: false,
                vrs: true,
                max_memory_mb: 8192,
                max_vram_mb: 4096,
                compute: true,
                touch: true,
                gamepad: true,
                keyboard_mouse: false,
                motion: true,
                haptics: true,
                hdr: true,
                max_fps: 120,
            },
            Platform::MetaQuest => Self {
                platform,
                max_texture_size: 4096,
                max_render_targets: 4,
                ray_tracing: false,
                hardware_rt: false,
                mesh_shaders: false,
                vrs: true,
                max_memory_mb: 6144,
                max_vram_mb: 2048,
                compute: true,
                touch: true,
                gamepad: true,
                keyboard_mouse: false,
                motion: true,
                haptics: true,
                hdr: false,
                max_fps: 120,
            },
            Platform::AppleVisionPro => Self {
                platform,
                max_texture_size: 16384,
                max_render_targets: 8,
                ray_tracing: true,
                hardware_rt: true,
                mesh_shaders: true,
                vrs: true,
                max_memory_mb: 16384,
                max_vram_mb: 16384,
                compute: true,
                touch: true,
                gamepad: true,
                keyboard_mouse: true,
                motion: true,
                haptics: true,
                hdr: true,
                max_fps: 120,
            },
            _ => Self::default_for(platform),
        }
    }

    fn default_for(platform: Platform) -> Self {
        Self {
            platform,
            max_texture_size: 4096,
            max_render_targets: 4,
            ray_tracing: false,
            hardware_rt: false,
            mesh_shaders: false,
            vrs: false,
            max_memory_mb: 4096,
            max_vram_mb: 2048,
            compute: true,
            touch: false,
            gamepad: true,
            keyboard_mouse: true,
            motion: false,
            haptics: false,
            hdr: false,
            max_fps: 60,
        }
    }
}

/// Build configuration
#[derive(Debug, Clone)]
pub struct BuildConfig {
    /// Target platform
    pub platform: Platform,
    /// Build type
    pub build_type: BuildType,
    /// Architecture
    pub arch: Architecture,
    /// Optimizations
    pub optimizations: OptimizationLevel,
    /// Strip symbols
    pub strip: bool,
    /// LTO enabled
    pub lto: bool,
    /// Output path
    pub output_path: String,
    /// Icon path
    pub icon: Option<String>,
    /// App name
    pub app_name: String,
    /// Version
    pub version: String,
    /// Bundle ID (mobile)
    pub bundle_id: Option<String>,
    /// Signing identity
    pub signing_identity: Option<String>,
}

/// Build type
#[derive(Debug, Clone, Copy)]
pub enum BuildType {
    Debug,
    Release,
    Profile,
    Shipping,
}

/// Target architecture
#[derive(Debug, Clone, Copy)]
pub enum Architecture {
    X86_64,
    ARM64,
    WASM32,
    Universal,
}

/// Optimization level
#[derive(Debug, Clone, Copy)]
pub enum OptimizationLevel {
    None,
    Size,
    Speed,
    Aggressive,
}

/// Platform builder
pub struct PlatformBuilder {
    /// Configurations
    configs: HashMap<Platform, BuildConfig>,
}

impl Default for PlatformBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformBuilder {
    /// Create new builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    /// Add build config
    pub fn add_config(&mut self, config: BuildConfig) {
        self.configs.insert(config.platform, config);
    }

    /// Build for platform
    ///
    /// # Errors
    /// Returns error if build fails
    pub fn build(&self, platform: Platform) -> Result<BuildResult, BuildError> {
        let config = self.configs.get(&platform)
            .ok_or(BuildError::NoPlatformConfig)?;

        // Simulate build process
        let result = BuildResult {
            platform,
            success: true,
            output_path: config.output_path.clone(),
            size_bytes: 50_000_000,
            build_time_secs: 120.0,
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        Ok(result)
    }

    /// Build for all configured platforms
    pub fn build_all(&self) -> Vec<Result<BuildResult, BuildError>> {
        self.configs.keys()
            .map(|&platform| self.build(platform))
            .collect()
    }
}

/// Build result
#[derive(Debug)]
pub struct BuildResult {
    /// Platform
    pub platform: Platform,
    /// Success
    pub success: bool,
    /// Output path
    pub output_path: String,
    /// Size in bytes
    pub size_bytes: u64,
    /// Build time
    pub build_time_secs: f64,
    /// Warnings
    pub warnings: Vec<String>,
    /// Errors
    pub errors: Vec<String>,
}

/// Build error
#[derive(Debug, Clone)]
pub enum BuildError {
    /// No config for platform
    NoPlatformConfig,
    /// Compilation failed
    CompilationFailed(String),
    /// Linking failed
    LinkingFailed(String),
    /// Signing failed
    SigningFailed(String),
    /// Packaging failed
    PackagingFailed(String),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoPlatformConfig => write!(f, "No platform configuration"),
            Self::CompilationFailed(e) => write!(f, "Compilation failed: {e}"),
            Self::LinkingFailed(e) => write!(f, "Linking failed: {e}"),
            Self::SigningFailed(e) => write!(f, "Signing failed: {e}"),
            Self::PackagingFailed(e) => write!(f, "Packaging failed: {e}"),
        }
    }
}

impl std::error::Error for BuildError {}
