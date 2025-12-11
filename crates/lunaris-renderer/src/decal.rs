//! Decal System
//!
//! Projected decals for bullet holes, blood, damage, etc.

use glam::{Vec3, Vec4, Mat4, Quat};

/// Decal blend mode
#[derive(Debug, Clone, Copy, Default)]
pub enum DecalBlendMode {
    /// Normal alpha blend
    #[default]
    Alpha,
    /// Additive blend
    Additive,
    /// Multiply blend
    Multiply,
    /// Replace (no blend)
    Replace,
    /// Screen blend
    Screen,
}

/// Decal projection type
#[derive(Debug, Clone, Copy, Default)]
pub enum DecalProjection {
    /// Box projection
    #[default]
    Box,
    /// Sphere projection
    Sphere,
    /// Cylinder projection
    Cylinder,
}

/// Decal instance
#[derive(Debug, Clone)]
pub struct Decal {
    /// Unique ID
    pub id: u64,
    /// Position
    pub position: Vec3,
    /// Rotation
    pub rotation: Quat,
    /// Size (half extents)
    pub size: Vec3,
    /// Texture/material ID
    pub texture_id: u64,
    /// Color tint
    pub color: Vec4,
    /// Blend mode
    pub blend_mode: DecalBlendMode,
    /// Projection type
    pub projection: DecalProjection,
    /// Fade distance start
    pub fade_start: f32,
    /// Fade distance end
    pub fade_end: f32,
    /// Normal fade (0=no fade, 1=full fade at 90deg)
    pub normal_fade: f32,
    /// Depth offset (prevent z-fighting)
    pub depth_bias: f32,
    /// Layer mask
    pub layer_mask: u32,
    /// Lifetime (seconds, 0=infinite)
    pub lifetime: f32,
    /// Current age
    pub age: f32,
    /// Sort order
    pub sort_order: i32,
}

impl Default for Decal {
    fn default() -> Self {
        Self {
            id: 0,
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            size: Vec3::splat(0.5),
            texture_id: 0,
            color: Vec4::ONE,
            blend_mode: DecalBlendMode::Alpha,
            projection: DecalProjection::Box,
            fade_start: 10.0,
            fade_end: 20.0,
            normal_fade: 0.5,
            depth_bias: 0.001,
            layer_mask: u32::MAX,
            lifetime: 0.0,
            age: 0.0,
            sort_order: 0,
        }
    }
}

impl Decal {
    /// Create a new decal
    #[must_use]
    pub fn new(position: Vec3, normal: Vec3, size: f32) -> Self {
        let rotation = Quat::from_rotation_arc(Vec3::Y, normal);
        Self {
            position,
            rotation,
            size: Vec3::new(size, size * 2.0, size),
            ..Default::default()
        }
    }

    /// Get projection matrix
    #[must_use]
    pub fn projection_matrix(&self) -> Mat4 {
        let inv_scale = Mat4::from_scale(Vec3::ONE / self.size);
        let inv_rotation = Mat4::from_quat(self.rotation.inverse());
        let inv_translation = Mat4::from_translation(-self.position);
        
        inv_scale * inv_rotation * inv_translation
    }

    /// Get world bounds
    #[must_use]
    pub fn world_bounds(&self) -> (Vec3, Vec3) {
        let corners = [
            self.rotation * Vec3::new(-self.size.x, -self.size.y, -self.size.z),
            self.rotation * Vec3::new(self.size.x, -self.size.y, -self.size.z),
            self.rotation * Vec3::new(-self.size.x, self.size.y, -self.size.z),
            self.rotation * Vec3::new(self.size.x, self.size.y, -self.size.z),
            self.rotation * Vec3::new(-self.size.x, -self.size.y, self.size.z),
            self.rotation * Vec3::new(self.size.x, -self.size.y, self.size.z),
            self.rotation * Vec3::new(-self.size.x, self.size.y, self.size.z),
            self.rotation * Vec3::new(self.size.x, self.size.y, self.size.z),
        ];

        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);

        for corner in &corners {
            let world = self.position + *corner;
            min = min.min(world);
            max = max.max(world);
        }

        (min, max)
    }

    /// Calculate alpha based on distance and normal
    #[must_use]
    pub fn calculate_alpha(&self, distance: f32, surface_normal: Vec3) -> f32 {
        // Distance fade
        let dist_alpha = if self.fade_end > self.fade_start {
            1.0 - ((distance - self.fade_start) / (self.fade_end - self.fade_start)).clamp(0.0, 1.0)
        } else {
            1.0
        };

        // Normal fade
        let decal_normal = self.rotation * Vec3::Y;
        let normal_dot = decal_normal.dot(surface_normal).max(0.0);
        let normal_alpha = 1.0 - (1.0 - normal_dot) * self.normal_fade;

        // Lifetime fade
        let life_alpha = if self.lifetime > 0.0 {
            1.0 - (self.age / self.lifetime).clamp(0.0, 1.0)
        } else {
            1.0
        };

        dist_alpha * normal_alpha * life_alpha * self.color.w
    }

    /// Is expired
    #[must_use]
    pub fn expired(&self) -> bool {
        self.lifetime > 0.0 && self.age >= self.lifetime
    }

    /// Update age
    pub fn update(&mut self, delta_time: f32) {
        self.age += delta_time;
    }
}

/// Decal manager
pub struct DecalManager {
    /// All decals
    decals: Vec<Decal>,
    /// Next decal ID
    next_id: u64,
    /// Max decals
    pub max_decals: usize,
    /// Auto cleanup expired
    pub auto_cleanup: bool,
}

impl Default for DecalManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DecalManager {
    /// Create a new decal manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            decals: Vec::new(),
            next_id: 1,
            max_decals: 256,
            auto_cleanup: true,
        }
    }

    /// Spawn a decal
    pub fn spawn(&mut self, mut decal: Decal) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        decal.id = id;
        
        self.decals.push(decal);
        
        // Remove oldest if at limit
        while self.decals.len() > self.max_decals {
            self.decals.remove(0);
        }

        id
    }

    /// Spawn at hit point
    pub fn spawn_at_hit(&mut self, position: Vec3, normal: Vec3, texture_id: u64, size: f32) -> u64 {
        let mut decal = Decal::new(position + normal * 0.01, normal, size);
        decal.texture_id = texture_id;
        self.spawn(decal)
    }

    /// Remove decal by ID
    pub fn remove(&mut self, id: u64) {
        self.decals.retain(|d| d.id != id);
    }

    /// Update all decals
    pub fn update(&mut self, delta_time: f32) {
        for decal in &mut self.decals {
            decal.update(delta_time);
        }

        if self.auto_cleanup {
            self.decals.retain(|d| !d.expired());
        }
    }

    /// Get visible decals
    #[must_use]
    pub fn visible_decals(&self, camera_pos: Vec3, frustum_bounds: Option<(Vec3, Vec3)>) -> Vec<&Decal> {
        let mut visible: Vec<_> = self.decals.iter()
            .filter(|d| {
                if let Some((fmin, fmax)) = frustum_bounds {
                    let (dmin, dmax) = d.world_bounds();
                    // AABB intersection test
                    dmax.x >= fmin.x && dmin.x <= fmax.x &&
                    dmax.y >= fmin.y && dmin.y <= fmax.y &&
                    dmax.z >= fmin.z && dmin.z <= fmax.z
                } else {
                    true
                }
            })
            .collect();

        // Sort by distance
        visible.sort_by(|a, b| {
            let da = (a.position - camera_pos).length_squared();
            let db = (b.position - camera_pos).length_squared();
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        });

        visible
    }

    /// Get decal count
    #[must_use]
    pub fn count(&self) -> usize {
        self.decals.len()
    }

    /// Clear all decals
    pub fn clear(&mut self) {
        self.decals.clear();
    }
}

/// Common decal presets
pub struct DecalPresets;

impl DecalPresets {
    /// Bullet hole decal
    #[must_use]
    pub fn bullet_hole(position: Vec3, normal: Vec3) -> Decal {
        let mut decal = Decal::new(position, normal, 0.05);
        decal.lifetime = 60.0;
        decal.color = Vec4::new(0.2, 0.2, 0.2, 1.0);
        decal
    }

    /// Blood splatter
    #[must_use]
    pub fn blood(position: Vec3, normal: Vec3, size: f32) -> Decal {
        let mut decal = Decal::new(position, normal, size);
        decal.lifetime = 120.0;
        decal.color = Vec4::new(0.5, 0.0, 0.0, 0.9);
        decal
    }

    /// Burn mark
    #[must_use]
    pub fn burn_mark(position: Vec3, normal: Vec3, size: f32) -> Decal {
        let mut decal = Decal::new(position, normal, size);
        decal.lifetime = 0.0; // Permanent
        decal.color = Vec4::new(0.1, 0.1, 0.1, 0.8);
        decal
    }

    /// Footprint
    #[must_use]
    pub fn footprint(position: Vec3, rotation: Quat) -> Decal {
        Decal {
            position,
            rotation,
            size: Vec3::new(0.3, 0.1, 0.5),
            lifetime: 30.0,
            color: Vec4::new(0.3, 0.25, 0.2, 0.6),
            ..Default::default()
        }
    }
}
