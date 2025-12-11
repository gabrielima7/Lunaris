//! Strand-Based Hair Rendering
//!
//! RTX-style hair with millions of individual strands.

use glam::{Vec2, Vec3, Vec4, Mat4};

/// Strand hair system
pub struct StrandHair {
    pub grooms: Vec<Groom>,
    pub settings: HairRenderSettings,
}

/// Groom
pub struct Groom {
    pub name: String,
    pub strands: Vec<HairStrand>,
    pub material: HairMaterial,
    pub lod: HairLOD,
    pub physics: HairPhysicsSettings,
}

/// Hair strand
pub struct HairStrand {
    pub root_position: Vec3,
    pub root_normal: Vec3,
    pub control_points: Vec<Vec3>,
    pub thickness: f32,
    pub tip_thickness: f32,
    pub color: Vec4,
    pub id: u32,
}

/// Hair material
pub struct HairMaterial {
    pub base_color: Vec3,
    pub melanin: f32,
    pub melanin_redness: f32,
    pub roughness: f32,
    pub scatter: f32,
    pub ior: f32,
    pub specular_shift: f32,
    pub secondary_specular_shift: f32,
    pub specular_tint: Vec3,
    pub backlit: f32,
    pub ao: f32,
}

impl Default for HairMaterial {
    fn default() -> Self {
        Self {
            base_color: Vec3::new(0.1, 0.05, 0.02),
            melanin: 0.5,
            melanin_redness: 0.5,
            roughness: 0.2,
            scatter: 0.5,
            ior: 1.55,
            specular_shift: 0.1,
            secondary_specular_shift: -0.1,
            specular_tint: Vec3::ONE,
            backlit: 0.3,
            ao: 1.0,
        }
    }
}

/// Hair LOD
pub struct HairLOD {
    pub max_strands: u32,
    pub lod_distances: Vec<f32>,
    pub lod_strand_counts: Vec<u32>,
    pub current_lod: usize,
    pub width_scale_lod: Vec<f32>,
}

impl Default for HairLOD {
    fn default() -> Self {
        Self {
            max_strands: 100000,
            lod_distances: vec![5.0, 15.0, 30.0, 50.0],
            lod_strand_counts: vec![100000, 50000, 20000, 5000],
            current_lod: 0,
            width_scale_lod: vec![1.0, 1.2, 1.5, 2.0],
        }
    }
}

/// Hair physics settings
pub struct HairPhysicsSettings {
    pub enabled: bool,
    pub gravity: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub collision_radius: f32,
    pub wind_influence: f32,
}

impl Default for HairPhysicsSettings {
    fn default() -> Self {
        Self { enabled: true, gravity: 9.81, stiffness: 0.8, damping: 0.9, collision_radius: 0.001, wind_influence: 0.5 }
    }
}

/// Hair render settings
pub struct HairRenderSettings {
    pub render_mode: HairRenderMode,
    pub shadow_mode: HairShadowMode,
    pub aa_mode: HairAAMode,
    pub depth_prepass: bool,
    pub transmittance_samples: u32,
}

/// Render mode
pub enum HairRenderMode { Strands, Cards, Both }

/// Shadow mode
pub enum HairShadowMode { Opaque, DeepShadowMap, RayTraced }

/// AA mode
pub enum HairAAMode { None, MSAA, TAA, HairAA }

impl Default for HairRenderSettings {
    fn default() -> Self {
        Self { render_mode: HairRenderMode::Strands, shadow_mode: HairShadowMode::DeepShadowMap, aa_mode: HairAAMode::TAA, depth_prepass: true, transmittance_samples: 8 }
    }
}

impl StrandHair {
    pub fn new() -> Self {
        Self { grooms: Vec::new(), settings: HairRenderSettings::default() }
    }

    pub fn create_groom(&mut self, name: &str) -> usize {
        let idx = self.grooms.len();
        self.grooms.push(Groom { name: name.into(), strands: Vec::new(), material: HairMaterial::default(), lod: HairLOD::default(), physics: HairPhysicsSettings::default() });
        idx
    }

    pub fn add_strand(&mut self, groom_idx: usize, root: Vec3, normal: Vec3, points: Vec<Vec3>, color: Vec4) {
        if let Some(groom) = self.grooms.get_mut(groom_idx) {
            let id = groom.strands.len() as u32;
            groom.strands.push(HairStrand { root_position: root, root_normal: normal, control_points: points, thickness: 0.0005, tip_thickness: 0.0001, color, id });
        }
    }

    pub fn update_lod(&mut self, camera_position: Vec3) {
        for groom in &mut self.grooms {
            let center = groom.strands.first().map(|s| s.root_position).unwrap_or(Vec3::ZERO);
            let dist = (camera_position - center).length();
            
            groom.lod.current_lod = groom.lod.lod_distances.iter().position(|d| dist < *d).unwrap_or(groom.lod.lod_distances.len());
        }
    }

    pub fn get_visible_strands(&self, groom_idx: usize) -> &[HairStrand] {
        if let Some(groom) = self.grooms.get(groom_idx) {
            let count = groom.lod.lod_strand_counts.get(groom.lod.current_lod).copied().unwrap_or(groom.strands.len() as u32);
            &groom.strands[..count.min(groom.strands.len() as u32) as usize]
        } else { &[] }
    }
}

/// Marschner hair shading model
pub struct MarschnerModel;

impl MarschnerModel {
    pub fn evaluate(
        light_dir: Vec3, view_dir: Vec3, tangent: Vec3,
        material: &HairMaterial,
    ) -> Vec3 {
        let h = (light_dir + view_dir).normalize();
        let sin_tl = tangent.cross(light_dir).length();
        let sin_tv = tangent.cross(view_dir).length();
        let cos_tl = tangent.dot(light_dir).abs();
        let cos_tv = tangent.dot(view_dir).abs();
        
        let sin_th = sin_tl * cos_tv + cos_tl * sin_tv;
        let cos_th_sq = 1.0 - sin_th * sin_th;
        
        // R (primary specular)
        let shift_r = material.specular_shift;
        let roughness_r = material.roughness;
        let r = Self::gaussian(sin_th - shift_r, roughness_r);
        
        // TRT (secondary specular)
        let shift_trt = material.secondary_specular_shift;
        let roughness_trt = material.roughness * 2.0;
        let trt = Self::gaussian(sin_th - shift_trt, roughness_trt) * material.scatter;
        
        // TT (transmission)
        let tt = cos_th_sq.sqrt() * material.backlit;
        
        let spec = material.specular_tint * (r + trt);
        let diff = material.base_color * (1.0 - material.melanin);
        
        diff + spec + material.base_color * tt
    }

    fn gaussian(x: f32, width: f32) -> f32 {
        let a = x / width;
        (-a * a * 0.5).exp()
    }
}

/// Deep shadow map for hair
pub struct DeepShadowMap {
    pub resolution: u32,
    pub layers: u32,
    pub data: Vec<DeepShadowLayer>,
}

/// Deep shadow layer
pub struct DeepShadowLayer {
    pub depth: f32,
    pub transmittance: f32,
}

impl DeepShadowMap {
    pub fn new(resolution: u32, layers: u32) -> Self {
        Self { resolution, layers, data: vec![DeepShadowLayer { depth: 0.0, transmittance: 1.0 }; (resolution * resolution * layers) as usize] }
    }
}
