//! MetaHuman-like Digital Human System
//!
//! Photorealistic digital human rendering and animation.

use glam::{Vec3, Vec4, Mat4, Quat};
use std::collections::HashMap;

/// DNA asset configuration
#[derive(Debug, Clone)]
pub struct DNAConfig {
    /// LOD levels
    pub lod_count: u8,
    /// Joint count
    pub joint_count: u32,
    /// Blend shape count
    pub blend_shape_count: u32,
    /// Animation rig compatibility
    pub rig_version: String,
    /// Texture resolution
    pub texture_resolution: u32,
}

/// Facial rig with control bones
#[derive(Debug, Clone)]
pub struct FacialRig {
    /// Control bones
    pub controls: Vec<FacialControl>,
    /// Corrective blend shapes
    pub correctives: Vec<CorrectiveBlendShape>,
    /// FACS mappings
    pub facs_mapping: HashMap<String, Vec<FACSMapping>>,
}

/// Facial control bone
#[derive(Debug, Clone)]
pub struct FacialControl {
    /// Control name
    pub name: String,
    /// Position
    pub position: Vec3,
    /// Rotation
    pub rotation: Quat,
    /// Min value
    pub min: f32,
    /// Max value
    pub max: f32,
    /// Current value
    pub value: f32,
    /// Driven blend shapes
    pub driven_shapes: Vec<(String, f32)>,
}

/// FACS action unit mapping
#[derive(Debug, Clone)]
pub struct FACSMapping {
    /// Action unit index
    pub au_index: u8,
    /// Weight
    pub weight: f32,
    /// Left/right side
    pub side: Side,
}

/// Face side
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Center,
    Left,
    Right,
}

/// Corrective blend shape
#[derive(Debug, Clone)]
pub struct CorrectiveBlendShape {
    /// Shape name
    pub name: String,
    /// Triggers (shape combinations)
    pub triggers: Vec<(String, String, f32)>,
    /// Weight
    pub weight: f32,
}

/// Skin shader parameters
#[derive(Debug, Clone)]
pub struct SkinShader {
    /// Albedo map
    pub albedo: Option<String>,
    /// Normal map
    pub normal: Option<String>,
    /// Roughness map
    pub roughness: Option<String>,
    /// Cavity map
    pub cavity: Option<String>,
    /// Subsurface color
    pub subsurface_color: Vec3,
    /// Subsurface radius (RGB)
    pub subsurface_radius: Vec3,
    /// Specular
    pub specular: f32,
    /// Melanin
    pub melanin: f32,
    /// Melanin redness
    pub melanin_redness: f32,
    /// Micro normal scale
    pub micro_normal_scale: f32,
    /// Pore map
    pub pore_map: Option<String>,
    /// Pore scale
    pub pore_scale: f32,
}

impl Default for SkinShader {
    fn default() -> Self {
        Self {
            albedo: None,
            normal: None,
            roughness: None,
            cavity: None,
            subsurface_color: Vec3::new(0.8, 0.23, 0.15),
            subsurface_radius: Vec3::new(1.0, 0.4, 0.2),
            specular: 0.5,
            melanin: 0.5,
            melanin_redness: 0.5,
            micro_normal_scale: 0.5,
            pore_map: None,
            pore_scale: 4.0,
        }
    }
}

/// Eye shader parameters
#[derive(Debug, Clone)]
pub struct EyeShader {
    /// Iris texture
    pub iris_texture: Option<String>,
    /// Iris color
    pub iris_color: Vec3,
    /// Iris size
    pub iris_size: f32,
    /// Pupil size
    pub pupil_size: f32,
    /// Limbal ring color
    pub limbal_ring_color: Vec3,
    /// Limbal ring intensity
    pub limbal_ring_intensity: f32,
    /// Sclera color
    pub sclera_color: Vec3,
    /// Eye wetness
    pub wetness: f32,
    /// Refraction index
    pub ior: f32,
    /// Caustics intensity
    pub caustics: f32,
}

impl Default for EyeShader {
    fn default() -> Self {
        Self {
            iris_texture: None,
            iris_color: Vec3::new(0.3, 0.5, 0.7),
            iris_size: 0.5,
            pupil_size: 0.2,
            limbal_ring_color: Vec3::new(0.1, 0.1, 0.1),
            limbal_ring_intensity: 0.5,
            sclera_color: Vec3::new(0.95, 0.93, 0.9),
            wetness: 0.7,
            ior: 1.4,
            caustics: 0.3,
        }
    }
}

/// Hair shader parameters
#[derive(Debug, Clone)]
pub struct HairShader {
    /// Base color
    pub base_color: Vec3,
    /// Melanin
    pub melanin: f32,
    /// Melanin redness
    pub melanin_redness: f32,
    /// Roughness
    pub roughness: f32,
    /// Scatter
    pub scatter: f32,
    /// Tangent map
    pub tangent_map: Option<String>,
    /// Alpha mask
    pub alpha_mask: Option<String>,
    /// Backlight
    pub backlight: f32,
    /// Specular shift
    pub specular_shift: f32,
}

impl Default for HairShader {
    fn default() -> Self {
        Self {
            base_color: Vec3::new(0.1, 0.05, 0.02),
            melanin: 0.7,
            melanin_redness: 0.3,
            roughness: 0.5,
            scatter: 0.5,
            tangent_map: None,
            alpha_mask: None,
            backlight: 0.5,
            specular_shift: 0.04,
        }
    }
}

/// Hair strand (for groom)
#[derive(Debug, Clone)]
pub struct HairStrand {
    /// Control points
    pub points: Vec<Vec3>,
    /// Widths at each point
    pub widths: Vec<f32>,
    /// UVs
    pub uvs: Vec<[f32; 2]>,
    /// Root UV
    pub root_uv: [f32; 2],
}

/// Groom asset
#[derive(Debug, Clone)]
pub struct GroomAsset {
    /// Name
    pub name: String,
    /// Strands
    pub strands: Vec<HairStrand>,
    /// LOD levels
    pub lod_levels: u8,
    /// Simulation enabled
    pub simulate: bool,
    /// Physics parameters
    pub physics: HairPhysics,
}

/// Hair physics parameters
#[derive(Debug, Clone)]
pub struct HairPhysics {
    /// Stiffness
    pub stiffness: f32,
    /// Damping
    pub damping: f32,
    /// Gravity scale
    pub gravity_scale: f32,
    /// Length constraint
    pub length_constraint: f32,
    /// Collision radius
    pub collision_radius: f32,
    /// Wind response
    pub wind_response: f32,
}

impl Default for HairPhysics {
    fn default() -> Self {
        Self {
            stiffness: 0.8,
            damping: 0.1,
            gravity_scale: 1.0,
            length_constraint: 0.9,
            collision_radius: 0.02,
            wind_response: 0.5,
        }
    }
}

/// Digital human instance
pub struct DigitalHuman {
    /// ID
    pub id: u64,
    /// Name
    pub name: String,
    /// DNA configuration
    pub dna: DNAConfig,
    /// Facial rig
    pub facial_rig: FacialRig,
    /// Skin shader
    pub skin: SkinShader,
    /// Eye shader
    pub eyes: EyeShader,
    /// Hair shader
    pub hair: HairShader,
    /// Groom assets
    pub grooms: Vec<GroomAsset>,
    /// Current LOD
    pub current_lod: u8,
    /// Blend shape weights
    pub blend_shape_weights: HashMap<String, f32>,
    /// Is speaking
    pub is_speaking: bool,
    /// Current expression
    pub expression: String,
}

impl DigitalHuman {
    /// Create new digital human
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
            dna: DNAConfig {
                lod_count: 8,
                joint_count: 256,
                blend_shape_count: 700,
                rig_version: "4.26.2".to_string(),
                texture_resolution: 4096,
            },
            facial_rig: FacialRig {
                controls: Vec::new(),
                correctives: Vec::new(),
                facs_mapping: HashMap::new(),
            },
            skin: SkinShader::default(),
            eyes: EyeShader::default(),
            hair: HairShader::default(),
            grooms: Vec::new(),
            current_lod: 0,
            blend_shape_weights: HashMap::new(),
            is_speaking: false,
            expression: "neutral".to_string(),
        }
    }

    /// Set blend shape weight
    pub fn set_blend_shape(&mut self, name: &str, weight: f32) {
        self.blend_shape_weights.insert(name.to_string(), weight.clamp(0.0, 1.0));
    }

    /// Set expression preset
    pub fn set_expression(&mut self, expression: &str, intensity: f32) {
        self.expression = expression.to_string();
        
        // Apply preset blend shapes
        match expression {
            "happy" => {
                self.set_blend_shape("jawOpen", 0.1 * intensity);
                self.set_blend_shape("mouthSmileLeft", intensity);
                self.set_blend_shape("mouthSmileRight", intensity);
                self.set_blend_shape("cheekSquintLeft", 0.5 * intensity);
                self.set_blend_shape("cheekSquintRight", 0.5 * intensity);
            }
            "sad" => {
                self.set_blend_shape("browInnerUp", 0.6 * intensity);
                self.set_blend_shape("mouthFrownLeft", intensity);
                self.set_blend_shape("mouthFrownRight", intensity);
            }
            "angry" => {
                self.set_blend_shape("browDownLeft", intensity);
                self.set_blend_shape("browDownRight", intensity);
                self.set_blend_shape("eyeSquintLeft", 0.5 * intensity);
                self.set_blend_shape("eyeSquintRight", 0.5 * intensity);
                self.set_blend_shape("jawForward", 0.3 * intensity);
            }
            "surprised" => {
                self.set_blend_shape("browInnerUp", intensity);
                self.set_blend_shape("browOuterUpLeft", intensity);
                self.set_blend_shape("browOuterUpRight", intensity);
                self.set_blend_shape("eyeWideLeft", intensity);
                self.set_blend_shape("eyeWideRight", intensity);
                self.set_blend_shape("jawOpen", 0.4 * intensity);
            }
            _ => {
                // Reset to neutral
                self.blend_shape_weights.clear();
            }
        }
    }

    /// Apply viseme for speech
    pub fn apply_viseme(&mut self, viseme: &str, weight: f32) {
        // Reset mouth shapes
        for key in self.blend_shape_weights.clone().keys() {
            if key.starts_with("mouth") || key.starts_with("jaw") {
                self.blend_shape_weights.remove(key);
            }
        }

        match viseme {
            "PP" | "BB" | "MM" => {
                self.set_blend_shape("mouthPucker", weight);
                self.set_blend_shape("mouthClose", 0.8 * weight);
            }
            "AA" => {
                self.set_blend_shape("jawOpen", 0.6 * weight);
                self.set_blend_shape("mouthFunnel", 0.2 * weight);
            }
            "EE" | "IH" => {
                self.set_blend_shape("mouthSmileLeft", 0.3 * weight);
                self.set_blend_shape("mouthSmileRight", 0.3 * weight);
            }
            "OH" => {
                self.set_blend_shape("jawOpen", 0.4 * weight);
                self.set_blend_shape("mouthFunnel", 0.5 * weight);
            }
            "OO" => {
                self.set_blend_shape("mouthPucker", 0.6 * weight);
                self.set_blend_shape("mouthFunnel", 0.4 * weight);
            }
            _ => {}
        }
    }

    /// Update LOD based on distance
    pub fn update_lod(&mut self, distance: f32) {
        self.current_lod = if distance < 2.0 {
            0
        } else if distance < 5.0 {
            1
        } else if distance < 10.0 {
            2
        } else if distance < 20.0 {
            3
        } else if distance < 40.0 {
            4
        } else if distance < 80.0 {
            5
        } else if distance < 150.0 {
            6
        } else {
            7
        };
    }
}

/// LiveLink face capture
#[derive(Debug, Clone)]
pub struct LiveLinkFace {
    /// Subject name
    pub subject: String,
    /// Is connected
    pub connected: bool,
    /// Blend shape data
    pub blend_shapes: HashMap<String, f32>,
    /// Head rotation
    pub head_rotation: Quat,
    /// Head position
    pub head_position: Vec3,
    /// Left eye rotation
    pub left_eye_rotation: Quat,
    /// Right eye rotation
    pub right_eye_rotation: Quat,
}

impl Default for LiveLinkFace {
    fn default() -> Self {
        Self {
            subject: "iPhone".to_string(),
            connected: false,
            blend_shapes: HashMap::new(),
            head_rotation: Quat::IDENTITY,
            head_position: Vec3::ZERO,
            left_eye_rotation: Quat::IDENTITY,
            right_eye_rotation: Quat::IDENTITY,
        }
    }
}

impl LiveLinkFace {
    /// Apply to digital human
    pub fn apply_to(&self, human: &mut DigitalHuman) {
        for (name, weight) in &self.blend_shapes {
            human.set_blend_shape(name, *weight);
        }
    }
}
