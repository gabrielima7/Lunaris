//! Post-Processing Pipeline
//!
//! Screen-space effects like bloom, tone mapping, and color grading.

use lunaris_core::math::Color;

/// Post-processing effect trait
pub trait PostProcessEffect: Send + Sync {
    /// Effect name
    fn name(&self) -> &str;
    /// Is enabled
    fn is_enabled(&self) -> bool;
    /// Set enabled
    fn set_enabled(&mut self, enabled: bool);
    /// Effect priority (lower = first)
    fn priority(&self) -> i32;
}

/// Post-processing pipeline configuration
#[derive(Debug, Clone)]
pub struct PostProcessConfig {
    /// HDR enabled
    pub hdr: bool,
    /// MSAA samples
    pub msaa_samples: u32,
    /// Render scale
    pub render_scale: f32,
}

impl Default for PostProcessConfig {
    fn default() -> Self {
        Self {
            hdr: true,
            msaa_samples: 4,
            render_scale: 1.0,
        }
    }
}

/// Bloom effect settings
#[derive(Debug, Clone)]
pub struct BloomSettings {
    /// Enabled
    pub enabled: bool,
    /// Intensity
    pub intensity: f32,
    /// Threshold
    pub threshold: f32,
    /// Soft threshold
    pub soft_threshold: f32,
    /// Scatter
    pub scatter: f32,
    /// Max iterations
    pub max_iterations: u32,
}

impl Default for BloomSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            intensity: 0.5,
            threshold: 1.0,
            soft_threshold: 0.5,
            scatter: 0.7,
            max_iterations: 8,
        }
    }
}

/// Bloom effect
pub struct Bloom {
    /// Settings
    pub settings: BloomSettings,
}

impl Default for Bloom {
    fn default() -> Self {
        Self::new()
    }
}

impl Bloom {
    /// Create new bloom effect
    #[must_use]
    pub fn new() -> Self {
        Self {
            settings: BloomSettings::default(),
        }
    }
}

impl PostProcessEffect for Bloom {
    fn name(&self) -> &str { "Bloom" }
    fn is_enabled(&self) -> bool { self.settings.enabled }
    fn set_enabled(&mut self, enabled: bool) { self.settings.enabled = enabled; }
    fn priority(&self) -> i32 { 100 }
}

/// Tone mapping mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToneMappingMode {
    /// No tone mapping
    None,
    /// Reinhard
    #[default]
    Reinhard,
    /// ACES filmic
    AcesFilmic,
    /// Uncharted 2
    Uncharted2,
    /// Neutral
    Neutral,
}

/// Tone mapping settings
#[derive(Debug, Clone)]
pub struct ToneMappingSettings {
    /// Enabled
    pub enabled: bool,
    /// Mode
    pub mode: ToneMappingMode,
    /// Exposure
    pub exposure: f32,
    /// Gamma
    pub gamma: f32,
    /// White point
    pub white_point: f32,
}

impl Default for ToneMappingSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: ToneMappingMode::Reinhard,
            exposure: 1.0,
            gamma: 2.2,
            white_point: 4.0,
        }
    }
}

/// Tone mapping pass
pub struct ToneMapping {
    /// Settings
    pub settings: ToneMappingSettings,
}

impl Default for ToneMapping {
    fn default() -> Self {
        Self::new()
    }
}

impl ToneMapping {
    /// Create new tone mapping
    #[must_use]
    pub fn new() -> Self {
        Self {
            settings: ToneMappingSettings::default(),
        }
    }
}

impl PostProcessEffect for ToneMapping {
    fn name(&self) -> &str { "ToneMapping" }
    fn is_enabled(&self) -> bool { self.settings.enabled }
    fn set_enabled(&mut self, enabled: bool) { self.settings.enabled = enabled; }
    fn priority(&self) -> i32 { 200 }
}

/// Color grading settings
#[derive(Debug, Clone)]
pub struct ColorGradingSettings {
    /// Enabled
    pub enabled: bool,
    /// Saturation (0 = grayscale, 1 = normal, 2 = oversaturated)
    pub saturation: f32,
    /// Contrast
    pub contrast: f32,
    /// Brightness
    pub brightness: f32,
    /// Temperature (cold = -1, neutral = 0, warm = 1)
    pub temperature: f32,
    /// Tint (green = -1, neutral = 0, magenta = 1)
    pub tint: f32,
    /// Color filter
    pub color_filter: Color,
    /// Shadows color
    pub shadows: Color,
    /// Midtones color
    pub midtones: Color,
    /// Highlights color
    pub highlights: Color,
}

impl Default for ColorGradingSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            saturation: 1.0,
            contrast: 1.0,
            brightness: 0.0,
            temperature: 0.0,
            tint: 0.0,
            color_filter: Color::WHITE,
            shadows: Color::new(0.5, 0.5, 0.5, 1.0),
            midtones: Color::new(0.5, 0.5, 0.5, 1.0),
            highlights: Color::new(0.5, 0.5, 0.5, 1.0),
        }
    }
}

/// Color grading effect
pub struct ColorGrading {
    /// Settings
    pub settings: ColorGradingSettings,
}

impl Default for ColorGrading {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorGrading {
    /// Create new color grading
    #[must_use]
    pub fn new() -> Self {
        Self {
            settings: ColorGradingSettings::default(),
        }
    }
}

impl PostProcessEffect for ColorGrading {
    fn name(&self) -> &str { "ColorGrading" }
    fn is_enabled(&self) -> bool { self.settings.enabled }
    fn set_enabled(&mut self, enabled: bool) { self.settings.enabled = enabled; }
    fn priority(&self) -> i32 { 300 }
}

/// Vignette settings
#[derive(Debug, Clone)]
pub struct VignetteSettings {
    /// Enabled
    pub enabled: bool,
    /// Intensity
    pub intensity: f32,
    /// Smoothness
    pub smoothness: f32,
    /// Roundness
    pub roundness: f32,
    /// Center
    pub center: (f32, f32),
    /// Color
    pub color: Color,
}

impl Default for VignetteSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            intensity: 0.3,
            smoothness: 0.5,
            roundness: 1.0,
            center: (0.5, 0.5),
            color: Color::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}

/// Vignette effect
pub struct Vignette {
    /// Settings
    pub settings: VignetteSettings,
}

impl Default for Vignette {
    fn default() -> Self {
        Self::new()
    }
}

impl Vignette {
    /// Create new vignette
    #[must_use]
    pub fn new() -> Self {
        Self {
            settings: VignetteSettings::default(),
        }
    }
}

impl PostProcessEffect for Vignette {
    fn name(&self) -> &str { "Vignette" }
    fn is_enabled(&self) -> bool { self.settings.enabled }
    fn set_enabled(&mut self, enabled: bool) { self.settings.enabled = enabled; }
    fn priority(&self) -> i32 { 400 }
}

/// Film grain settings
#[derive(Debug, Clone)]
pub struct FilmGrainSettings {
    /// Enabled
    pub enabled: bool,
    /// Intensity
    pub intensity: f32,
    /// Response
    pub response: f32,
}

impl Default for FilmGrainSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            intensity: 0.1,
            response: 0.8,
        }
    }
}

/// Chromatic aberration settings
#[derive(Debug, Clone)]
pub struct ChromaticAberrationSettings {
    /// Enabled
    pub enabled: bool,
    /// Intensity
    pub intensity: f32,
}

impl Default for ChromaticAberrationSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            intensity: 0.1,
        }
    }
}

/// Depth of field settings
#[derive(Debug, Clone)]
pub struct DepthOfFieldSettings {
    /// Enabled
    pub enabled: bool,
    /// Focus distance
    pub focus_distance: f32,
    /// Focal length
    pub focal_length: f32,
    /// Aperture (f-stop)
    pub aperture: f32,
    /// Bokeh shape (blade count, 0 = circle)
    pub blade_count: u32,
}

impl Default for DepthOfFieldSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            focus_distance: 10.0,
            focal_length: 50.0,
            aperture: 2.8,
            blade_count: 6,
        }
    }
}

/// Motion blur settings
#[derive(Debug, Clone)]
pub struct MotionBlurSettings {
    /// Enabled
    pub enabled: bool,
    /// Intensity
    pub intensity: f32,
    /// Sample count
    pub sample_count: u32,
}

impl Default for MotionBlurSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            intensity: 0.5,
            sample_count: 8,
        }
    }
}

/// SSAO settings
#[derive(Debug, Clone)]
pub struct SsaoSettings {
    /// Enabled
    pub enabled: bool,
    /// Intensity
    pub intensity: f32,
    /// Radius
    pub radius: f32,
    /// Bias
    pub bias: f32,
    /// Sample count
    pub sample_count: u32,
}

impl Default for SsaoSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            intensity: 0.5,
            radius: 0.5,
            bias: 0.025,
            sample_count: 16,
        }
    }
}

/// Post-processing stack
pub struct PostProcessStack {
    /// Configuration
    pub config: PostProcessConfig,
    /// Bloom
    pub bloom: Bloom,
    /// Tone mapping
    pub tone_mapping: ToneMapping,
    /// Color grading
    pub color_grading: ColorGrading,
    /// Vignette
    pub vignette: Vignette,
    /// Film grain
    pub film_grain: FilmGrainSettings,
    /// Chromatic aberration
    pub chromatic_aberration: ChromaticAberrationSettings,
    /// Depth of field
    pub depth_of_field: DepthOfFieldSettings,
    /// Motion blur
    pub motion_blur: MotionBlurSettings,
    /// SSAO
    pub ssao: SsaoSettings,
}

impl Default for PostProcessStack {
    fn default() -> Self {
        Self::new()
    }
}

impl PostProcessStack {
    /// Create a new post-process stack
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: PostProcessConfig::default(),
            bloom: Bloom::new(),
            tone_mapping: ToneMapping::new(),
            color_grading: ColorGrading::new(),
            vignette: Vignette::new(),
            film_grain: FilmGrainSettings::default(),
            chromatic_aberration: ChromaticAberrationSettings::default(),
            depth_of_field: DepthOfFieldSettings::default(),
            motion_blur: MotionBlurSettings::default(),
            ssao: SsaoSettings::default(),
        }
    }

    /// Get enabled effects count
    #[must_use]
    pub fn enabled_count(&self) -> usize {
        let mut count = 0;
        if self.bloom.is_enabled() { count += 1; }
        if self.tone_mapping.is_enabled() { count += 1; }
        if self.color_grading.is_enabled() { count += 1; }
        if self.vignette.is_enabled() { count += 1; }
        if self.film_grain.enabled { count += 1; }
        if self.chromatic_aberration.enabled { count += 1; }
        if self.depth_of_field.enabled { count += 1; }
        if self.motion_blur.enabled { count += 1; }
        if self.ssao.enabled { count += 1; }
        count
    }
}

/// WGSL shader for post-processing
pub mod shaders {
    /// Bloom threshold shader
    pub const BLOOM_THRESHOLD: &str = r#"
struct Uniforms {
    threshold: f32,
    soft_threshold: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var input_texture: texture_2d<f32>;
@group(0) @binding(2) var input_sampler: sampler;

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let color = textureSample(input_texture, input_sampler, uv);
    let brightness = max(max(color.r, color.g), color.b);
    let soft = brightness - uniforms.threshold + uniforms.soft_threshold;
    let soft_clamped = clamp(soft, 0.0, 2.0 * uniforms.soft_threshold);
    let contribution = soft_clamped * soft_clamped / (4.0 * uniforms.soft_threshold + 0.00001);
    let factor = max(brightness - uniforms.threshold, contribution) / max(brightness, 0.00001);
    return vec4<f32>(color.rgb * factor, 1.0);
}
"#;

    /// Tone mapping shader
    pub const TONE_MAPPING: &str = r#"
struct Uniforms {
    exposure: f32,
    gamma: f32,
    mode: u32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var input_texture: texture_2d<f32>;
@group(0) @binding(2) var input_sampler: sampler;

fn reinhard(color: vec3<f32>) -> vec3<f32> {
    return color / (color + vec3<f32>(1.0));
}

fn aces_filmic(color: vec3<f32>) -> vec3<f32> {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    return clamp((color * (a * color + b)) / (color * (c * color + d) + e), vec3<f32>(0.0), vec3<f32>(1.0));
}

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    var color = textureSample(input_texture, input_sampler, uv).rgb;
    color = color * uniforms.exposure;
    
    if (uniforms.mode == 1u) {
        color = reinhard(color);
    } else if (uniforms.mode == 2u) {
        color = aces_filmic(color);
    }
    
    color = pow(color, vec3<f32>(1.0 / uniforms.gamma));
    return vec4<f32>(color, 1.0);
}
"#;
}
