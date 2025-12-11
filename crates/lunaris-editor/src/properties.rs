//! Property Editors and Color Picker
//!
//! Advanced property editing widgets.

use glam::{Vec2, Vec3, Vec4};
use std::any::Any;

// ==================== COLOR PICKER ====================

/// HSV color
#[derive(Debug, Clone, Copy)]
pub struct HsvColor {
    /// Hue (0-360)
    pub h: f32,
    /// Saturation (0-1)
    pub s: f32,
    /// Value (0-1)
    pub v: f32,
    /// Alpha (0-1)
    pub a: f32,
}

impl HsvColor {
    /// Create from HSV
    #[must_use]
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        Self { h, s, v, a: 1.0 }
    }

    /// Create from RGBA
    #[must_use]
    pub fn from_rgb(r: f32, g: f32, b: f32, a: f32) -> Self {
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let v = max;
        let s = if max > 0.0 { delta / max } else { 0.0 };

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };

        let h = if h < 0.0 { h + 360.0 } else { h };

        Self { h, s, v, a }
    }

    /// Convert to RGBA
    #[must_use]
    pub fn to_rgb(&self) -> [f32; 4] {
        let c = self.v * self.s;
        let x = c * (1.0 - ((self.h / 60.0) % 2.0 - 1.0).abs());
        let m = self.v - c;

        let (r, g, b) = if self.h < 60.0 {
            (c, x, 0.0)
        } else if self.h < 120.0 {
            (x, c, 0.0)
        } else if self.h < 180.0 {
            (0.0, c, x)
        } else if self.h < 240.0 {
            (0.0, x, c)
        } else if self.h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        [r + m, g + m, b + m, self.a]
    }

    /// Convert to hex string
    #[must_use]
    pub fn to_hex(&self) -> String {
        let [r, g, b, a] = self.to_rgb();
        format!("#{:02X}{:02X}{:02X}{:02X}",
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
            (a * 255.0) as u8
        )
    }

    /// Parse from hex string
    #[must_use]
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        
        let (r, g, b, a) = match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
            }
            _ => return None,
        };

        Some(Self::from_rgb(r, g, b, a))
    }
}

/// Color picker widget
pub struct ColorPicker {
    /// Current color (HSV)
    pub current: HsvColor,
    /// Original color (for comparison)
    pub original: HsvColor,
    /// Picker mode
    pub mode: ColorPickerMode,
    /// Show alpha
    pub show_alpha: bool,
    /// Recent colors
    pub recent: Vec<HsvColor>,
    /// Favorite colors
    pub favorites: Vec<HsvColor>,
    /// Expanded state
    pub expanded: bool,
    /// Hex input
    pub hex_input: String,
}

/// Color picker mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorPickerMode {
    /// Hue wheel + SV square
    Wheel,
    /// RGB sliders
    Rgb,
    /// HSV sliders
    Hsv,
    /// Hex input
    Hex,
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self::new([1.0, 1.0, 1.0, 1.0])
    }
}

impl ColorPicker {
    /// Create new color picker
    #[must_use]
    pub fn new(color: [f32; 4]) -> Self {
        let hsv = HsvColor::from_rgb(color[0], color[1], color[2], color[3]);
        Self {
            current: hsv,
            original: hsv,
            mode: ColorPickerMode::Wheel,
            show_alpha: true,
            recent: Vec::new(),
            favorites: Vec::new(),
            expanded: false,
            hex_input: hsv.to_hex(),
        }
    }

    /// Set color from RGBA
    pub fn set_rgb(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.current = HsvColor::from_rgb(r, g, b, a);
        self.hex_input = self.current.to_hex();
    }

    /// Get RGBA color
    #[must_use]
    pub fn get_rgb(&self) -> [f32; 4] {
        self.current.to_rgb()
    }

    /// Set hue
    pub fn set_hue(&mut self, h: f32) {
        self.current.h = h.clamp(0.0, 360.0);
        self.hex_input = self.current.to_hex();
    }

    /// Set saturation
    pub fn set_saturation(&mut self, s: f32) {
        self.current.s = s.clamp(0.0, 1.0);
        self.hex_input = self.current.to_hex();
    }

    /// Set value
    pub fn set_value(&mut self, v: f32) {
        self.current.v = v.clamp(0.0, 1.0);
        self.hex_input = self.current.to_hex();
    }

    /// Set alpha
    pub fn set_alpha(&mut self, a: f32) {
        self.current.a = a.clamp(0.0, 1.0);
        self.hex_input = self.current.to_hex();
    }

    /// Apply hex input
    pub fn apply_hex(&mut self) {
        if let Some(color) = HsvColor::from_hex(&self.hex_input) {
            self.current = color;
        }
    }

    /// Add to recent colors
    pub fn add_to_recent(&mut self) {
        // Remove if already exists
        self.recent.retain(|c| {
            let [r1, g1, b1, a1] = c.to_rgb();
            let [r2, g2, b2, a2] = self.current.to_rgb();
            (r1 - r2).abs() > 0.01 || (g1 - g2).abs() > 0.01 || 
            (b1 - b2).abs() > 0.01 || (a1 - a2).abs() > 0.01
        });

        // Add to front
        self.recent.insert(0, self.current);

        // Limit size
        if self.recent.len() > 16 {
            self.recent.pop();
        }
    }

    /// Add to favorites
    pub fn add_to_favorites(&mut self) {
        if self.favorites.len() < 32 {
            self.favorites.push(self.current);
        }
    }

    /// Reset to original
    pub fn reset(&mut self) {
        self.current = self.original;
        self.hex_input = self.current.to_hex();
    }
}

// ==================== PROPERTY EDITORS ====================

/// Property type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyType {
    Bool,
    Int,
    Float,
    String,
    Vec2,
    Vec3,
    Vec4,
    Color,
    Enum(Vec<String>),
    Object(String), // Type name
    Array(Box<PropertyType>),
    Custom(String),
}

/// Property value
#[derive(Debug, Clone)]
pub enum PropertyValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Color([f32; 4]),
    Enum(usize),
    Object(Option<u64>),
    Array(Vec<PropertyValue>),
}

/// Property definition
#[derive(Debug, Clone)]
pub struct Property {
    /// Property name
    pub name: String,
    /// Display name
    pub display_name: String,
    /// Type
    pub property_type: PropertyType,
    /// Current value
    pub value: PropertyValue,
    /// Default value
    pub default: PropertyValue,
    /// Is read-only
    pub read_only: bool,
    /// Is visible
    pub visible: bool,
    /// Category
    pub category: String,
    /// Tooltip
    pub tooltip: String,
    /// Min value (for numeric)
    pub min: Option<f32>,
    /// Max value (for numeric)
    pub max: Option<f32>,
    /// Step (for numeric)
    pub step: f32,
    /// Metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl Property {
    /// Create bool property
    #[must_use]
    pub fn bool(name: &str, value: bool) -> Self {
        Self {
            name: name.to_string(),
            display_name: name.to_string(),
            property_type: PropertyType::Bool,
            value: PropertyValue::Bool(value),
            default: PropertyValue::Bool(value),
            read_only: false,
            visible: true,
            category: "General".to_string(),
            tooltip: String::new(),
            min: None,
            max: None,
            step: 1.0,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create float property
    #[must_use]
    pub fn float(name: &str, value: f32) -> Self {
        Self {
            name: name.to_string(),
            display_name: name.to_string(),
            property_type: PropertyType::Float,
            value: PropertyValue::Float(value),
            default: PropertyValue::Float(value),
            read_only: false,
            visible: true,
            category: "General".to_string(),
            tooltip: String::new(),
            min: None,
            max: None,
            step: 0.1,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create vector3 property
    #[must_use]
    pub fn vec3(name: &str, value: Vec3) -> Self {
        Self {
            name: name.to_string(),
            display_name: name.to_string(),
            property_type: PropertyType::Vec3,
            value: PropertyValue::Vec3(value),
            default: PropertyValue::Vec3(value),
            read_only: false,
            visible: true,
            category: "General".to_string(),
            tooltip: String::new(),
            min: None,
            max: None,
            step: 0.1,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create color property
    #[must_use]
    pub fn color(name: &str, value: [f32; 4]) -> Self {
        Self {
            name: name.to_string(),
            display_name: name.to_string(),
            property_type: PropertyType::Color,
            value: PropertyValue::Color(value),
            default: PropertyValue::Color(value),
            read_only: false,
            visible: true,
            category: "General".to_string(),
            tooltip: String::new(),
            min: None,
            max: None,
            step: 0.01,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set range
    #[must_use]
    pub fn with_range(mut self, min: f32, max: f32) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    /// Set step
    #[must_use]
    pub fn with_step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    /// Set category
    #[must_use]
    pub fn with_category(mut self, category: &str) -> Self {
        self.category = category.to_string();
        self
    }

    /// Set tooltip
    #[must_use]
    pub fn with_tooltip(mut self, tooltip: &str) -> Self {
        self.tooltip = tooltip.to_string();
        self
    }

    /// Reset to default
    pub fn reset(&mut self) {
        self.value = self.default.clone();
    }
}

/// Property grid for editing multiple properties
pub struct PropertyGrid {
    /// Properties grouped by category
    pub properties: Vec<Property>,
    /// Category fold states
    pub category_expanded: std::collections::HashMap<String, bool>,
    /// Search filter
    pub search: String,
    /// Show advanced
    pub show_advanced: bool,
}

impl Default for PropertyGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl PropertyGrid {
    /// Create new property grid
    #[must_use]
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
            category_expanded: std::collections::HashMap::new(),
            search: String::new(),
            show_advanced: false,
        }
    }

    /// Add property
    pub fn add(&mut self, property: Property) {
        let category = property.category.clone();
        
        // Ensure category is expanded by default
        self.category_expanded.entry(category).or_insert(true);
        
        self.properties.push(property);
    }

    /// Get property by name
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Property> {
        self.properties.iter().find(|p| p.name == name)
    }

    /// Get property mutably
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Property> {
        self.properties.iter_mut().find(|p| p.name == name)
    }

    /// Get categories
    #[must_use]
    pub fn categories(&self) -> Vec<String> {
        let mut cats: Vec<_> = self.properties.iter()
            .map(|p| p.category.clone())
            .collect();
        cats.sort();
        cats.dedup();
        cats
    }

    /// Get properties in category
    #[must_use]
    pub fn properties_in_category(&self, category: &str) -> Vec<&Property> {
        self.properties.iter()
            .filter(|p| p.category == category)
            .filter(|p| p.visible)
            .filter(|p| {
                if self.search.is_empty() {
                    true
                } else {
                    p.name.to_lowercase().contains(&self.search.to_lowercase()) ||
                    p.display_name.to_lowercase().contains(&self.search.to_lowercase())
                }
            })
            .collect()
    }

    /// Toggle category
    pub fn toggle_category(&mut self, category: &str) {
        let expanded = self.category_expanded.entry(category.to_string()).or_insert(true);
        *expanded = !*expanded;
    }

    /// Reset all to defaults
    pub fn reset_all(&mut self) {
        for prop in &mut self.properties {
            prop.reset();
        }
    }
}

// ==================== GRADIENT EDITOR ====================

/// Gradient stop
#[derive(Debug, Clone)]
pub struct GradientStop {
    /// Position (0-1)
    pub position: f32,
    /// Color
    pub color: [f32; 4],
}

/// Gradient
#[derive(Debug, Clone)]
pub struct Gradient {
    /// Color stops
    pub stops: Vec<GradientStop>,
    /// Interpolation mode
    pub interpolation: GradientInterpolation,
}

/// Gradient interpolation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradientInterpolation {
    Linear,
    Smooth,
    Step,
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            stops: vec![
                GradientStop { position: 0.0, color: [0.0, 0.0, 0.0, 1.0] },
                GradientStop { position: 1.0, color: [1.0, 1.0, 1.0, 1.0] },
            ],
            interpolation: GradientInterpolation::Linear,
        }
    }
}

impl Gradient {
    /// Evaluate gradient at position
    #[must_use]
    pub fn evaluate(&self, t: f32) -> [f32; 4] {
        let t = t.clamp(0.0, 1.0);

        if self.stops.is_empty() {
            return [0.0, 0.0, 0.0, 1.0];
        }

        if self.stops.len() == 1 {
            return self.stops[0].color;
        }

        // Find surrounding stops
        let mut left = &self.stops[0];
        let mut right = &self.stops[self.stops.len() - 1];

        for i in 0..self.stops.len() - 1 {
            if self.stops[i].position <= t && self.stops[i + 1].position >= t {
                left = &self.stops[i];
                right = &self.stops[i + 1];
                break;
            }
        }

        if right.position == left.position {
            return left.color;
        }

        let local_t = (t - left.position) / (right.position - left.position);

        let blend_t = match self.interpolation {
            GradientInterpolation::Linear => local_t,
            GradientInterpolation::Smooth => local_t * local_t * (3.0 - 2.0 * local_t),
            GradientInterpolation::Step => if local_t < 0.5 { 0.0 } else { 1.0 },
        };

        [
            left.color[0] + (right.color[0] - left.color[0]) * blend_t,
            left.color[1] + (right.color[1] - left.color[1]) * blend_t,
            left.color[2] + (right.color[2] - left.color[2]) * blend_t,
            left.color[3] + (right.color[3] - left.color[3]) * blend_t,
        ]
    }

    /// Add stop
    pub fn add_stop(&mut self, position: f32, color: [f32; 4]) {
        let stop = GradientStop { position, color };
        let idx = self.stops.partition_point(|s| s.position < position);
        self.stops.insert(idx, stop);
    }

    /// Remove stop
    pub fn remove_stop(&mut self, index: usize) {
        if self.stops.len() > 2 && index < self.stops.len() {
            self.stops.remove(index);
        }
    }
}

/// Gradient editor widget
pub struct GradientEditor {
    /// Gradient being edited
    pub gradient: Gradient,
    /// Selected stop
    pub selected: Option<usize>,
    /// Widget bounds
    pub bounds: crate::widgets::Rect,
}

impl Default for GradientEditor {
    fn default() -> Self {
        Self {
            gradient: Gradient::default(),
            selected: None,
            bounds: crate::widgets::Rect { x: 0.0, y: 0.0, width: 300.0, height: 40.0 },
        }
    }
}

impl GradientEditor {
    /// Create new gradient editor
    #[must_use]
    pub fn new(gradient: Gradient) -> Self {
        Self {
            gradient,
            selected: None,
            bounds: crate::widgets::Rect { x: 0.0, y: 0.0, width: 300.0, height: 40.0 },
        }
    }

    /// Position to gradient t
    #[must_use]
    pub fn position_to_t(&self, x: f32) -> f32 {
        ((x - self.bounds.x) / self.bounds.width).clamp(0.0, 1.0)
    }

    /// Gradient t to position
    #[must_use]
    pub fn t_to_position(&self, t: f32) -> f32 {
        self.bounds.x + t * self.bounds.width
    }
}
