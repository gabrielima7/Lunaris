//! Ray Tracing System
//!
//! Hardware and software ray tracing for realistic rendering.

use glam::{Vec3, Vec4, Mat4};
use std::collections::HashMap;

/// Ray structure
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    /// Origin point
    pub origin: Vec3,
    /// Direction (normalized)
    pub direction: Vec3,
    /// Minimum t value
    pub t_min: f32,
    /// Maximum t value
    pub t_max: f32,
}

impl Ray {
    /// Create a new ray
    #[must_use]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
            t_min: 0.001,
            t_max: f32::MAX,
        }
    }

    /// Get point at distance t
    #[must_use]
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

/// Ray hit information
#[derive(Debug, Clone)]
pub struct RayHit {
    /// Hit position
    pub position: Vec3,
    /// Surface normal
    pub normal: Vec3,
    /// Distance from ray origin
    pub distance: f32,
    /// UV coordinates
    pub uv: [f32; 2],
    /// Material ID
    pub material_id: u32,
    /// Instance ID
    pub instance_id: u32,
    /// Primitive index
    pub primitive_index: u32,
    /// Is front face
    pub front_face: bool,
}

/// Ray tracing mode
#[derive(Debug, Clone, Copy, Default)]
pub enum RTMode {
    /// Software ray tracing (fallback)
    #[default]
    Software,
    /// Hardware ray tracing (requires RTX/DXR)
    Hardware,
    /// Hybrid (uses both)
    Hybrid,
}

/// Ray tracing feature flags
#[derive(Debug, Clone, Copy, Default)]
pub struct RTFeatures {
    /// Ray traced shadows
    pub shadows: bool,
    /// Ray traced reflections
    pub reflections: bool,
    /// Ray traced global illumination
    pub global_illumination: bool,
    /// Ray traced ambient occlusion
    pub ambient_occlusion: bool,
    /// Ray traced translucency
    pub translucency: bool,
}

/// Ray tracing quality
#[derive(Debug, Clone, Copy, Default)]
pub enum RTQuality {
    /// Low (1 sample per pixel)
    Low,
    /// Medium (4 samples)
    #[default]
    Medium,
    /// High (16 samples)
    High,
    /// Ultra (64 samples)
    Ultra,
    /// Reference (4096 samples, offline)
    Reference,
}

impl RTQuality {
    /// Get samples per pixel
    #[must_use]
    pub fn samples(&self) -> u32 {
        match self {
            RTQuality::Low => 1,
            RTQuality::Medium => 4,
            RTQuality::High => 16,
            RTQuality::Ultra => 64,
            RTQuality::Reference => 4096,
        }
    }
}

/// RT configuration
#[derive(Debug, Clone)]
pub struct RTConfig {
    /// Mode
    pub mode: RTMode,
    /// Features
    pub features: RTFeatures,
    /// Quality
    pub quality: RTQuality,
    /// Max bounces
    pub max_bounces: u32,
    /// Denoiser enabled
    pub denoiser: bool,
    /// Temporal accumulation
    pub temporal: bool,
    /// Max ray distance
    pub max_distance: f32,
}

impl Default for RTConfig {
    fn default() -> Self {
        Self {
            mode: RTMode::Software,
            features: RTFeatures {
                shadows: true,
                reflections: true,
                global_illumination: false,
                ambient_occlusion: true,
                translucency: false,
            },
            quality: RTQuality::Medium,
            max_bounces: 4,
            denoiser: true,
            temporal: true,
            max_distance: 1000.0,
        }
    }
}

/// Axis-aligned bounding box
#[derive(Debug, Clone, Copy)]
pub struct AABB {
    /// Minimum corner
    pub min: Vec3,
    /// Maximum corner
    pub max: Vec3,
}

impl AABB {
    /// Create a new AABB
    #[must_use]
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// Create from points
    #[must_use]
    pub fn from_points(points: &[Vec3]) -> Self {
        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);
        
        for p in points {
            min = min.min(*p);
            max = max.max(*p);
        }
        
        Self { min, max }
    }

    /// Expand to include another AABB
    #[must_use]
    pub fn union(&self, other: &AABB) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Get center
    #[must_use]
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    /// Get surface area
    #[must_use]
    pub fn surface_area(&self) -> f32 {
        let d = self.max - self.min;
        2.0 * (d.x * d.y + d.y * d.z + d.z * d.x)
    }

    /// Ray-AABB intersection
    #[must_use]
    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let inv_dir = Vec3::ONE / ray.direction;
        
        let t1 = (self.min - ray.origin) * inv_dir;
        let t2 = (self.max - ray.origin) * inv_dir;
        
        let t_min = t1.min(t2);
        let t_max = t1.max(t2);
        
        let t_near = t_min.x.max(t_min.y).max(t_min.z);
        let t_far = t_max.x.min(t_max.y).min(t_max.z);
        
        if t_near <= t_far && t_far >= ray.t_min && t_near <= ray.t_max {
            Some(t_near.max(ray.t_min))
        } else {
            None
        }
    }
}

/// Triangle primitive
#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    /// Vertices
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    /// Normals
    pub n0: Vec3,
    pub n1: Vec3,
    pub n2: Vec3,
    /// UVs
    pub uv0: [f32; 2],
    pub uv1: [f32; 2],
    pub uv2: [f32; 2],
}

impl Triangle {
    /// Ray-triangle intersection (Möller–Trumbore)
    #[must_use]
    pub fn intersect(&self, ray: &Ray) -> Option<(f32, f32, f32)> {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);
        
        if a.abs() < 1e-8 {
            return None;
        }
        
        let f = 1.0 / a;
        let s = ray.origin - self.v0;
        let u = f * s.dot(h);
        
        if !(0.0..=1.0).contains(&u) {
            return None;
        }
        
        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);
        
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        
        let t = f * edge2.dot(q);
        
        if t >= ray.t_min && t <= ray.t_max {
            Some((t, u, v))
        } else {
            None
        }
    }

    /// Get AABB
    #[must_use]
    pub fn aabb(&self) -> AABB {
        AABB::from_points(&[self.v0, self.v1, self.v2])
    }

    /// Get interpolated normal
    #[must_use]
    pub fn interpolate_normal(&self, u: f32, v: f32) -> Vec3 {
        (self.n0 * (1.0 - u - v) + self.n1 * u + self.n2 * v).normalize()
    }
}

/// BVH node
#[derive(Debug, Clone)]
pub struct BVHNode {
    /// Bounding box
    pub aabb: AABB,
    /// Left child (None = leaf)
    pub left: Option<Box<BVHNode>>,
    /// Right child
    pub right: Option<Box<BVHNode>>,
    /// Primitive indices (for leaves)
    pub primitives: Vec<u32>,
}

/// Bounding Volume Hierarchy
pub struct BVH {
    /// Root node
    root: Option<BVHNode>,
    /// Triangles
    triangles: Vec<Triangle>,
    /// Build time (ms)
    pub build_time_ms: f32,
}

impl BVH {
    /// Build BVH from triangles
    #[must_use]
    pub fn build(triangles: Vec<Triangle>) -> Self {
        let start = std::time::Instant::now();
        
        if triangles.is_empty() {
            return Self {
                root: None,
                triangles,
                build_time_ms: 0.0,
            };
        }

        let indices: Vec<u32> = (0..triangles.len() as u32).collect();
        let root = Self::build_node(&triangles, indices, 0);
        
        let build_time_ms = start.elapsed().as_secs_f32() * 1000.0;
        
        Self {
            root: Some(root),
            triangles,
            build_time_ms,
        }
    }

    fn build_node(triangles: &[Triangle], indices: Vec<u32>, depth: u32) -> BVHNode {
        // Calculate AABB
        let mut aabb = triangles[indices[0] as usize].aabb();
        for &idx in &indices[1..] {
            aabb = aabb.union(&triangles[idx as usize].aabb());
        }

        // Leaf node
        if indices.len() <= 4 || depth > 32 {
            return BVHNode {
                aabb,
                left: None,
                right: None,
                primitives: indices,
            };
        }

        // Find split axis (largest extent)
        let extent = aabb.max - aabb.min;
        let axis = if extent.x > extent.y && extent.x > extent.z {
            0
        } else if extent.y > extent.z {
            1
        } else {
            2
        };

        // Sort by centroid
        let mut sorted_indices = indices;
        sorted_indices.sort_by(|&a, &b| {
            let ca = triangles[a as usize].aabb().center();
            let cb = triangles[b as usize].aabb().center();
            let va = match axis { 0 => ca.x, 1 => ca.y, _ => ca.z };
            let vb = match axis { 0 => cb.x, 1 => cb.y, _ => cb.z };
            va.partial_cmp(&vb).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Split in half
        let mid = sorted_indices.len() / 2;
        let left_indices = sorted_indices[..mid].to_vec();
        let right_indices = sorted_indices[mid..].to_vec();

        BVHNode {
            aabb,
            left: Some(Box::new(Self::build_node(triangles, left_indices, depth + 1))),
            right: Some(Box::new(Self::build_node(triangles, right_indices, depth + 1))),
            primitives: Vec::new(),
        }
    }

    /// Trace ray through BVH
    #[must_use]
    pub fn trace(&self, ray: &Ray) -> Option<RayHit> {
        self.root.as_ref().and_then(|root| self.trace_node(root, ray))
    }

    fn trace_node(&self, node: &BVHNode, ray: &Ray) -> Option<RayHit> {
        // Check AABB first
        if node.aabb.intersect(ray).is_none() {
            return None;
        }

        // Leaf node - test primitives
        if node.left.is_none() {
            let mut closest: Option<RayHit> = None;
            let mut closest_t = ray.t_max;

            for &idx in &node.primitives {
                let tri = &self.triangles[idx as usize];
                if let Some((t, u, v)) = tri.intersect(ray) {
                    if t < closest_t {
                        closest_t = t;
                        let normal = tri.interpolate_normal(u, v);
                        let front_face = ray.direction.dot(normal) < 0.0;
                        
                        closest = Some(RayHit {
                            position: ray.at(t),
                            normal: if front_face { normal } else { -normal },
                            distance: t,
                            uv: [
                                tri.uv0[0] * (1.0 - u - v) + tri.uv1[0] * u + tri.uv2[0] * v,
                                tri.uv0[1] * (1.0 - u - v) + tri.uv1[1] * u + tri.uv2[1] * v,
                            ],
                            material_id: 0,
                            instance_id: 0,
                            primitive_index: idx,
                            front_face,
                        });
                    }
                }
            }
            return closest;
        }

        // Recurse into children
        let left_hit = node.left.as_ref().and_then(|l| self.trace_node(l, ray));
        let right_hit = node.right.as_ref().and_then(|r| self.trace_node(r, ray));

        match (left_hit, right_hit) {
            (Some(l), Some(r)) => {
                if l.distance < r.distance { Some(l) } else { Some(r) }
            }
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        }
    }

    /// Get triangle count
    #[must_use]
    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }
}

/// Top-Level Acceleration Structure (for instances)
pub struct TLAS {
    /// Instances
    instances: Vec<RTInstance>,
    /// BVH for instances
    bvh: Option<BVH>,
}

/// RT instance
pub struct RTInstance {
    /// BLAS reference
    pub blas_id: u32,
    /// Transform
    pub transform: Mat4,
    /// Inverse transform
    pub inverse_transform: Mat4,
    /// Instance ID
    pub instance_id: u32,
    /// Material ID
    pub material_id: u32,
    /// Mask
    pub mask: u32,
}

/// Ray tracing pipeline
pub struct RayTracingPipeline {
    /// Configuration
    pub config: RTConfig,
    /// BLAS cache
    blas_cache: HashMap<u32, BVH>,
    /// TLAS
    tlas: Option<TLAS>,
    /// Frame count
    frame: u64,
    /// Statistics
    stats: RTStats,
}

/// RT statistics
#[derive(Debug, Clone, Default)]
pub struct RTStats {
    /// Rays traced
    pub rays_traced: u64,
    /// Triangles tested
    pub triangles_tested: u64,
    /// BVH nodes visited
    pub nodes_visited: u64,
    /// Average rays per second
    pub rays_per_second: f64,
}

impl Default for RayTracingPipeline {
    fn default() -> Self {
        Self::new(RTConfig::default())
    }
}

impl RayTracingPipeline {
    /// Create a new RT pipeline
    #[must_use]
    pub fn new(config: RTConfig) -> Self {
        Self {
            config,
            blas_cache: HashMap::new(),
            tlas: None,
            frame: 0,
            stats: RTStats::default(),
        }
    }

    /// Add BLAS (Bottom-Level Acceleration Structure)
    pub fn add_blas(&mut self, id: u32, triangles: Vec<Triangle>) {
        self.blas_cache.insert(id, BVH::build(triangles));
    }

    /// Trace primary ray
    #[must_use]
    pub fn trace_primary(&self, ray: &Ray) -> Option<RayHit> {
        // For now, trace through all BLAS directly
        let mut closest: Option<RayHit> = None;
        let mut closest_t = ray.t_max;

        for bvh in self.blas_cache.values() {
            if let Some(hit) = bvh.trace(ray) {
                if hit.distance < closest_t {
                    closest_t = hit.distance;
                    closest = Some(hit);
                }
            }
        }

        closest
    }

    /// Trace shadow ray
    #[must_use]
    pub fn trace_shadow(&self, origin: Vec3, direction: Vec3, max_distance: f32) -> bool {
        let ray = Ray {
            origin,
            direction: direction.normalize(),
            t_min: 0.001,
            t_max: max_distance,
        };

        // Any hit = shadow
        for bvh in self.blas_cache.values() {
            if bvh.trace(&ray).is_some() {
                return true;
            }
        }

        false
    }

    /// Update frame
    pub fn update(&mut self) {
        self.frame += 1;
    }

    /// Get statistics
    #[must_use]
    pub fn stats(&self) -> &RTStats {
        &self.stats
    }
}
