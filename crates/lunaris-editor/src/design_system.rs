//! Professional UI Design System
//!
//! Complete design system for creating polished, professional editor interfaces.
//! Inspired by modern design systems like Material Design, Fluent, and Carbon.

use glam::Vec2;

// ==================== DESIGN TOKENS ====================

/// Design tokens - the foundation of the design system
#[derive(Debug, Clone)]
pub struct DesignTokens {
    /// Color palette
    pub colors: ColorPalette,
    /// Typography
    pub typography: Typography,
    /// Spacing scale
    pub spacing: SpacingScale,
    /// Border radii
    pub radii: BorderRadii,
    /// Shadows
    pub shadows: Shadows,
    /// Transitions
    pub transitions: Transitions,
    /// Z-indices
    pub z_indices: ZIndices,
}

impl DesignTokens {
    /// Default dark theme tokens
    pub fn dark() -> Self {
        Self {
            colors: ColorPalette::dark(),
            typography: Typography::default(),
            spacing: SpacingScale::default(),
            radii: BorderRadii::default(),
            shadows: Shadows::dark(),
            transitions: Transitions::default(),
            z_indices: ZIndices::default(),
        }
    }

    /// Light theme tokens
    pub fn light() -> Self {
        Self {
            colors: ColorPalette::light(),
            typography: Typography::default(),
            spacing: SpacingScale::default(),
            radii: BorderRadii::default(),
            shadows: Shadows::light(),
            transitions: Transitions::default(),
            z_indices: ZIndices::default(),
        }
    }
}

/// Complete color palette
#[derive(Debug, Clone)]
pub struct ColorPalette {
    // Backgrounds
    pub bg_base: Color,
    pub bg_subtle: Color,
    pub bg_muted: Color,
    pub bg_emphasis: Color,
    pub bg_inverse: Color,

    // Foregrounds
    pub fg_default: Color,
    pub fg_muted: Color,
    pub fg_subtle: Color,
    pub fg_on_emphasis: Color,

    // Canvas (editor specific)
    pub canvas_default: Color,
    pub canvas_subtle: Color,
    pub canvas_inset: Color,

    // Borders
    pub border_default: Color,
    pub border_muted: Color,
    pub border_subtle: Color,

    // Accent colors
    pub accent_fg: Color,
    pub accent_emphasis: Color,
    pub accent_muted: Color,
    pub accent_subtle: Color,

    // Semantic colors
    pub success_fg: Color,
    pub success_emphasis: Color,
    pub success_muted: Color,
    pub success_subtle: Color,

    pub warning_fg: Color,
    pub warning_emphasis: Color,
    pub warning_muted: Color,
    pub warning_subtle: Color,

    pub danger_fg: Color,
    pub danger_emphasis: Color,
    pub danger_muted: Color,
    pub danger_subtle: Color,

    pub info_fg: Color,
    pub info_emphasis: Color,

    // Interactive states
    pub hover_overlay: Color,
    pub pressed_overlay: Color,
    pub focus_ring: Color,
    pub selection_bg: Color,
}

impl ColorPalette {
    pub fn dark() -> Self {
        Self {
            // Backgrounds - rich dark with depth
            bg_base: Color::hex("#0d1117"),
            bg_subtle: Color::hex("#161b22"),
            bg_muted: Color::hex("#21262d"),
            bg_emphasis: Color::hex("#6366f1"),
            bg_inverse: Color::hex("#f0f6fc"),

            // Foregrounds
            fg_default: Color::hex("#e6edf3"),
            fg_muted: Color::hex("#8b949e"),
            fg_subtle: Color::hex("#6e7681"),
            fg_on_emphasis: Color::hex("#ffffff"),

            // Canvas
            canvas_default: Color::hex("#0d1117"),
            canvas_subtle: Color::hex("#161b22"),
            canvas_inset: Color::hex("#010409"),

            // Borders
            border_default: Color::hex("#30363d"),
            border_muted: Color::hex("#21262d"),
            border_subtle: Color::hex("#1b1f24"),

            // Accent (Indigo)
            accent_fg: Color::hex("#818cf8"),
            accent_emphasis: Color::hex("#6366f1"),
            accent_muted: Color::rgba(99, 102, 241, 0.4),
            accent_subtle: Color::rgba(99, 102, 241, 0.15),

            // Success (Green)
            success_fg: Color::hex("#4ade80"),
            success_emphasis: Color::hex("#22c55e"),
            success_muted: Color::rgba(34, 197, 94, 0.4),
            success_subtle: Color::rgba(34, 197, 94, 0.15),

            // Warning (Amber)
            warning_fg: Color::hex("#fbbf24"),
            warning_emphasis: Color::hex("#f59e0b"),
            warning_muted: Color::rgba(251, 191, 36, 0.4),
            warning_subtle: Color::rgba(251, 191, 36, 0.15),

            // Danger (Red)
            danger_fg: Color::hex("#f87171"),
            danger_emphasis: Color::hex("#ef4444"),
            danger_muted: Color::rgba(239, 68, 68, 0.4),
            danger_subtle: Color::rgba(239, 68, 68, 0.15),

            // Info (Blue)
            info_fg: Color::hex("#60a5fa"),
            info_emphasis: Color::hex("#3b82f6"),

            // Interactive
            hover_overlay: Color::rgba(255, 255, 255, 0.05),
            pressed_overlay: Color::rgba(255, 255, 255, 0.1),
            focus_ring: Color::rgba(99, 102, 241, 0.5),
            selection_bg: Color::rgba(99, 102, 241, 0.3),
        }
    }

    pub fn light() -> Self {
        Self {
            bg_base: Color::hex("#ffffff"),
            bg_subtle: Color::hex("#f6f8fa"),
            bg_muted: Color::hex("#eaeef2"),
            bg_emphasis: Color::hex("#6366f1"),
            bg_inverse: Color::hex("#24292f"),

            fg_default: Color::hex("#1f2328"),
            fg_muted: Color::hex("#57606a"),
            fg_subtle: Color::hex("#6e7781"),
            fg_on_emphasis: Color::hex("#ffffff"),

            canvas_default: Color::hex("#ffffff"),
            canvas_subtle: Color::hex("#f6f8fa"),
            canvas_inset: Color::hex("#eaeef2"),

            border_default: Color::hex("#d0d7de"),
            border_muted: Color::hex("#d8dee4"),
            border_subtle: Color::hex("#eaeef2"),

            accent_fg: Color::hex("#4f46e5"),
            accent_emphasis: Color::hex("#6366f1"),
            accent_muted: Color::rgba(99, 102, 241, 0.4),
            accent_subtle: Color::rgba(99, 102, 241, 0.1),

            success_fg: Color::hex("#16a34a"),
            success_emphasis: Color::hex("#22c55e"),
            success_muted: Color::rgba(34, 197, 94, 0.4),
            success_subtle: Color::rgba(34, 197, 94, 0.1),

            warning_fg: Color::hex("#d97706"),
            warning_emphasis: Color::hex("#f59e0b"),
            warning_muted: Color::rgba(251, 191, 36, 0.4),
            warning_subtle: Color::rgba(251, 191, 36, 0.1),

            danger_fg: Color::hex("#dc2626"),
            danger_emphasis: Color::hex("#ef4444"),
            danger_muted: Color::rgba(239, 68, 68, 0.4),
            danger_subtle: Color::rgba(239, 68, 68, 0.1),

            info_fg: Color::hex("#2563eb"),
            info_emphasis: Color::hex("#3b82f6"),

            hover_overlay: Color::rgba(0, 0, 0, 0.04),
            pressed_overlay: Color::rgba(0, 0, 0, 0.08),
            focus_ring: Color::rgba(99, 102, 241, 0.4),
            selection_bg: Color::rgba(99, 102, 241, 0.2),
        }
    }
}

/// Color type
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
        Self { r, g, b, a: 1.0 }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a,
        }
    }

    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn with_alpha(&self, a: f32) -> Self {
        Self { a, ..*self }
    }

    pub fn blend(&self, other: &Color, t: f32) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }
}

/// Typography system
#[derive(Debug, Clone)]
pub struct Typography {
    pub font_family: String,
    pub font_family_mono: String,

    // Font sizes
    pub size_xs: f32,   // 10px
    pub size_sm: f32,   // 12px
    pub size_base: f32, // 14px
    pub size_lg: f32,   // 16px
    pub size_xl: f32,   // 18px
    pub size_2xl: f32,  // 20px
    pub size_3xl: f32,  // 24px
    pub size_4xl: f32,  // 30px

    // Line heights
    pub line_tight: f32,  // 1.25
    pub line_normal: f32, // 1.5
    pub line_relaxed: f32, // 1.625

    // Font weights
    pub weight_normal: u32,  // 400
    pub weight_medium: u32,  // 500
    pub weight_semibold: u32, // 600
    pub weight_bold: u32,    // 700
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_family: "Inter, -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
            font_family_mono: "JetBrains Mono, Menlo, Monaco, monospace".to_string(),
            size_xs: 10.0,
            size_sm: 12.0,
            size_base: 14.0,
            size_lg: 16.0,
            size_xl: 18.0,
            size_2xl: 20.0,
            size_3xl: 24.0,
            size_4xl: 30.0,
            line_tight: 1.25,
            line_normal: 1.5,
            line_relaxed: 1.625,
            weight_normal: 400,
            weight_medium: 500,
            weight_semibold: 600,
            weight_bold: 700,
        }
    }
}

/// Spacing scale (4px base unit)
#[derive(Debug, Clone)]
pub struct SpacingScale {
    pub px: f32,     // 1px
    pub s0_5: f32,   // 2px
    pub s1: f32,     // 4px
    pub s1_5: f32,   // 6px
    pub s2: f32,     // 8px
    pub s2_5: f32,   // 10px
    pub s3: f32,     // 12px
    pub s4: f32,     // 16px
    pub s5: f32,     // 20px
    pub s6: f32,     // 24px
    pub s8: f32,     // 32px
    pub s10: f32,    // 40px
    pub s12: f32,    // 48px
    pub s16: f32,    // 64px
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self {
            px: 1.0,
            s0_5: 2.0,
            s1: 4.0,
            s1_5: 6.0,
            s2: 8.0,
            s2_5: 10.0,
            s3: 12.0,
            s4: 16.0,
            s5: 20.0,
            s6: 24.0,
            s8: 32.0,
            s10: 40.0,
            s12: 48.0,
            s16: 64.0,
        }
    }
}

/// Border radii
#[derive(Debug, Clone)]
pub struct BorderRadii {
    pub none: f32,
    pub sm: f32,     // 2px
    pub base: f32,   // 4px
    pub md: f32,     // 6px
    pub lg: f32,     // 8px
    pub xl: f32,     // 12px
    pub full: f32,   // 9999px
}

impl Default for BorderRadii {
    fn default() -> Self {
        Self {
            none: 0.0,
            sm: 2.0,
            base: 4.0,
            md: 6.0,
            lg: 8.0,
            xl: 12.0,
            full: 9999.0,
        }
    }
}

/// Shadow definitions
#[derive(Debug, Clone)]
pub struct Shadows {
    pub sm: Shadow,
    pub base: Shadow,
    pub md: Shadow,
    pub lg: Shadow,
    pub xl: Shadow,
    pub inner: Shadow,
    pub glow: Shadow,
}

impl Shadows {
    pub fn dark() -> Self {
        Self {
            sm: Shadow::new(0.0, 1.0, 2.0, 0.0, Color::rgba(0, 0, 0, 0.3)),
            base: Shadow::new(0.0, 1.0, 3.0, 0.0, Color::rgba(0, 0, 0, 0.4)),
            md: Shadow::new(0.0, 4.0, 6.0, -1.0, Color::rgba(0, 0, 0, 0.4)),
            lg: Shadow::new(0.0, 10.0, 15.0, -3.0, Color::rgba(0, 0, 0, 0.5)),
            xl: Shadow::new(0.0, 20.0, 25.0, -5.0, Color::rgba(0, 0, 0, 0.5)),
            inner: Shadow::inset(0.0, 2.0, 4.0, 0.0, Color::rgba(0, 0, 0, 0.3)),
            glow: Shadow::new(0.0, 0.0, 20.0, 0.0, Color::rgba(99, 102, 241, 0.4)),
        }
    }

    pub fn light() -> Self {
        Self {
            sm: Shadow::new(0.0, 1.0, 2.0, 0.0, Color::rgba(0, 0, 0, 0.05)),
            base: Shadow::new(0.0, 1.0, 3.0, 0.0, Color::rgba(0, 0, 0, 0.1)),
            md: Shadow::new(0.0, 4.0, 6.0, -1.0, Color::rgba(0, 0, 0, 0.1)),
            lg: Shadow::new(0.0, 10.0, 15.0, -3.0, Color::rgba(0, 0, 0, 0.1)),
            xl: Shadow::new(0.0, 20.0, 25.0, -5.0, Color::rgba(0, 0, 0, 0.15)),
            inner: Shadow::inset(0.0, 2.0, 4.0, 0.0, Color::rgba(0, 0, 0, 0.06)),
            glow: Shadow::new(0.0, 0.0, 20.0, 0.0, Color::rgba(99, 102, 241, 0.3)),
        }
    }
}

/// Shadow definition
#[derive(Debug, Clone, Copy)]
pub struct Shadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub spread: f32,
    pub color: Color,
    pub inset: bool,
}

impl Shadow {
    pub fn new(x: f32, y: f32, blur: f32, spread: f32, color: Color) -> Self {
        Self { offset_x: x, offset_y: y, blur, spread, color, inset: false }
    }

    pub fn inset(x: f32, y: f32, blur: f32, spread: f32, color: Color) -> Self {
        Self { offset_x: x, offset_y: y, blur, spread, color, inset: true }
    }
}

/// Transition settings
#[derive(Debug, Clone)]
pub struct Transitions {
    pub duration_fast: f32,     // 100ms
    pub duration_normal: f32,   // 200ms
    pub duration_slow: f32,     // 300ms
    pub duration_slower: f32,   // 500ms
    pub easing_default: Easing,
    pub easing_in: Easing,
    pub easing_out: Easing,
    pub easing_in_out: Easing,
}

impl Default for Transitions {
    fn default() -> Self {
        Self {
            duration_fast: 0.1,
            duration_normal: 0.2,
            duration_slow: 0.3,
            duration_slower: 0.5,
            easing_default: Easing::EaseOut,
            easing_in: Easing::EaseIn,
            easing_out: Easing::EaseOut,
            easing_in_out: Easing::EaseInOut,
        }
    }
}

/// Easing functions
#[derive(Debug, Clone, Copy)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    Spring,
}

impl Easing {
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Self::EaseInOut => {
                if t < 0.5 { 2.0 * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(2) / 2.0 }
            }
            Self::EaseInQuad => t * t,
            Self::EaseOutQuad => 1.0 - (1.0 - t).powi(2),
            Self::EaseInOutQuad => {
                if t < 0.5 { 2.0 * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(2) / 2.0 }
            }
            Self::EaseInCubic => t * t * t,
            Self::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            Self::EaseInOutCubic => {
                if t < 0.5 { 4.0 * t * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(3) / 2.0 }
            }
            Self::Spring => {
                let c4 = (2.0 * std::f32::consts::PI) / 3.0;
                if t == 0.0 { 0.0 }
                else if t == 1.0 { 1.0 }
                else { 2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0 }
            }
        }
    }
}

/// Z-index scale
#[derive(Debug, Clone)]
pub struct ZIndices {
    pub base: i32,
    pub docked: i32,
    pub dropdown: i32,
    pub sticky: i32,
    pub modal_backdrop: i32,
    pub modal: i32,
    pub popover: i32,
    pub tooltip: i32,
    pub toast: i32,
}

impl Default for ZIndices {
    fn default() -> Self {
        Self {
            base: 0,
            docked: 10,
            dropdown: 1000,
            sticky: 1100,
            modal_backdrop: 1200,
            modal: 1300,
            popover: 1400,
            tooltip: 1500,
            toast: 1600,
        }
    }
}

// ==================== ICON SYSTEM ====================

/// Icon set for the editor
pub struct IconSet {
    icons: std::collections::HashMap<String, IconDef>,
}

/// Icon definition
#[derive(Debug, Clone)]
pub struct IconDef {
    pub name: String,
    pub path_data: String, // SVG path data
    pub view_box: (f32, f32, f32, f32),
}

impl IconSet {
    pub fn editor_icons() -> Self {
        let mut icons = std::collections::HashMap::new();

        // File operations
        icons.insert("file".to_string(), IconDef::path("M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z", "M14 2v6h6"));
        icons.insert("folder".to_string(), IconDef::path("M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z", ""));
        icons.insert("save".to_string(), IconDef::path("M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z", "M17 21v-8H7v8M7 3v5h8"));
        
        // Edit operations
        icons.insert("undo".to_string(), IconDef::path("M3 7v6h6", "M21 17a9 9 0 0 0-9-9 9 9 0 0 0-6 2.3L3 13"));
        icons.insert("redo".to_string(), IconDef::path("M21 7v6h-6", "M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3L21 13"));
        icons.insert("copy".to_string(), IconDef::path("M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2", ""));
        icons.insert("paste".to_string(), IconDef::path("M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2", "M8 2h8v4H8z"));
        
        // Transform tools
        icons.insert("move".to_string(), IconDef::path("M5 9l-3 3l3 3M9 5l3-3l3 3M15 19l-3 3l-3-3M19 9l3 3l-3 3M2 12h20M12 2v20", ""));
        icons.insert("rotate".to_string(), IconDef::path("M23 4v6h-6", "M20.49 15a9 9 0 1 1-2.12-9.36L23 10"));
        icons.insert("scale".to_string(), IconDef::path("M21 21l-6-6m6 6v-4.8m0 4.8h-4.8", "M3 16.2V21h4.8M3 3h4.8M21 3v4.8M3 3l6 6"));
        
        // View controls
        icons.insert("eye".to_string(), IconDef::path("M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z", "M12 9a3 3 0 1 0 0 6 3 3 0 0 0 0-6z"));
        icons.insert("eye-off".to_string(), IconDef::path("M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24", "M1 1l22 22"));
        icons.insert("lock".to_string(), IconDef::path("M19 11H5a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7a2 2 0 0 0-2-2z", "M7 11V7a5 5 0 0 1 10 0v4"));
        icons.insert("unlock".to_string(), IconDef::path("M19 11H5a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7a2 2 0 0 0-2-2z", "M7 11V7a5 5 0 0 1 9.9-1"));
        
        // Playback controls
        icons.insert("play".to_string(), IconDef::path("M5 3l14 9l-14 9V3z", ""));
        icons.insert("pause".to_string(), IconDef::path("M6 4h4v16H6zM14 4h4v16h-4z", ""));
        icons.insert("stop".to_string(), IconDef::path("M6 6h12v12H6z", ""));
        icons.insert("skip-back".to_string(), IconDef::path("M19 20L9 12l10-8v16zM5 19V5", ""));
        icons.insert("skip-forward".to_string(), IconDef::path("M5 4l10 8l-10 8V4zM19 5v14", ""));
        
        // Panel icons
        icons.insert("hierarchy".to_string(), IconDef::path("M18 21a2 2 0 1 0 0-4 2 2 0 0 0 0 4zM18 11a2 2 0 1 0 0-4 2 2 0 0 0 0 4zM6 7a2 2 0 1 0 0-4 2 2 0 0 0 0 4z", "M6 7v10a4 4 0 0 0 4 4h2M6 7h6m6 2v6"));
        icons.insert("inspector".to_string(), IconDef::path("M12 3h7a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-7m0-18H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7m0-18v18", ""));
        icons.insert("console".to_string(), IconDef::path("M4 17l6-6l-6-6M12 19h8", ""));
        icons.insert("assets".to_string(), IconDef::path("M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z", "M12 11v6M9 14h6"));
        
        // Object icons
        icons.insert("cube".to_string(), IconDef::path("M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z", "M3.27 6.96L12 12.01l8.73-5.05M12 22.08V12"));
        icons.insert("sphere".to_string(), IconDef::path("M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z", "M2 12h20M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"));
        icons.insert("light".to_string(), IconDef::path("M12 7a5 5 0 1 0 0 10 5 5 0 0 0 0-10z", "M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"));
        icons.insert("camera".to_string(), IconDef::path("M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z", "M12 17a4 4 0 1 0 0-8 4 4 0 0 0 0 8z"));
        icons.insert("audio".to_string(), IconDef::path("M11 5L6 9H2v6h4l5 4V5zM19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07", ""));
        
        // Utility icons
        icons.insert("settings".to_string(), IconDef::path("M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z", "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"));
        icons.insert("search".to_string(), IconDef::path("M11 19a8 8 0 1 0 0-16 8 8 0 0 0 0 16zM21 21l-4.35-4.35", ""));
        icons.insert("plus".to_string(), IconDef::path("M12 5v14M5 12h14", ""));
        icons.insert("minus".to_string(), IconDef::path("M5 12h14", ""));
        icons.insert("x".to_string(), IconDef::path("M18 6L6 18M6 6l12 12", ""));
        icons.insert("check".to_string(), IconDef::path("M20 6L9 17l-5-5", ""));
        icons.insert("chevron-right".to_string(), IconDef::path("M9 18l6-6l-6-6", ""));
        icons.insert("chevron-down".to_string(), IconDef::path("M6 9l6 6l6-6", ""));
        icons.insert("menu".to_string(), IconDef::path("M3 12h18M3 6h18M3 18h18", ""));
        
        Self { icons }
    }

    pub fn get(&self, name: &str) -> Option<&IconDef> {
        self.icons.get(name)
    }
}

impl IconDef {
    fn path(d: &str, extra: &str) -> Self {
        let path_data = if extra.is_empty() {
            d.to_string()
        } else {
            format!("{} {}", d, extra)
        };
        Self {
            name: String::new(),
            path_data,
            view_box: (0.0, 0.0, 24.0, 24.0),
        }
    }
}

// ==================== COMPONENT STYLES ====================

/// Button variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
    Success,
}

/// Button sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    XSmall,
    Small,
    Medium,
    Large,
}

/// Input states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputState {
    Default,
    Hover,
    Focus,
    Disabled,
    Error,
    Success,
}

/// Component style builder
pub struct StyleBuilder<'a> {
    tokens: &'a DesignTokens,
}

impl<'a> StyleBuilder<'a> {
    pub fn new(tokens: &'a DesignTokens) -> Self {
        Self { tokens }
    }

    pub fn button(&self, variant: ButtonVariant, size: ButtonSize) -> ButtonStyle {
        let (height, padding_h, font_size, radius) = match size {
            ButtonSize::XSmall => (24.0, 8.0, self.tokens.typography.size_xs, self.tokens.radii.sm),
            ButtonSize::Small => (28.0, 12.0, self.tokens.typography.size_sm, self.tokens.radii.base),
            ButtonSize::Medium => (32.0, 16.0, self.tokens.typography.size_base, self.tokens.radii.md),
            ButtonSize::Large => (40.0, 20.0, self.tokens.typography.size_lg, self.tokens.radii.lg),
        };

        let (bg, fg, border, hover_bg) = match variant {
            ButtonVariant::Primary => (
                self.tokens.colors.accent_emphasis,
                self.tokens.colors.fg_on_emphasis,
                Color::new(0.0, 0.0, 0.0, 0.0),
                self.tokens.colors.accent_fg,
            ),
            ButtonVariant::Secondary => (
                self.tokens.colors.bg_muted,
                self.tokens.colors.fg_default,
                self.tokens.colors.border_default,
                self.tokens.colors.bg_emphasis.with_alpha(0.1),
            ),
            ButtonVariant::Outline => (
                Color::new(0.0, 0.0, 0.0, 0.0),
                self.tokens.colors.fg_default,
                self.tokens.colors.border_default,
                self.tokens.colors.hover_overlay,
            ),
            ButtonVariant::Ghost => (
                Color::new(0.0, 0.0, 0.0, 0.0),
                self.tokens.colors.fg_muted,
                Color::new(0.0, 0.0, 0.0, 0.0),
                self.tokens.colors.hover_overlay,
            ),
            ButtonVariant::Danger => (
                self.tokens.colors.danger_emphasis,
                self.tokens.colors.fg_on_emphasis,
                Color::new(0.0, 0.0, 0.0, 0.0),
                self.tokens.colors.danger_fg,
            ),
            ButtonVariant::Success => (
                self.tokens.colors.success_emphasis,
                self.tokens.colors.fg_on_emphasis,
                Color::new(0.0, 0.0, 0.0, 0.0),
                self.tokens.colors.success_fg,
            ),
        };

        ButtonStyle {
            height,
            padding_horizontal: padding_h,
            font_size,
            border_radius: radius,
            background: bg,
            foreground: fg,
            border: border,
            hover_background: hover_bg,
            focus_ring: self.tokens.colors.focus_ring,
        }
    }
}

/// Button style configuration
#[derive(Debug, Clone)]
pub struct ButtonStyle {
    pub height: f32,
    pub padding_horizontal: f32,
    pub font_size: f32,
    pub border_radius: f32,
    pub background: Color,
    pub foreground: Color,
    pub border: Color,
    pub hover_background: Color,
    pub focus_ring: Color,
}
