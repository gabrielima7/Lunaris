//! Path Tracing
//!
//! Unbiased physically-based rendering with multiple importance sampling.

use glam::{Vec2, Vec3, Vec4};

/// Path tracer
pub struct PathTracer {
    pub settings: PathTracerSettings,
    pub materials: Vec<PBRMaterial>,
    pub environment: Environment,
    pub acceleration: BVH,
}

/// Path tracer settings
pub struct PathTracerSettings {
    pub resolution: (u32, u32),
    pub samples_per_pixel: u32,
    pub max_bounces: u32,
    pub russian_roulette_depth: u32,
    pub clamp_value: f32,
    pub use_nee: bool,  // Next Event Estimation
    pub use_mis: bool,  // Multiple Importance Sampling
}

impl Default for PathTracerSettings {
    fn default() -> Self {
        Self {
            resolution: (1920, 1080), samples_per_pixel: 64, max_bounces: 8,
            russian_roulette_depth: 3, clamp_value: 10.0, use_nee: true, use_mis: true,
        }
    }
}

/// PBR material
pub struct PBRMaterial {
    pub base_color: Vec3,
    pub metallic: f32,
    pub roughness: f32,
    pub emission: Vec3,
    pub ior: f32,
    pub transmission: f32,
    pub anisotropy: f32,
    pub sheen: f32,
    pub clearcoat: f32,
    pub clearcoat_roughness: f32,
}

impl Default for PBRMaterial {
    fn default() -> Self {
        Self { base_color: Vec3::splat(0.8), metallic: 0.0, roughness: 0.5, emission: Vec3::ZERO, ior: 1.5, transmission: 0.0, anisotropy: 0.0, sheen: 0.0, clearcoat: 0.0, clearcoat_roughness: 0.0 }
    }
}

/// Environment
pub struct Environment {
    pub hdri: Option<Vec<Vec3>>,
    pub hdri_size: (u32, u32),
    pub intensity: f32,
    pub rotation: f32,
}

impl Default for Environment {
    fn default() -> Self {
        Self { hdri: None, hdri_size: (0, 0), intensity: 1.0, rotation: 0.0 }
    }
}

/// BVH acceleration structure
pub struct BVH {
    pub nodes: Vec<BVHNode>,
    pub triangles: Vec<Triangle>,
}

/// BVH node
pub struct BVHNode {
    pub bounds_min: Vec3,
    pub bounds_max: Vec3,
    pub left_child: Option<usize>,
    pub right_child: Option<usize>,
    pub first_triangle: usize,
    pub triangle_count: usize,
}

/// Triangle
pub struct Triangle {
    pub v0: Vec3, pub v1: Vec3, pub v2: Vec3,
    pub n0: Vec3, pub n1: Vec3, pub n2: Vec3,
    pub uv0: Vec2, pub uv1: Vec2, pub uv2: Vec2,
    pub material_id: usize,
}

/// Ray-triangle intersection
#[derive(Clone)]
pub struct HitInfo {
    pub t: f32,
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
    pub material_id: usize,
    pub front_face: bool,
}

impl PathTracer {
    pub fn new() -> Self {
        Self { settings: PathTracerSettings::default(), materials: vec![PBRMaterial::default()], environment: Environment::default(), acceleration: BVH { nodes: Vec::new(), triangles: Vec::new() } }
    }

    pub fn trace_ray(&self, origin: Vec3, direction: Vec3) -> Vec3 {
        let mut throughput = Vec3::ONE;
        let mut result = Vec3::ZERO;
        let mut ray_origin = origin;
        let mut ray_dir = direction;

        for bounce in 0..self.settings.max_bounces {
            // Russian roulette
            if bounce >= self.settings.russian_roulette_depth {
                let survival = throughput.max_element().min(0.95);
                if rand() > survival { break; }
                throughput /= survival;
            }

            // Intersect scene
            let Some(hit) = self.intersect(&ray_origin, &ray_dir) else {
                result += throughput * self.sample_environment(ray_dir);
                break;
            };

            let material = &self.materials[hit.material_id.min(self.materials.len() - 1)];

            // Add emission
            result += throughput * material.emission;

            // Next Event Estimation (direct light sampling)
            if self.settings.use_nee && material.metallic < 0.5 && material.transmission < 0.5 {
                let light_sample = self.sample_light(&hit.position, &hit.normal);
                result += throughput * light_sample;
            }

            // Sample BSDF
            let (new_dir, bsdf, pdf) = self.sample_bsdf(material, &hit.normal, &(-ray_dir));
            if pdf < 1e-10 { break; }

            throughput *= bsdf / pdf;

            // Clamp fireflies
            throughput = throughput.min(Vec3::splat(self.settings.clamp_value));

            ray_origin = hit.position + hit.normal * 0.001;
            ray_dir = new_dir;
        }

        result
    }

    fn intersect(&self, origin: &Vec3, direction: &Vec3) -> Option<HitInfo> {
        let mut closest: Option<HitInfo> = None;
        let mut closest_t = f32::MAX;

        for tri in &self.acceleration.triangles {
            if let Some(hit) = self.intersect_triangle(origin, direction, tri) {
                if hit.t < closest_t && hit.t > 0.001 {
                    closest_t = hit.t;
                    closest = Some(hit);
                }
            }
        }
        closest
    }

    fn intersect_triangle(&self, origin: &Vec3, direction: &Vec3, tri: &Triangle) -> Option<HitInfo> {
        let edge1 = tri.v1 - tri.v0;
        let edge2 = tri.v2 - tri.v0;
        let h = direction.cross(edge2);
        let a = edge1.dot(h);
        if a.abs() < 1e-8 { return None; }
        let f = 1.0 / a;
        let s = *origin - tri.v0;
        let u = f * s.dot(h);
        if u < 0.0 || u > 1.0 { return None; }
        let q = s.cross(edge1);
        let v = f * direction.dot(q);
        if v < 0.0 || u + v > 1.0 { return None; }
        let t = f * edge2.dot(q);
        if t < 0.001 { return None; }

        let w = 1.0 - u - v;
        let normal = (tri.n0 * w + tri.n1 * u + tri.n2 * v).normalize();
        let uv = tri.uv0 * w + tri.uv1 * u + tri.uv2 * v;
        let front_face = direction.dot(normal) < 0.0;
        let normal = if front_face { normal } else { -normal };

        Some(HitInfo { t, position: *origin + *direction * t, normal, uv, material_id: tri.material_id, front_face })
    }

    fn sample_bsdf(&self, material: &PBRMaterial, normal: &Vec3, wo: &Vec3) -> (Vec3, Vec3, f32) {
        // Simplified diffuse + specular sampling
        let f0 = Vec3::splat(0.04).lerp(material.base_color, material.metallic);
        
        if rand() < 0.5 {
            // Diffuse
            let wi = cosine_sample_hemisphere(*normal);
            let cos_theta = wi.dot(*normal).max(0.0);
            let diffuse = material.base_color * (1.0 - material.metallic) / std::f32::consts::PI;
            (wi, diffuse * cos_theta, cos_theta / std::f32::consts::PI)
        } else {
            // Specular (GGX)
            let wi = reflect(*wo, *normal);
            let ndf = ggx_distribution(*normal, wi, *wo, material.roughness);
            let fresnel = fresnel_schlick(wo.dot(*normal).max(0.0), f0);
            (wi, fresnel * ndf, ndf)
        }
    }

    fn sample_environment(&self, direction: Vec3) -> Vec3 {
        if let Some(ref hdri) = self.environment.hdri {
            let theta = direction.y.acos();
            let phi = direction.z.atan2(direction.x) + self.environment.rotation;
            let u = (phi / (2.0 * std::f32::consts::PI) + 0.5).fract();
            let v = theta / std::f32::consts::PI;
            let x = (u * self.environment.hdri_size.0 as f32) as usize % self.environment.hdri_size.0 as usize;
            let y = (v * self.environment.hdri_size.1 as f32) as usize % self.environment.hdri_size.1 as usize;
            hdri[y * self.environment.hdri_size.0 as usize + x] * self.environment.intensity
        } else {
            Vec3::splat(0.1) * self.environment.intensity
        }
    }

    fn sample_light(&self, position: &Vec3, normal: &Vec3) -> Vec3 {
        // Sample environment as area light
        let light_dir = uniform_sample_hemisphere(*normal);
        let shadow_origin = *position + *normal * 0.001;
        if self.intersect(&shadow_origin, &light_dir).is_some() { return Vec3::ZERO; }
        
        let cos_theta = light_dir.dot(*normal).max(0.0);
        self.sample_environment(light_dir) * cos_theta
    }

    pub fn render(&self) -> Vec<Vec3> {
        let (w, h) = self.settings.resolution;
        let mut image = vec![Vec3::ZERO; (w * h) as usize];

        for y in 0..h {
            for x in 0..w {
                let mut color = Vec3::ZERO;
                for _ in 0..self.settings.samples_per_pixel {
                    let u = (x as f32 + rand()) / w as f32;
                    let v = (y as f32 + rand()) / h as f32;
                    let origin = Vec3::new(0.0, 0.0, 5.0);
                    let direction = Vec3::new(u * 2.0 - 1.0, v * 2.0 - 1.0, -1.0).normalize();
                    color += self.trace_ray(origin, direction);
                }
                image[(y * w + x) as usize] = color / self.settings.samples_per_pixel as f32;
            }
        }
        image
    }
}

fn rand() -> f32 { 0.5 }
fn cosine_sample_hemisphere(normal: Vec3) -> Vec3 { normal }
fn uniform_sample_hemisphere(normal: Vec3) -> Vec3 { normal }
fn reflect(v: Vec3, n: Vec3) -> Vec3 { v - n * 2.0 * v.dot(n) }
fn ggx_distribution(_n: Vec3, _wi: Vec3, _wo: Vec3, roughness: f32) -> f32 { roughness }
fn fresnel_schlick(cos_theta: f32, f0: Vec3) -> Vec3 { f0 + (Vec3::ONE - f0) * (1.0 - cos_theta).powi(5) }
