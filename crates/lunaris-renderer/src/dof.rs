//! Cinematic Depth of Field
//!
//! Hexagonal bokeh, anamorphic effects, and lens flares.

use glam::{Vec2, Vec3, Vec4};

/// DOF system
pub struct DepthOfField {
    pub enabled: bool,
    pub focus_distance: f32,
    pub aperture: f32,
    pub focal_length: f32,
    pub bokeh: BokehSettings,
    pub auto_focus: AutoFocus,
}

/// Bokeh settings
pub struct BokehSettings {
    pub shape: BokehShape,
    pub size: f32,
    pub threshold: f32,
    pub brightness: f32,
    pub rotation: f32,
    pub chromatic_aberration: f32,
}

/// Bokeh shape
pub enum BokehShape { Circle, Hexagon, Octagon, Custom(Vec<Vec2>) }

/// Auto focus
pub struct AutoFocus {
    pub enabled: bool,
    pub speed: f32,
    pub screen_position: Vec2,
    pub current_distance: f32,
}

impl Default for DepthOfField {
    fn default() -> Self {
        Self {
            enabled: true, focus_distance: 10.0, aperture: 2.8, focal_length: 50.0,
            bokeh: BokehSettings::default(), auto_focus: AutoFocus::default(),
        }
    }
}

impl Default for BokehSettings {
    fn default() -> Self {
        Self { shape: BokehShape::Hexagon, size: 1.0, threshold: 0.5, brightness: 1.0, rotation: 0.0, chromatic_aberration: 0.0 }
    }
}

impl Default for AutoFocus {
    fn default() -> Self {
        Self { enabled: false, speed: 5.0, screen_position: Vec2::new(0.5, 0.5), current_distance: 10.0 }
    }
}

impl DepthOfField {
    pub fn circle_of_confusion(&self, depth: f32) -> f32 {
        let f = self.focal_length / 1000.0;
        let n = self.aperture;
        let s = self.focus_distance;
        let d = depth;
        
        let coc = f.abs() * (s - d).abs() / (d * (s - f)) * (f / n);
        coc * 1000.0  // Convert to pixels
    }

    pub fn update_auto_focus(&mut self, dt: f32, depth_at_screen: f32) {
        if self.auto_focus.enabled {
            let target = depth_at_screen;
            self.auto_focus.current_distance += (target - self.auto_focus.current_distance) * self.auto_focus.speed * dt;
            self.focus_distance = self.auto_focus.current_distance;
        }
    }

    pub fn get_bokeh_kernel(&self, samples: u32) -> Vec<Vec2> {
        match &self.bokeh.shape {
            BokehShape::Circle => Self::circle_kernel(samples),
            BokehShape::Hexagon => Self::hexagon_kernel(samples),
            BokehShape::Octagon => Self::octagon_kernel(samples),
            BokehShape::Custom(points) => points.clone(),
        }
    }

    fn circle_kernel(samples: u32) -> Vec<Vec2> {
        (0..samples).map(|i| {
            let angle = (i as f32 / samples as f32) * std::f32::consts::TAU;
            let r = (i as f32 / samples as f32).sqrt();
            Vec2::new(angle.cos() * r, angle.sin() * r)
        }).collect()
    }

    fn hexagon_kernel(samples: u32) -> Vec<Vec2> {
        let mut points = Vec::new();
        for i in 0..samples {
            let angle = (i as f32 / samples as f32) * std::f32::consts::TAU;
            let sector = ((angle / (std::f32::consts::PI / 3.0)) as i32) % 6;
            let sector_angle = (sector as f32) * (std::f32::consts::PI / 3.0);
            let r = 1.0 / (sector_angle - angle + std::f32::consts::PI / 6.0).cos().max(0.001);
            let r = r.min(1.0) * (i as f32 / samples as f32).sqrt();
            points.push(Vec2::new(angle.cos() * r, angle.sin() * r));
        }
        points
    }

    fn octagon_kernel(samples: u32) -> Vec<Vec2> {
        let mut points = Vec::new();
        for i in 0..samples {
            let angle = (i as f32 / samples as f32) * std::f32::consts::TAU;
            let sector = ((angle / (std::f32::consts::PI / 4.0)) as i32) % 8;
            let sector_angle = (sector as f32) * (std::f32::consts::PI / 4.0);
            let r = 1.0 / (sector_angle - angle + std::f32::consts::PI / 8.0).cos().max(0.001);
            let r = r.min(1.0) * (i as f32 / samples as f32).sqrt();
            points.push(Vec2::new(angle.cos() * r, angle.sin() * r));
        }
        points
    }
}

/// Anamorphic effects
pub struct AnamorphicEffects {
    pub enabled: bool,
    pub stretch: f32,
    pub streak_intensity: f32,
    pub streak_length: f32,
    pub streak_threshold: f32,
    pub blue_tint: f32,
}

impl Default for AnamorphicEffects {
    fn default() -> Self {
        Self { enabled: false, stretch: 1.33, streak_intensity: 0.5, streak_length: 0.5, streak_threshold: 0.8, blue_tint: 0.2 }
    }
}

impl AnamorphicEffects {
    pub fn compute_streaks(&self, uv: Vec2, brightness_fn: impl Fn(Vec2) -> f32) -> f32 {
        let mut streak = 0.0;
        let samples = 32;
        
        for i in 0..samples {
            let offset = (i as f32 / samples as f32 - 0.5) * self.streak_length;
            let sample_uv = Vec2::new(uv.x + offset, uv.y);
            let brightness = brightness_fn(sample_uv);
            if brightness > self.streak_threshold {
                let dist = (1.0 - (i as f32 / samples as f32 - 0.5).abs() * 2.0).max(0.0);
                streak += (brightness - self.streak_threshold) * dist;
            }
        }
        
        streak / samples as f32 * self.streak_intensity
    }
}

/// Lens flare
pub struct LensFlare {
    pub enabled: bool,
    pub elements: Vec<FlareElement>,
    pub threshold: f32,
    pub intensity: f32,
    pub chromatic_distortion: f32,
}

/// Flare element
pub struct FlareElement {
    pub position: f32,  // 0 = screen center, 1 = light source, -1 = opposite
    pub size: f32,
    pub color: Vec3,
    pub element_type: FlareType,
}

/// Flare type  
pub enum FlareType { Circle, Ring, Hexagon, Starburst, Streak }

impl Default for LensFlare {
    fn default() -> Self {
        Self {
            enabled: true,
            elements: vec![
                FlareElement { position: 0.8, size: 0.02, color: Vec3::new(1.0, 0.8, 0.6), element_type: FlareType::Circle },
                FlareElement { position: 0.5, size: 0.05, color: Vec3::new(0.8, 0.6, 1.0), element_type: FlareType::Ring },
                FlareElement { position: -0.3, size: 0.08, color: Vec3::new(0.6, 0.8, 1.0), element_type: FlareType::Hexagon },
                FlareElement { position: -0.8, size: 0.1, color: Vec3::new(1.0, 0.6, 0.4), element_type: FlareType::Circle },
            ],
            threshold: 0.9,
            intensity: 1.0,
            chromatic_distortion: 0.02,
        }
    }
}

impl LensFlare {
    pub fn compute(&self, light_screen_pos: Vec2, light_brightness: f32) -> Vec<(Vec2, FlareElement)> {
        if light_brightness < self.threshold { return Vec::new(); }
        
        let center = Vec2::new(0.5, 0.5);
        let light_to_center = center - light_screen_pos;
        
        self.elements.iter().map(|element| {
            let pos = light_screen_pos + light_to_center * element.position;
            (pos, FlareElement { position: element.position, size: element.size, color: element.color * light_brightness * self.intensity, element_type: FlareType::Circle })
        }).collect()
    }
}
