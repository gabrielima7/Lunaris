//! Gaussian Splatting
//!
//! 3D Gaussian rendering for real-time novel view synthesis.

use glam::{Vec2, Vec3, Vec4, Mat3, Quat};

/// Gaussian splatting system
pub struct GaussianSplatting {
    pub scenes: Vec<GaussianScene>,
    pub renderer: GaussianRenderer,
    pub settings: GaussianSettings,
}

/// Gaussian scene
pub struct GaussianScene {
    pub id: u64,
    pub name: String,
    pub gaussians: Vec<Gaussian3D>,
    pub sh_degree: u32,
    pub bounds: SceneBounds,
}

/// 3D Gaussian
pub struct Gaussian3D {
    pub position: Vec3,
    pub covariance: Mat3,
    pub opacity: f32,
    pub color_sh: Vec<Vec3>,  // Spherical harmonics coefficients
    pub scale: Vec3,
    pub rotation: Quat,
}

impl Gaussian3D {
    pub fn new(position: Vec3, scale: Vec3, rotation: Quat, opacity: f32, color: Vec3) -> Self {
        let covariance = Self::compute_covariance(scale, rotation);
        Self { position, covariance, opacity, color_sh: vec![color], scale, rotation }
    }

    fn compute_covariance(scale: Vec3, rotation: Quat) -> Mat3 {
        let r = Mat3::from_quat(rotation);
        let s = Mat3::from_diagonal(scale * scale);
        r * s * r.transpose()
    }

    pub fn project_to_2d(&self, view_matrix: Mat3, focal: Vec2) -> Gaussian2D {
        let cov_2d = self.compute_cov_2d(view_matrix, focal);
        Gaussian2D { mean: Vec2::ZERO, covariance: cov_2d, opacity: self.opacity, color: self.color_sh.first().copied().unwrap_or(Vec3::ONE) }
    }

    fn compute_cov_2d(&self, view_matrix: Mat3, focal: Vec2) -> Mat2 {
        let j = Mat3::from_cols(
            Vec3::new(focal.x, 0.0, 0.0),
            Vec3::new(0.0, focal.y, 0.0),
            Vec3::ZERO,
        );
        let w = view_matrix;
        let t = j * w;
        let cov = t * self.covariance * t.transpose();
        Mat2 { m00: cov.x_axis.x, m01: cov.x_axis.y, m10: cov.y_axis.x, m11: cov.y_axis.y }
    }

    pub fn evaluate_sh(&self, direction: Vec3) -> Vec3 {
        if self.color_sh.is_empty() { return Vec3::ONE; }
        
        let mut result = self.color_sh[0] * 0.28209479177;  // Y_0^0
        
        if self.color_sh.len() > 1 {
            result += self.color_sh[1] * 0.4886025119 * direction.y;  // Y_1^-1
        }
        if self.color_sh.len() > 2 {
            result += self.color_sh[2] * 0.4886025119 * direction.z;  // Y_1^0
        }
        if self.color_sh.len() > 3 {
            result += self.color_sh[3] * 0.4886025119 * direction.x;  // Y_1^1
        }
        
        result.max(Vec3::ZERO)
    }
}

/// 2D Gaussian
pub struct Gaussian2D {
    pub mean: Vec2,
    pub covariance: Mat2,
    pub opacity: f32,
    pub color: Vec3,
}

/// 2x2 Matrix
#[derive(Clone, Copy)]
pub struct Mat2 { pub m00: f32, pub m01: f32, pub m10: f32, pub m11: f32 }

impl Mat2 {
    pub fn inverse(&self) -> Option<Mat2> {
        let det = self.m00 * self.m11 - self.m01 * self.m10;
        if det.abs() < 1e-10 { return None; }
        let inv_det = 1.0 / det;
        Some(Mat2 { m00: self.m11 * inv_det, m01: -self.m01 * inv_det, m10: -self.m10 * inv_det, m11: self.m00 * inv_det })
    }
}

/// Scene bounds
pub struct SceneBounds {
    pub min: Vec3,
    pub max: Vec3,
}

/// Gaussian renderer
pub struct GaussianRenderer {
    pub tile_size: u32,
    pub max_gaussians_per_tile: u32,
    pub depth_sorting: bool,
    pub culling_enabled: bool,
}

impl Default for GaussianRenderer {
    fn default() -> Self {
        Self { tile_size: 16, max_gaussians_per_tile: 256, depth_sorting: true, culling_enabled: true }
    }
}

/// Gaussian settings
pub struct GaussianSettings {
    pub resolution: (u32, u32),
    pub background_color: Vec3,
    pub sh_degree: u32,
    pub antialiasing: bool,
}

impl Default for GaussianSettings {
    fn default() -> Self {
        Self { resolution: (1920, 1080), background_color: Vec3::ZERO, sh_degree: 3, antialiasing: true }
    }
}

impl GaussianSplatting {
    pub fn new() -> Self {
        Self { scenes: Vec::new(), renderer: GaussianRenderer::default(), settings: GaussianSettings::default() }
    }

    pub fn create_scene(&mut self, name: &str) -> usize {
        let id = self.scenes.len();
        self.scenes.push(GaussianScene { id: id as u64, name: name.into(), gaussians: Vec::new(), sh_degree: 3, bounds: SceneBounds { min: Vec3::NEG_ONE * 10.0, max: Vec3::ONE * 10.0 } });
        id
    }

    pub fn add_gaussian(&mut self, scene_id: usize, position: Vec3, scale: Vec3, rotation: Quat, opacity: f32, color: Vec3) {
        if let Some(scene) = self.scenes.get_mut(scene_id) {
            scene.gaussians.push(Gaussian3D::new(position, scale, rotation, opacity, color));
        }
    }

    pub fn render(&self, scene_id: usize, camera: &GaussianCamera) -> Vec<Vec4> {
        let (w, h) = self.settings.resolution;
        let mut framebuffer = vec![self.settings.background_color.extend(1.0); (w * h) as usize];

        let Some(scene) = self.scenes.get(scene_id) else { return framebuffer; };

        // Sort gaussians by depth
        let mut sorted: Vec<(usize, f32)> = scene.gaussians.iter().enumerate()
            .map(|(i, g)| (i, (g.position - camera.position).dot(camera.forward())))
            .collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Render each gaussian
        for (idx, _depth) in sorted {
            let gaussian = &scene.gaussians[idx];
            self.splat_gaussian(&mut framebuffer, w, h, gaussian, camera);
        }

        framebuffer
    }

    fn splat_gaussian(&self, framebuffer: &mut [Vec4], w: u32, h: u32, gaussian: &Gaussian3D, camera: &GaussianCamera) {
        let screen_pos = camera.project(gaussian.position);
        if screen_pos.x < -1.0 || screen_pos.x > 1.0 || screen_pos.y < -1.0 || screen_pos.y > 1.0 { return; }

        let px = ((screen_pos.x * 0.5 + 0.5) * w as f32) as i32;
        let py = ((screen_pos.y * 0.5 + 0.5) * h as f32) as i32;
        let radius = (gaussian.scale.max_element() * camera.focal.x / (gaussian.position - camera.position).length()) as i32;
        let radius = radius.clamp(1, 50);

        let view_dir = (camera.position - gaussian.position).normalize();
        let color = gaussian.evaluate_sh(view_dir);

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let x = px + dx;
                let y = py + dy;
                if x < 0 || x >= w as i32 || y < 0 || y >= h as i32 { continue; }

                let dist_sq = (dx * dx + dy * dy) as f32;
                let sigma_sq = (radius as f32 * 0.5).powi(2);
                let weight = (-dist_sq / (2.0 * sigma_sq)).exp() * gaussian.opacity;

                let idx = (y as u32 * w + x as u32) as usize;
                let old = framebuffer[idx];
                framebuffer[idx] = Vec4::new(
                    old.x * (1.0 - weight) + color.x * weight,
                    old.y * (1.0 - weight) + color.y * weight,
                    old.z * (1.0 - weight) + color.z * weight,
                    1.0,
                );
            }
        }
    }
}

/// Gaussian camera
pub struct GaussianCamera {
    pub position: Vec3,
    pub rotation: Quat,
    pub fov: f32,
    pub aspect: f32,
    pub focal: Vec2,
}

impl GaussianCamera {
    pub fn project(&self, point: Vec3) -> Vec2 {
        let local = self.rotation.inverse() * (point - self.position);
        if local.z <= 0.0 { return Vec2::new(-999.0, -999.0); }
        Vec2::new(local.x / local.z * self.focal.x, local.y / local.z * self.focal.y)
    }

    pub fn forward(&self) -> Vec3 { self.rotation * Vec3::NEG_Z }
}
