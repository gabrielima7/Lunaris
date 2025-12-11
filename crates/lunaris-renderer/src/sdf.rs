//! Signed Distance Fields (SDF) Raymarching
//!
//! Procedural geometry, CSG operations, smooth blending.

use glam::{Vec2, Vec3, Vec4, Mat4};

/// SDF system
pub struct SDFSystem {
    pub scenes: Vec<SDFScene>,
    pub materials: Vec<SDFMaterial>,
    pub settings: SDFSettings,
}

/// SDF scene
pub struct SDFScene {
    pub id: u64,
    pub name: String,
    pub root: SDFNode,
}

/// SDF node (CSG tree)
pub enum SDFNode {
    Primitive(SDFPrimitive),
    Transform { child: Box<SDFNode>, transform: Mat4 },
    Union { a: Box<SDFNode>, b: Box<SDFNode>, smooth: f32 },
    Intersection { a: Box<SDFNode>, b: Box<SDFNode>, smooth: f32 },
    Subtraction { a: Box<SDFNode>, b: Box<SDFNode>, smooth: f32 },
    Blend { a: Box<SDFNode>, b: Box<SDFNode>, factor: f32 },
    Repeat { child: Box<SDFNode>, period: Vec3 },
    Twist { child: Box<SDFNode>, amount: f32 },
    Bend { child: Box<SDFNode>, amount: f32 },
    Displacement { child: Box<SDFNode>, frequency: f32, amplitude: f32 },
    Round { child: Box<SDFNode>, radius: f32 },
    Onion { child: Box<SDFNode>, thickness: f32 },
}

/// SDF primitive
pub enum SDFPrimitive {
    Sphere { radius: f32 },
    Box { size: Vec3 },
    RoundBox { size: Vec3, radius: f32 },
    Torus { major_radius: f32, minor_radius: f32 },
    Cylinder { height: f32, radius: f32 },
    Cone { height: f32, radius: f32 },
    Capsule { a: Vec3, b: Vec3, radius: f32 },
    Plane { normal: Vec3, distance: f32 },
    Mandelbulb { power: f32, iterations: u32 },
    Julia { c: Vec4, iterations: u32 },
}

impl SDFNode {
    pub fn evaluate(&self, p: Vec3) -> f32 {
        match self {
            SDFNode::Primitive(prim) => prim.evaluate(p),
            SDFNode::Transform { child, transform } => {
                let inv = transform.inverse();
                let local_p = inv.transform_point3(p);
                child.evaluate(local_p)
            }
            SDFNode::Union { a, b, smooth } => smooth_union(a.evaluate(p), b.evaluate(p), *smooth),
            SDFNode::Intersection { a, b, smooth } => smooth_intersection(a.evaluate(p), b.evaluate(p), *smooth),
            SDFNode::Subtraction { a, b, smooth } => smooth_subtraction(a.evaluate(p), b.evaluate(p), *smooth),
            SDFNode::Blend { a, b, factor } => a.evaluate(p) * (1.0 - *factor) + b.evaluate(p) * *factor,
            SDFNode::Repeat { child, period } => {
                let q = Vec3::new(
                    (p.x % period.x) - period.x * 0.5,
                    (p.y % period.y) - period.y * 0.5,
                    (p.z % period.z) - period.z * 0.5,
                );
                child.evaluate(q)
            }
            SDFNode::Twist { child, amount } => {
                let c = (p.y * *amount).cos();
                let s = (p.y * *amount).sin();
                let q = Vec3::new(c * p.x - s * p.z, p.y, s * p.x + c * p.z);
                child.evaluate(q)
            }
            SDFNode::Bend { child, amount } => {
                let c = (p.x * *amount).cos();
                let s = (p.x * *amount).sin();
                let q = Vec3::new(c * p.x - s * p.y, s * p.x + c * p.y, p.z);
                child.evaluate(q)
            }
            SDFNode::Displacement { child, frequency, amplitude } => {
                let d = (p.x * *frequency).sin() * (p.y * *frequency).sin() * (p.z * *frequency).sin() * *amplitude;
                child.evaluate(p) + d
            }
            SDFNode::Round { child, radius } => child.evaluate(p) - *radius,
            SDFNode::Onion { child, thickness } => child.evaluate(p).abs() - *thickness,
        }
    }

    pub fn normal(&self, p: Vec3) -> Vec3 {
        let eps = 0.001;
        let d = self.evaluate(p);
        Vec3::new(
            self.evaluate(p + Vec3::new(eps, 0.0, 0.0)) - d,
            self.evaluate(p + Vec3::new(0.0, eps, 0.0)) - d,
            self.evaluate(p + Vec3::new(0.0, 0.0, eps)) - d,
        ).normalize()
    }
}

impl SDFPrimitive {
    pub fn evaluate(&self, p: Vec3) -> f32 {
        match self {
            SDFPrimitive::Sphere { radius } => p.length() - *radius,
            SDFPrimitive::Box { size } => {
                let q = p.abs() - *size;
                q.max(Vec3::ZERO).length() + q.max_element().min(0.0)
            }
            SDFPrimitive::RoundBox { size, radius } => {
                let q = p.abs() - *size;
                q.max(Vec3::ZERO).length() + q.max_element().min(0.0) - *radius
            }
            SDFPrimitive::Torus { major_radius, minor_radius } => {
                let q = Vec2::new(Vec2::new(p.x, p.z).length() - *major_radius, p.y);
                q.length() - *minor_radius
            }
            SDFPrimitive::Cylinder { height, radius } => {
                let d = Vec2::new(Vec2::new(p.x, p.z).length(), p.y).abs() - Vec2::new(*radius, *height);
                d.x.max(d.y).min(0.0) + d.max(Vec2::ZERO).length()
            }
            SDFPrimitive::Cone { height, radius } => {
                let q = Vec2::new(Vec2::new(p.x, p.z).length(), -p.y);
                let tip = Vec2::new(0.0, *height);
                let bottom = Vec2::new(*radius, 0.0);
                let e = bottom - tip;
                let w = q - tip;
                let t = (w.dot(e) / e.dot(e)).clamp(0.0, 1.0);
                (w - e * t).length()
            }
            SDFPrimitive::Capsule { a, b, radius } => {
                let ab = *b - *a;
                let t = (p - *a).dot(ab) / ab.dot(ab);
                let t = t.clamp(0.0, 1.0);
                (p - *a - ab * t).length() - *radius
            }
            SDFPrimitive::Plane { normal, distance } => p.dot(*normal) + *distance,
            SDFPrimitive::Mandelbulb { power, iterations } => {
                let mut z = p;
                let mut dr = 1.0;
                let mut r = 0.0;
                for _ in 0..*iterations {
                    r = z.length();
                    if r > 2.0 { break; }
                    let theta = (z.z / r).acos();
                    let phi = z.y.atan2(z.x);
                    dr = r.powf(*power - 1.0) * *power * dr + 1.0;
                    let zr = r.powf(*power);
                    let theta = theta * *power;
                    let phi = phi * *power;
                    z = Vec3::new(theta.sin() * phi.cos(), theta.sin() * phi.sin(), theta.cos()) * zr + p;
                }
                0.5 * r.ln() * r / dr
            }
            SDFPrimitive::Julia { c, iterations } => 0.0, // Simplified
        }
    }
}

fn smooth_union(d1: f32, d2: f32, k: f32) -> f32 {
    if k <= 0.0 { return d1.min(d2); }
    let h = (0.5 + 0.5 * (d2 - d1) / k).clamp(0.0, 1.0);
    lerp(d2, d1, h) - k * h * (1.0 - h)
}

fn smooth_intersection(d1: f32, d2: f32, k: f32) -> f32 {
    -smooth_union(-d1, -d2, k)
}

fn smooth_subtraction(d1: f32, d2: f32, k: f32) -> f32 {
    smooth_intersection(d1, -d2, k)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 { a + (b - a) * t }

/// SDF material
pub struct SDFMaterial {
    pub color: Vec3,
    pub roughness: f32,
    pub metallic: f32,
    pub emission: Vec3,
}

/// SDF settings
pub struct SDFSettings {
    pub max_steps: u32,
    pub max_distance: f32,
    pub epsilon: f32,
    pub shadow_softness: f32,
    pub ao_steps: u32,
}

impl Default for SDFSettings {
    fn default() -> Self {
        Self { max_steps: 256, max_distance: 100.0, epsilon: 0.001, shadow_softness: 16.0, ao_steps: 5 }
    }
}

impl SDFSystem {
    pub fn new() -> Self { Self { scenes: Vec::new(), materials: Vec::new(), settings: SDFSettings::default() } }

    pub fn raymarch(&self, scene: &SDFScene, origin: Vec3, direction: Vec3) -> Option<(f32, Vec3)> {
        let mut t = 0.0;
        for _ in 0..self.settings.max_steps {
            let p = origin + direction * t;
            let d = scene.root.evaluate(p);
            if d < self.settings.epsilon { return Some((t, scene.root.normal(p))); }
            if t > self.settings.max_distance { break; }
            t += d;
        }
        None
    }

    pub fn soft_shadow(&self, scene: &SDFScene, origin: Vec3, direction: Vec3, max_t: f32) -> f32 {
        let mut res = 1.0;
        let mut t = 0.01;
        while t < max_t {
            let h = scene.root.evaluate(origin + direction * t);
            if h < self.settings.epsilon { return 0.0; }
            res = res.min(self.settings.shadow_softness * h / t);
            t += h;
        }
        res.clamp(0.0, 1.0)
    }

    pub fn ambient_occlusion(&self, scene: &SDFScene, p: Vec3, n: Vec3) -> f32 {
        let mut occ = 0.0;
        let mut sca = 1.0;
        for i in 0..self.settings.ao_steps {
            let h = 0.01 + 0.12 * i as f32;
            let d = scene.root.evaluate(p + n * h);
            occ += (h - d) * sca;
            sca *= 0.95;
        }
        (1.0 - 3.0 * occ).clamp(0.0, 1.0)
    }
}
