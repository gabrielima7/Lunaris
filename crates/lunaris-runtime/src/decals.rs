//! Decal System
//!
//! Projected decals for blood splatter, bullet holes, footprints.

use glam::{Vec3, Quat, Mat4};

/// Decal system
pub struct DecalSystem {
    pub decals: Vec<Decal>,
    pub pools: Vec<DecalPool>,
    pub settings: DecalSettings,
}

/// Decal
pub struct Decal {
    pub id: u64,
    pub position: Vec3,
    pub rotation: Quat,
    pub size: Vec3,
    pub material: String,
    pub color: [f32; 4],
    pub lifetime: f32,
    pub age: f32,
    pub fade_out: f32,
    pub sort_order: i32,
    pub pool_id: Option<u64>,
}

/// Decal pool (for reusing decals)
pub struct DecalPool {
    pub id: u64,
    pub name: String,
    pub material: String,
    pub max_decals: usize,
    pub decal_ids: Vec<u64>,
}

/// Decal settings
pub struct DecalSettings {
    pub max_decals: usize,
    pub default_lifetime: f32,
    pub fade_duration: f32,
    pub depth_bias: f32,
    pub normal_threshold: f32,
}

impl Default for DecalSettings {
    fn default() -> Self {
        Self { max_decals: 500, default_lifetime: 30.0, fade_duration: 2.0, depth_bias: 0.001, normal_threshold: 0.5 }
    }
}

impl DecalSystem {
    pub fn new() -> Self {
        Self { decals: Vec::new(), pools: Vec::new(), settings: DecalSettings::default() }
    }

    pub fn spawn(&mut self, position: Vec3, normal: Vec3, material: &str, size: f32) -> u64 {
        let id = self.decals.len() as u64;
        let rotation = Quat::from_rotation_arc(Vec3::Y, normal);
        
        self.decals.push(Decal {
            id, position, rotation, size: Vec3::splat(size), material: material.into(),
            color: [1.0, 1.0, 1.0, 1.0], lifetime: self.settings.default_lifetime, age: 0.0,
            fade_out: self.settings.fade_duration, sort_order: 0, pool_id: None,
        });
        
        // Remove oldest if over limit
        while self.decals.len() > self.settings.max_decals {
            self.decals.remove(0);
        }
        
        id
    }

    pub fn spawn_pooled(&mut self, pool_id: u64, position: Vec3, normal: Vec3, size: f32) -> Option<u64> {
        let pool = self.pools.iter_mut().find(|p| p.id == pool_id)?;
        let material = pool.material.clone();
        
        if pool.decal_ids.len() >= pool.max_decals {
            // Recycle oldest
            let oldest_id = pool.decal_ids.remove(0);
            if let Some(decal) = self.decals.iter_mut().find(|d| d.id == oldest_id) {
                decal.position = position;
                decal.rotation = Quat::from_rotation_arc(Vec3::Y, normal);
                decal.age = 0.0;
                pool.decal_ids.push(oldest_id);
                return Some(oldest_id);
            }
        }
        
        let id = self.spawn(position, normal, &material, size);
        pool.decal_ids.push(id);
        self.decals.iter_mut().find(|d| d.id == id)?.pool_id = Some(pool_id);
        Some(id)
    }

    pub fn create_pool(&mut self, name: &str, material: &str, max_decals: usize) -> u64 {
        let id = self.pools.len() as u64;
        self.pools.push(DecalPool { id, name: name.into(), material: material.into(), max_decals, decal_ids: Vec::new() });
        id
    }

    pub fn update(&mut self, dt: f32) {
        for decal in &mut self.decals {
            decal.age += dt;
            if decal.age > decal.lifetime - decal.fade_out {
                let fade_progress = (decal.age - (decal.lifetime - decal.fade_out)) / decal.fade_out;
                decal.color[3] = 1.0 - fade_progress;
            }
        }
        
        // Remove expired decals
        self.decals.retain(|d| d.age < d.lifetime);
    }

    pub fn get_projection_matrix(&self, decal: &Decal) -> Mat4 {
        let scale = Mat4::from_scale(decal.size);
        let rotation = Mat4::from_quat(decal.rotation);
        let translation = Mat4::from_translation(decal.position);
        translation * rotation * scale
    }

    pub fn clear(&mut self) { self.decals.clear(); }
    pub fn count(&self) -> usize { self.decals.len() }
}

// Preset spawners
impl DecalSystem {
    pub fn blood_splatter(&mut self, position: Vec3, normal: Vec3) -> u64 {
        let id = self.spawn(position, normal, "decals/blood", 0.5 + rand() * 0.5);
        if let Some(decal) = self.decals.iter_mut().find(|d| d.id == id) {
            decal.color = [0.5, 0.0, 0.0, 1.0];
            decal.lifetime = 60.0;
        }
        id
    }

    pub fn bullet_hole(&mut self, position: Vec3, normal: Vec3, material_type: &str) -> u64 {
        let mat = match material_type {
            "metal" => "decals/bullet_metal",
            "wood" => "decals/bullet_wood",
            "concrete" => "decals/bullet_concrete",
            _ => "decals/bullet_generic",
        };
        self.spawn(position, normal, mat, 0.05)
    }

    pub fn footprint(&mut self, position: Vec3, rotation: Quat, left: bool) -> u64 {
        let mat = if left { "decals/footprint_left" } else { "decals/footprint_right" };
        let id = self.spawn(position, Vec3::Y, mat, 0.3);
        if let Some(decal) = self.decals.iter_mut().find(|d| d.id == id) {
            decal.rotation = rotation;
            decal.lifetime = 10.0;
            decal.color[3] = 0.5;
        }
        id
    }

    pub fn scorch_mark(&mut self, position: Vec3, normal: Vec3, size: f32) -> u64 {
        let id = self.spawn(position, normal, "decals/scorch", size);
        if let Some(decal) = self.decals.iter_mut().find(|d| d.id == id) {
            decal.lifetime = 120.0;
        }
        id
    }
}

fn rand() -> f32 { 0.5 }
