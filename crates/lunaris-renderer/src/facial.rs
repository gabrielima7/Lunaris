//! Facial Animation System
//!
//! Blend shapes, morph targets, and procedural facial animation.

use glam::Vec3;
use std::collections::HashMap;

/// Blend shape target
#[derive(Debug, Clone)]
pub struct BlendShape {
    /// Name (e.g., "smile", "blink_L")
    pub name: String,
    /// Vertex deltas
    pub deltas: Vec<Vec3>,
    /// Normal deltas (optional)
    pub normal_deltas: Option<Vec<Vec3>>,
    /// Current weight (0-1)
    pub weight: f32,
}

impl BlendShape {
    /// Create a new blend shape
    #[must_use]
    pub fn new(name: &str, deltas: Vec<Vec3>) -> Self {
        Self {
            name: name.to_string(),
            deltas,
            normal_deltas: None,
            weight: 0.0,
        }
    }
}

/// Standard FACS-based blend shapes (Apple ARKit compatible)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FacialBlendShape {
    // Eye shapes
    EyeBlinkLeft,
    EyeBlinkRight,
    EyeLookDownLeft,
    EyeLookDownRight,
    EyeLookInLeft,
    EyeLookInRight,
    EyeLookOutLeft,
    EyeLookOutRight,
    EyeLookUpLeft,
    EyeLookUpRight,
    EyeSquintLeft,
    EyeSquintRight,
    EyeWideLeft,
    EyeWideRight,
    
    // Jaw and mouth
    JawForward,
    JawLeft,
    JawRight,
    JawOpen,
    MouthClose,
    MouthFunnel,
    MouthPucker,
    MouthLeft,
    MouthRight,
    MouthSmileLeft,
    MouthSmileRight,
    MouthFrownLeft,
    MouthFrownRight,
    MouthDimpleLeft,
    MouthDimpleRight,
    MouthStretchLeft,
    MouthStretchRight,
    MouthRollLower,
    MouthRollUpper,
    MouthShrugLower,
    MouthShrugUpper,
    MouthPressLeft,
    MouthPressRight,
    MouthLowerDownLeft,
    MouthLowerDownRight,
    MouthUpperUpLeft,
    MouthUpperUpRight,
    
    // Brow
    BrowDownLeft,
    BrowDownRight,
    BrowInnerUp,
    BrowOuterUpLeft,
    BrowOuterUpRight,
    
    // Cheek
    CheekPuff,
    CheekSquintLeft,
    CheekSquintRight,
    
    // Nose
    NoseSneerLeft,
    NoseSneerRight,
    
    // Tongue
    TongueOut,
}

/// Facial expression preset
#[derive(Debug, Clone)]
pub struct FacialExpression {
    /// Name
    pub name: String,
    /// Blend shape weights
    pub weights: HashMap<FacialBlendShape, f32>,
    /// Intensity multiplier
    pub intensity: f32,
}

impl FacialExpression {
    /// Create a new expression
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            weights: HashMap::new(),
            intensity: 1.0,
        }
    }

    /// Set a blend shape weight
    pub fn set(&mut self, shape: FacialBlendShape, weight: f32) -> &mut Self {
        self.weights.insert(shape, weight);
        self
    }

    /// Create happy expression preset
    #[must_use]
    pub fn happy() -> Self {
        let mut expr = Self::new("Happy");
        expr.set(FacialBlendShape::MouthSmileLeft, 0.8);
        expr.set(FacialBlendShape::MouthSmileRight, 0.8);
        expr.set(FacialBlendShape::CheekSquintLeft, 0.3);
        expr.set(FacialBlendShape::CheekSquintRight, 0.3);
        expr.set(FacialBlendShape::EyeSquintLeft, 0.2);
        expr.set(FacialBlendShape::EyeSquintRight, 0.2);
        expr
    }

    /// Create sad expression preset
    #[must_use]
    pub fn sad() -> Self {
        let mut expr = Self::new("Sad");
        expr.set(FacialBlendShape::MouthFrownLeft, 0.7);
        expr.set(FacialBlendShape::MouthFrownRight, 0.7);
        expr.set(FacialBlendShape::BrowInnerUp, 0.6);
        expr.set(FacialBlendShape::BrowDownLeft, 0.3);
        expr.set(FacialBlendShape::BrowDownRight, 0.3);
        expr
    }

    /// Create angry expression preset
    #[must_use]
    pub fn angry() -> Self {
        let mut expr = Self::new("Angry");
        expr.set(FacialBlendShape::BrowDownLeft, 0.8);
        expr.set(FacialBlendShape::BrowDownRight, 0.8);
        expr.set(FacialBlendShape::JawOpen, 0.3);
        expr.set(FacialBlendShape::MouthFrownLeft, 0.4);
        expr.set(FacialBlendShape::MouthFrownRight, 0.4);
        expr.set(FacialBlendShape::NoseSneerLeft, 0.5);
        expr.set(FacialBlendShape::NoseSneerRight, 0.5);
        expr
    }

    /// Create surprised expression preset
    #[must_use]
    pub fn surprised() -> Self {
        let mut expr = Self::new("Surprised");
        expr.set(FacialBlendShape::BrowInnerUp, 0.9);
        expr.set(FacialBlendShape::BrowOuterUpLeft, 0.7);
        expr.set(FacialBlendShape::BrowOuterUpRight, 0.7);
        expr.set(FacialBlendShape::EyeWideLeft, 0.8);
        expr.set(FacialBlendShape::EyeWideRight, 0.8);
        expr.set(FacialBlendShape::JawOpen, 0.5);
        expr
    }
}

/// Viseme for lip sync
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Viseme {
    /// Silence / neutral
    Silence,
    /// AA (as in "father")
    AA,
    /// AE (as in "cat")
    AE,
    /// AH (as in "but")
    AH,
    /// AO (as in "thought")
    AO,
    /// AW (as in "cow")
    AW,
    /// AY (as in "bite")
    AY,
    /// B, M, P
    BMP,
    /// CH, J, SH
    CHJSH,
    /// D, T, N
    DTN,
    /// EH (as in "bed")
    EH,
    /// ER (as in "bird")
    ER,
    /// EY (as in "say")
    EY,
    /// F, V
    FV,
    /// G, K, NG
    GKNG,
    /// IH (as in "sit")
    IH,
    /// IY (as in "see")
    IY,
    /// L
    L,
    /// OW (as in "go")
    OW,
    /// OY (as in "boy")
    OY,
    /// R
    R,
    /// S, Z
    SZ,
    /// TH
    TH,
    /// UH (as in "book")
    UH,
    /// UW (as in "blue")
    UW,
    /// W
    W,
}

/// Lip sync controller
pub struct LipSync {
    /// Current viseme
    pub current_viseme: Viseme,
    /// Viseme weight
    pub viseme_weight: f32,
    /// Blend time
    pub blend_time: f32,
    /// Viseme to blend shape mapping
    viseme_mapping: HashMap<Viseme, HashMap<FacialBlendShape, f32>>,
    /// Current blend target
    target_weights: HashMap<FacialBlendShape, f32>,
    /// Current weights
    current_weights: HashMap<FacialBlendShape, f32>,
}

impl Default for LipSync {
    fn default() -> Self {
        Self::new()
    }
}

impl LipSync {
    /// Create a new lip sync controller
    #[must_use]
    pub fn new() -> Self {
        let mut mapping = HashMap::new();
        
        // Map visemes to blend shapes
        let mut silence = HashMap::new();
        silence.insert(FacialBlendShape::MouthClose, 1.0);
        mapping.insert(Viseme::Silence, silence);
        
        let mut aa = HashMap::new();
        aa.insert(FacialBlendShape::JawOpen, 0.7);
        aa.insert(FacialBlendShape::MouthFunnel, 0.3);
        mapping.insert(Viseme::AA, aa);
        
        let mut bmp = HashMap::new();
        bmp.insert(FacialBlendShape::MouthClose, 0.9);
        bmp.insert(FacialBlendShape::MouthPressLeft, 0.5);
        bmp.insert(FacialBlendShape::MouthPressRight, 0.5);
        mapping.insert(Viseme::BMP, bmp);
        
        let mut fv = HashMap::new();
        fv.insert(FacialBlendShape::MouthFunnel, 0.4);
        fv.insert(FacialBlendShape::MouthLowerDownLeft, 0.3);
        fv.insert(FacialBlendShape::MouthLowerDownRight, 0.3);
        mapping.insert(Viseme::FV, fv);
        
        let mut iy = HashMap::new();
        iy.insert(FacialBlendShape::MouthSmileLeft, 0.5);
        iy.insert(FacialBlendShape::MouthSmileRight, 0.5);
        mapping.insert(Viseme::IY, iy);
        
        let mut ow = HashMap::new();
        ow.insert(FacialBlendShape::MouthPucker, 0.7);
        ow.insert(FacialBlendShape::JawOpen, 0.3);
        mapping.insert(Viseme::OW, ow);
        
        Self {
            current_viseme: Viseme::Silence,
            viseme_weight: 0.0,
            blend_time: 0.05,
            viseme_mapping: mapping,
            target_weights: HashMap::new(),
            current_weights: HashMap::new(),
        }
    }

    /// Set target viseme
    pub fn set_viseme(&mut self, viseme: Viseme, weight: f32) {
        self.current_viseme = viseme;
        self.viseme_weight = weight;
        
        // Update target weights
        self.target_weights.clear();
        if let Some(mapping) = self.viseme_mapping.get(&viseme) {
            for (shape, w) in mapping {
                self.target_weights.insert(*shape, w * weight);
            }
        }
    }

    /// Update blend
    pub fn update(&mut self, delta_time: f32) {
        let blend_factor = (delta_time / self.blend_time).clamp(0.0, 1.0);
        
        // Blend current weights towards target
        for (shape, target) in &self.target_weights {
            let current = self.current_weights.entry(*shape).or_insert(0.0);
            *current += (*target - *current) * blend_factor;
        }
        
        // Decay weights not in target
        let targets: Vec<_> = self.target_weights.keys().copied().collect();
        for (shape, weight) in self.current_weights.iter_mut() {
            if !targets.contains(shape) {
                *weight *= 1.0 - blend_factor;
            }
        }
    }

    /// Get current blend shape weights
    #[must_use]
    pub fn get_weights(&self) -> &HashMap<FacialBlendShape, f32> {
        &self.current_weights
    }
}

/// Procedural eye animation
pub struct ProceduralEyes {
    /// Blink rate (blinks per minute)
    pub blink_rate: f32,
    /// Blink duration (seconds)
    pub blink_duration: f32,
    /// Look target (world space)
    pub look_target: Option<Vec3>,
    /// Eye movement speed
    pub eye_speed: f32,
    /// Current blink progress (0-1)
    blink_progress: f32,
    /// Time until next blink
    next_blink: f32,
    /// Eye rotation (horizontal, vertical)
    eye_rotation: (f32, f32),
}

impl Default for ProceduralEyes {
    fn default() -> Self {
        Self {
            blink_rate: 15.0,
            blink_duration: 0.15,
            look_target: None,
            eye_speed: 5.0,
            blink_progress: 1.0,
            next_blink: 2.0,
            eye_rotation: (0.0, 0.0),
        }
    }
}

impl ProceduralEyes {
    /// Update eye animation
    pub fn update(&mut self, delta_time: f32, head_position: Vec3, head_forward: Vec3) {
        // Update blink
        self.next_blink -= delta_time;
        if self.next_blink <= 0.0 {
            self.blink_progress = 0.0;
            let avg_interval = 60.0 / self.blink_rate;
            self.next_blink = avg_interval * (0.5 + self.pseudo_random() * 1.0);
        }
        
        if self.blink_progress < 1.0 {
            self.blink_progress += delta_time / self.blink_duration;
        }
        
        // Update look direction
        if let Some(target) = self.look_target {
            let to_target = (target - head_position).normalize();
            let horizontal = head_forward.cross(Vec3::Y).dot(to_target).atan2(head_forward.dot(to_target));
            let vertical = to_target.y.asin();
            
            let target_h = horizontal.clamp(-0.5, 0.5);
            let target_v = vertical.clamp(-0.3, 0.3);
            
            self.eye_rotation.0 += (target_h - self.eye_rotation.0) * self.eye_speed * delta_time;
            self.eye_rotation.1 += (target_v - self.eye_rotation.1) * self.eye_speed * delta_time;
        }
    }

    /// Get blink weight
    #[must_use]
    pub fn blink_weight(&self) -> f32 {
        let progress = self.blink_progress.clamp(0.0, 1.0);
        // Smooth blink curve
        if progress < 0.5 {
            // Closing
            (progress * 2.0).powi(2)
        } else {
            // Opening
            (1.0 - (progress - 0.5) * 2.0).powi(2)
        }
    }

    /// Get eye rotation (horizontal, vertical)
    #[must_use]
    pub fn eye_rotation(&self) -> (f32, f32) {
        self.eye_rotation
    }

    fn pseudo_random(&self) -> f32 {
        ((self.next_blink * 12.9898).sin() * 43758.5453).fract()
    }
}

/// Complete facial animation controller
pub struct FacialAnimator {
    /// Blend shapes
    pub blend_shapes: HashMap<FacialBlendShape, f32>,
    /// Active expressions
    expressions: Vec<(FacialExpression, f32)>,
    /// Lip sync
    pub lip_sync: LipSync,
    /// Procedural eyes
    pub eyes: ProceduralEyes,
    /// Blend expression weight
    pub expression_weight: f32,
}

impl Default for FacialAnimator {
    fn default() -> Self {
        Self::new()
    }
}

impl FacialAnimator {
    /// Create a new facial animator
    #[must_use]
    pub fn new() -> Self {
        Self {
            blend_shapes: HashMap::new(),
            expressions: Vec::new(),
            lip_sync: LipSync::new(),
            eyes: ProceduralEyes::default(),
            expression_weight: 1.0,
        }
    }

    /// Set expression
    pub fn set_expression(&mut self, expression: FacialExpression, weight: f32) {
        self.expressions.clear();
        self.expressions.push((expression, weight));
    }

    /// Add expression layer
    pub fn add_expression(&mut self, expression: FacialExpression, weight: f32) {
        self.expressions.push((expression, weight));
    }

    /// Clear expressions
    pub fn clear_expressions(&mut self) {
        self.expressions.clear();
    }

    /// Update all facial animation
    pub fn update(&mut self, delta_time: f32, head_position: Vec3, head_forward: Vec3) {
        // Reset blend shapes
        self.blend_shapes.clear();
        
        // Apply expressions
        for (expression, weight) in &self.expressions {
            let effective_weight = weight * expression.intensity * self.expression_weight;
            for (shape, expr_weight) in &expression.weights {
                *self.blend_shapes.entry(*shape).or_insert(0.0) += expr_weight * effective_weight;
            }
        }
        
        // Update lip sync
        self.lip_sync.update(delta_time);
        for (shape, weight) in self.lip_sync.get_weights() {
            *self.blend_shapes.entry(*shape).or_insert(0.0) += weight;
        }
        
        // Update eyes
        self.eyes.update(delta_time, head_position, head_forward);
        let blink = self.eyes.blink_weight();
        *self.blend_shapes.entry(FacialBlendShape::EyeBlinkLeft).or_insert(0.0) += blink;
        *self.blend_shapes.entry(FacialBlendShape::EyeBlinkRight).or_insert(0.0) += blink;
        
        // Clamp all weights
        for weight in self.blend_shapes.values_mut() {
            *weight = weight.clamp(0.0, 1.0);
        }
    }

    /// Get blend shape weight
    #[must_use]
    pub fn get_weight(&self, shape: FacialBlendShape) -> f32 {
        self.blend_shapes.get(&shape).copied().unwrap_or(0.0)
    }

    /// Get all weights
    #[must_use]
    pub fn all_weights(&self) -> &HashMap<FacialBlendShape, f32> {
        &self.blend_shapes
    }
}
