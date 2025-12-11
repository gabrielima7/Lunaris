//! Foliage Wind System
//!
//! Wind simulation, grass/tree sway, and wind zones.

use glam::{Vec2, Vec3, Vec4};
use std::f32::consts::PI;

/// Foliage wind system
pub struct FoliageWind {
    pub global_wind: GlobalWind,
    pub zones: Vec<WindZone>,
    pub settings: WindSettings,
    pub time: f32,
}

/// Global wind
pub struct GlobalWind {
    pub direction: Vec3,
    pub speed: f32,
    pub turbulence: f32,
    pub gust_frequency: f32,
    pub gust_strength: f32,
}

impl Default for GlobalWind {
    fn default() -> Self {
        Self { direction: Vec3::new(1.0, 0.0, 0.3).normalize(), speed: 5.0, turbulence: 0.3, gust_frequency: 0.5, gust_strength: 0.5 }
    }
}

/// Wind zone
pub struct WindZone {
    pub id: usize,
    pub zone_type: ZoneType,
    pub position: Vec3,
    pub radius: f32,
    pub falloff: f32,
    pub strength: f32,
    pub pulse_frequency: f32,
}

/// Zone type
pub enum ZoneType {
    Directional { direction: Vec3 },
    Point { outward: bool },
    Vortex { axis: Vec3 },
}

/// Wind settings
pub struct WindSettings {
    pub grass_amplitude: f32,
    pub grass_frequency: f32,
    pub tree_amplitude: f32,
    pub tree_frequency: f32,
    pub branch_influence: f32,
    pub leaf_flutter: f32,
}

impl Default for WindSettings {
    fn default() -> Self {
        Self { grass_amplitude: 0.3, grass_frequency: 2.0, tree_amplitude: 0.1, tree_frequency: 0.5, branch_influence: 0.5, leaf_flutter: 1.0 }
    }
}

impl FoliageWind {
    pub fn new() -> Self {
        Self { global_wind: GlobalWind::default(), zones: Vec::new(), settings: WindSettings::default(), time: 0.0 }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
    }

    pub fn get_wind_at(&self, position: Vec3) -> Vec3 {
        let mut wind = self.get_global_wind();
        
        for zone in &self.zones {
            let zone_wind = self.sample_zone(zone, position);
            wind += zone_wind;
        }
        
        wind
    }

    fn get_global_wind(&self) -> Vec3 {
        let base = self.global_wind.direction * self.global_wind.speed;
        
        // Turbulence
        let turbulence = Vec3::new(
            noise(self.time * 0.5, 0.0, 0.0) - 0.5,
            noise(0.0, self.time * 0.5, 0.0) - 0.5,
            noise(0.0, 0.0, self.time * 0.5) - 0.5,
        ) * self.global_wind.turbulence * self.global_wind.speed;
        
        // Gusts
        let gust = (self.time * self.global_wind.gust_frequency).sin().max(0.0).powi(3) * self.global_wind.gust_strength;
        
        base * (1.0 + gust) + turbulence
    }

    fn sample_zone(&self, zone: &WindZone, position: Vec3) -> Vec3 {
        let to_pos = position - zone.position;
        let dist = to_pos.length();
        
        if dist > zone.radius { return Vec3::ZERO; }
        
        let falloff = (1.0 - dist / zone.radius).powf(zone.falloff);
        let pulse = (self.time * zone.pulse_frequency * PI * 2.0).sin() * 0.5 + 0.5;
        
        let dir = match &zone.zone_type {
            ZoneType::Directional { direction } => *direction,
            ZoneType::Point { outward } => if *outward { to_pos.normalize() } else { -to_pos.normalize() },
            ZoneType::Vortex { axis } => axis.cross(to_pos.normalize()).normalize(),
        };
        
        dir * zone.strength * falloff * (0.5 + pulse * 0.5)
    }

    pub fn get_grass_offset(&self, position: Vec3, vertex_height: f32) -> Vec3 {
        let wind = self.get_wind_at(position);
        let phase = position.x * 0.1 + position.z * 0.13 + self.time * self.settings.grass_frequency;
        let sway = phase.sin() * self.settings.grass_amplitude * vertex_height;
        
        Vec3::new(wind.x * sway, 0.0, wind.z * sway)
    }

    pub fn get_tree_params(&self, position: Vec3, vertex_height: f32, branch_phase: f32) -> TreeWindParams {
        let wind = self.get_wind_at(position);
        let wind_strength = wind.length();
        
        // Main trunk sway
        let trunk_phase = position.x * 0.05 + self.time * self.settings.tree_frequency;
        let trunk_sway = trunk_phase.sin() * self.settings.tree_amplitude * vertex_height * wind_strength;
        
        // Branch movement
        let branch_frequency = self.settings.tree_frequency * 2.0;
        let branch_sway = (branch_phase + self.time * branch_frequency).sin() * self.settings.branch_influence * wind_strength;
        
        // Leaf flutter
        let leaf_phase = position.dot(Vec3::ONE) * 10.0 + self.time * 10.0;
        let flutter = leaf_phase.sin() * self.settings.leaf_flutter * wind_strength * 0.1;
        
        TreeWindParams { trunk_offset: Vec3::new(wind.x.signum() * trunk_sway, 0.0, wind.z.signum() * trunk_sway), branch_offset: branch_sway, leaf_flutter: flutter }
    }

    pub fn add_zone(&mut self, position: Vec3, zone_type: ZoneType, radius: f32, strength: f32) {
        let id = self.zones.len();
        self.zones.push(WindZone { id, zone_type, position, radius, falloff: 1.0, strength, pulse_frequency: 0.0 });
    }
}

/// Tree wind parameters
pub struct TreeWindParams {
    pub trunk_offset: Vec3,
    pub branch_offset: f32,
    pub leaf_flutter: f32,
}

/// Wind-affected foliage instance
pub struct WindFoliageInstance {
    pub position: Vec3,
    pub foliage_type: FoliageType,
    pub phase_offset: f32,
    pub stiffness: f32,
    pub height: f32,
}

/// Foliage type
pub enum FoliageType { Grass, Bush, SmallTree, LargeTree }

impl WindFoliageInstance {
    pub fn get_offset(&self, wind_system: &FoliageWind) -> Vec3 {
        let base_offset = match self.foliage_type {
            FoliageType::Grass => wind_system.get_grass_offset(self.position, self.height),
            FoliageType::Bush => wind_system.get_grass_offset(self.position, self.height) * 0.7,
            FoliageType::SmallTree | FoliageType::LargeTree => {
                let params = wind_system.get_tree_params(self.position, self.height, self.phase_offset);
                params.trunk_offset
            }
        };
        
        base_offset / self.stiffness
    }
}

/// Shader parameters for wind
pub struct WindShaderParams {
    pub time: f32,
    pub direction: Vec4,  // xyz = direction, w = strength
    pub frequency: Vec4,  // x = grass, y = tree, z = branch, w = leaf
    pub amplitude: Vec4,  // x = grass, y = tree, z = branch, w = leaf
    pub gust: Vec4,       // x = frequency, y = strength, z = phase, w = unused
}

impl FoliageWind {
    pub fn get_shader_params(&self) -> WindShaderParams {
        let wind = self.get_global_wind();
        WindShaderParams {
            time: self.time,
            direction: Vec4::new(wind.x, wind.y, wind.z, wind.length()),
            frequency: Vec4::new(self.settings.grass_frequency, self.settings.tree_frequency, self.settings.tree_frequency * 2.0, 10.0),
            amplitude: Vec4::new(self.settings.grass_amplitude, self.settings.tree_amplitude, self.settings.branch_influence, self.settings.leaf_flutter),
            gust: Vec4::new(self.global_wind.gust_frequency, self.global_wind.gust_strength, (self.time * self.global_wind.gust_frequency).sin(), 0.0),
        }
    }
}

fn noise(x: f32, y: f32, z: f32) -> f32 {
    ((x * 12.9898 + y * 78.233 + z * 45.164).sin() * 43758.5453).fract()
}
