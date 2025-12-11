//! Spline System
//!
//! Bezier/Catmull-Rom curves for roads, rails, and paths.

use glam::{Vec3, Quat};

/// Spline
pub struct Spline {
    pub points: Vec<SplinePoint>,
    pub spline_type: SplineType,
    pub closed: bool,
    pub resolution: u32,
}

/// Spline point
pub struct SplinePoint {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub tangent_in: Vec3,
    pub tangent_out: Vec3,
    pub roll: f32,
}

/// Spline type
pub enum SplineType { Linear, CatmullRom, Bezier, Hermite }

impl Default for SplinePoint {
    fn default() -> Self {
        Self { position: Vec3::ZERO, rotation: Quat::IDENTITY, scale: Vec3::ONE, tangent_in: Vec3::NEG_Z, tangent_out: Vec3::Z, roll: 0.0 }
    }
}

impl Spline {
    pub fn new(spline_type: SplineType) -> Self {
        Self { points: Vec::new(), spline_type, closed: false, resolution: 32 }
    }

    pub fn add_point(&mut self, position: Vec3) {
        self.points.push(SplinePoint { position, ..Default::default() });
        self.auto_tangents();
    }

    fn auto_tangents(&mut self) {
        for i in 0..self.points.len() {
            let prev = if i > 0 { Some(self.points[i - 1].position) } else if self.closed { Some(self.points.last().unwrap().position) } else { None };
            let next = if i < self.points.len() - 1 { Some(self.points[i + 1].position) } else if self.closed { Some(self.points[0].position) } else { None };
            
            if let (Some(p), Some(n)) = (prev, next) {
                let tangent = (n - p).normalize() * ((n - p).length() * 0.25);
                self.points[i].tangent_in = -tangent;
                self.points[i].tangent_out = tangent;
            }
        }
    }

    pub fn evaluate(&self, t: f32) -> Vec3 {
        if self.points.len() < 2 { return self.points.first().map(|p| p.position).unwrap_or(Vec3::ZERO); }
        
        let segment_count = if self.closed { self.points.len() } else { self.points.len() - 1 };
        let t_scaled = t * segment_count as f32;
        let segment = (t_scaled.floor() as usize).min(segment_count - 1);
        let local_t = t_scaled.fract();
        
        let i0 = segment;
        let i1 = (segment + 1) % self.points.len();
        
        match self.spline_type {
            SplineType::Linear => self.points[i0].position.lerp(self.points[i1].position, local_t),
            SplineType::CatmullRom => self.catmull_rom(segment, local_t),
            SplineType::Bezier => self.bezier(segment, local_t),
            SplineType::Hermite => self.hermite(segment, local_t),
        }
    }

    fn catmull_rom(&self, segment: usize, t: f32) -> Vec3 {
        let n = self.points.len();
        let p0 = self.points[if segment == 0 { if self.closed { n - 1 } else { 0 } } else { segment - 1 }].position;
        let p1 = self.points[segment].position;
        let p2 = self.points[(segment + 1) % n].position;
        let p3 = self.points[(segment + 2) % n].position;
        
        let t2 = t * t;
        let t3 = t2 * t;
        
        0.5 * ((2.0 * p1) + (-p0 + p2) * t + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2 + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
    }

    fn bezier(&self, segment: usize, t: f32) -> Vec3 {
        let p0 = self.points[segment].position;
        let p1 = self.points[segment].position + self.points[segment].tangent_out;
        let p2 = self.points[(segment + 1) % self.points.len()].position + self.points[(segment + 1) % self.points.len()].tangent_in;
        let p3 = self.points[(segment + 1) % self.points.len()].position;
        
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        
        p0 * mt3 + p1 * 3.0 * mt2 * t + p2 * 3.0 * mt * t2 + p3 * t3
    }

    fn hermite(&self, segment: usize, t: f32) -> Vec3 {
        let p0 = self.points[segment].position;
        let p1 = self.points[(segment + 1) % self.points.len()].position;
        let m0 = self.points[segment].tangent_out;
        let m1 = self.points[(segment + 1) % self.points.len()].tangent_in;
        
        let t2 = t * t;
        let t3 = t2 * t;
        
        p0 * (2.0 * t3 - 3.0 * t2 + 1.0) + m0 * (t3 - 2.0 * t2 + t) + p1 * (-2.0 * t3 + 3.0 * t2) + m1 * (t3 - t2)
    }

    pub fn get_tangent(&self, t: f32) -> Vec3 {
        let delta = 0.001;
        let a = self.evaluate((t - delta).max(0.0));
        let b = self.evaluate((t + delta).min(1.0));
        (b - a).normalize()
    }

    pub fn get_length(&self) -> f32 {
        let mut length = 0.0;
        let steps = self.resolution * self.points.len().max(1) as u32;
        for i in 0..steps {
            let t0 = i as f32 / steps as f32;
            let t1 = (i + 1) as f32 / steps as f32;
            length += (self.evaluate(t1) - self.evaluate(t0)).length();
        }
        length
    }

    pub fn sample_points(&self, count: usize) -> Vec<Vec3> {
        (0..count).map(|i| self.evaluate(i as f32 / (count - 1).max(1) as f32)).collect()
    }
}

/// Spline mesh
pub struct SplineMesh {
    pub spline: Spline,
    pub mesh_template: String,
    pub scale: Vec3,
    pub spacing: f32,
    pub align_to_spline: bool,
    pub deform_mesh: bool,
}

impl SplineMesh {
    pub fn generate_instances(&self) -> Vec<(Vec3, Quat, Vec3)> {
        let length = self.spline.get_length();
        let count = (length / self.spacing).ceil() as usize;
        
        (0..count).map(|i| {
            let t = i as f32 / count.max(1) as f32;
            let pos = self.spline.evaluate(t);
            let forward = self.spline.get_tangent(t);
            let rot = Quat::from_rotation_arc(Vec3::Z, forward);
            (pos, rot, self.scale)
        }).collect()
    }
}
