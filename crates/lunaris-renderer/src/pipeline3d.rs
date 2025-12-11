//! 3D Render Pipeline
//!
//! Handles 3D mesh rendering with PBR materials.

use super::{Camera3D, CameraUniform, Vertex3D};
use crate::material::{Material, MaterialId};
use crate::mesh::{Mesh, MeshId};
use lunaris_core::math::{Color, Vec3};
use std::collections::HashMap;
use wgpu::*;

/// 3D render instance data
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MeshInstance {
    /// Model matrix (4x4)
    pub model: [[f32; 4]; 4],
    /// Normal matrix (3x3, padded to 4x3)
    pub normal: [[f32; 4]; 3],
}

unsafe impl bytemuck::Pod for MeshInstance {}
unsafe impl bytemuck::Zeroable for MeshInstance {}

impl Default for MeshInstance {
    fn default() -> Self {
        Self::identity()
    }
}

impl MeshInstance {
    /// Create identity transform
    #[must_use]
    pub fn identity() -> Self {
        Self {
            model: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            normal: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
        }
    }

    /// Create from position, rotation (euler), and scale
    #[must_use]
    pub fn from_transform(position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        // Rotation matrices
        let (sx, cx) = rotation.x.sin_cos();
        let (sy, cy) = rotation.y.sin_cos();
        let (sz, cz) = rotation.z.sin_cos();

        // Combined rotation (ZYX order)
        let r00 = cy * cz;
        let r01 = cy * sz;
        let r02 = -sy;
        let r10 = sx * sy * cz - cx * sz;
        let r11 = sx * sy * sz + cx * cz;
        let r12 = sx * cy;
        let r20 = cx * sy * cz + sx * sz;
        let r21 = cx * sy * sz - sx * cz;
        let r22 = cx * cy;

        Self {
            model: [
                [r00 * scale.x, r01 * scale.x, r02 * scale.x, 0.0],
                [r10 * scale.y, r11 * scale.y, r12 * scale.y, 0.0],
                [r20 * scale.z, r21 * scale.z, r22 * scale.z, 0.0],
                [position.x, position.y, position.z, 1.0],
            ],
            normal: [
                [r00, r01, r02, 0.0],
                [r10, r11, r12, 0.0],
                [r20, r21, r22, 0.0],
            ],
        }
    }
}

/// Light types
#[derive(Debug, Clone, Copy)]
pub enum LightType {
    /// Directional light (sun)
    Directional,
    /// Point light
    Point,
    /// Spot light
    Spot,
}

/// Light data
#[derive(Debug, Clone, Copy)]
pub struct Light {
    /// Light type
    pub light_type: LightType,
    /// Position (for point/spot)
    pub position: Vec3,
    /// Direction (for directional/spot)
    pub direction: Vec3,
    /// Color
    pub color: Color,
    /// Intensity
    pub intensity: f32,
    /// Range (for point/spot)
    pub range: f32,
    /// Spot angle (for spot)
    pub spot_angle: f32,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            light_type: LightType::Directional,
            position: Vec3::ZERO,
            direction: Vec3::new(0.0, -1.0, -1.0).normalize(),
            color: Color::WHITE,
            intensity: 1.0,
            range: 10.0,
            spot_angle: 45.0_f32.to_radians(),
        }
    }
}

impl Light {
    /// Create a directional light
    #[must_use]
    pub fn directional(direction: Vec3, color: Color, intensity: f32) -> Self {
        Self {
            light_type: LightType::Directional,
            direction: direction.normalize(),
            color,
            intensity,
            ..Default::default()
        }
    }

    /// Create a point light
    #[must_use]
    pub fn point(position: Vec3, color: Color, intensity: f32, range: f32) -> Self {
        Self {
            light_type: LightType::Point,
            position,
            color,
            intensity,
            range,
            ..Default::default()
        }
    }

    /// Create a spot light
    #[must_use]
    pub fn spot(position: Vec3, direction: Vec3, color: Color, intensity: f32, angle: f32) -> Self {
        Self {
            light_type: LightType::Spot,
            position,
            direction: direction.normalize(),
            color,
            intensity,
            spot_angle: angle,
            ..Default::default()
        }
    }
}

/// GPU light uniform
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LightUniform {
    /// Position (padded)
    pub position: [f32; 4],
    /// Direction (padded)
    pub direction: [f32; 4],
    /// Color and intensity
    pub color_intensity: [f32; 4],
    /// Range and spot angle
    pub params: [f32; 4],
}

unsafe impl bytemuck::Pod for LightUniform {}
unsafe impl bytemuck::Zeroable for LightUniform {}

impl From<&Light> for LightUniform {
    fn from(light: &Light) -> Self {
        let type_id = match light.light_type {
            LightType::Directional => 0.0,
            LightType::Point => 1.0,
            LightType::Spot => 2.0,
        };
        Self {
            position: [light.position.x, light.position.y, light.position.z, type_id],
            direction: [light.direction.x, light.direction.y, light.direction.z, 0.0],
            color_intensity: [light.color.r, light.color.g, light.color.b, light.intensity],
            params: [light.range, light.spot_angle, 0.0, 0.0],
        }
    }
}

/// Render command for 3D
#[derive(Debug, Clone)]
pub struct RenderCommand3D {
    /// Mesh ID
    pub mesh_id: MeshId,
    /// Material ID
    pub material_id: MaterialId,
    /// Transform instance
    pub instance: MeshInstance,
    /// Cast shadows
    pub cast_shadows: bool,
    /// Receive shadows
    pub receive_shadows: bool,
}

/// 3D Render pipeline
pub struct Render3D {
    /// Render commands
    commands: Vec<RenderCommand3D>,
    /// Lights
    lights: Vec<Light>,
    /// Camera
    camera: Camera3D,
    /// Ambient color
    ambient_color: Color,
    /// Environment map enabled
    env_map_enabled: bool,
    /// Statistics
    stats: RenderStats3D,
}

/// 3D Render statistics
#[derive(Debug, Clone, Copy, Default)]
pub struct RenderStats3D {
    /// Draw calls
    pub draw_calls: usize,
    /// Triangles rendered
    pub triangles: usize,
    /// Vertices processed
    pub vertices: usize,
    /// Lights active
    pub lights: usize,
}

impl Default for Render3D {
    fn default() -> Self {
        Self::new()
    }
}

impl Render3D {
    /// Create a new 3D renderer
    #[must_use]
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            lights: Vec::new(),
            camera: Camera3D::default(),
            ambient_color: Color::new(0.1, 0.1, 0.15, 1.0),
            env_map_enabled: false,
            stats: RenderStats3D::default(),
        }
    }

    /// Set the camera
    pub fn set_camera(&mut self, camera: Camera3D) {
        self.camera = camera;
    }

    /// Get the camera
    #[must_use]
    pub fn camera(&self) -> &Camera3D {
        &self.camera
    }

    /// Set ambient color
    pub fn set_ambient(&mut self, color: Color) {
        self.ambient_color = color;
    }

    /// Begin a new frame
    pub fn begin_frame(&mut self) {
        self.commands.clear();
        self.lights.clear();
        self.stats = RenderStats3D::default();
    }

    /// Add a light
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    /// Submit a mesh for rendering
    pub fn draw_mesh(
        &mut self,
        mesh_id: MeshId,
        material_id: MaterialId,
        transform: MeshInstance,
    ) {
        self.commands.push(RenderCommand3D {
            mesh_id,
            material_id,
            instance: transform,
            cast_shadows: true,
            receive_shadows: true,
        });
    }

    /// Submit a mesh with position only
    pub fn draw_mesh_at(
        &mut self,
        mesh_id: MeshId,
        material_id: MaterialId,
        position: Vec3,
    ) {
        self.draw_mesh(
            mesh_id,
            material_id,
            MeshInstance::from_transform(position, Vec3::ZERO, Vec3::ONE),
        );
    }

    /// End frame and get statistics
    #[must_use]
    pub fn end_frame(&mut self) -> RenderStats3D {
        self.stats.draw_calls = self.commands.len();
        self.stats.lights = self.lights.len();
        self.stats
    }

    /// Get render commands
    #[must_use]
    pub fn commands(&self) -> &[RenderCommand3D] {
        &self.commands
    }

    /// Get lights
    #[must_use]
    pub fn lights(&self) -> &[Light] {
        &self.lights
    }

    /// Sort commands by material for batching
    pub fn sort_by_material(&mut self) {
        self.commands.sort_by(|a, b| a.material_id.0.raw().cmp(&b.material_id.0.raw()));
    }

    /// Sort commands front-to-back for opaque
    pub fn sort_front_to_back(&mut self) {
        let camera_pos = self.camera.position;
        self.commands.sort_by(|a, b| {
            let dist_a = Vec3::new(a.instance.model[3][0], a.instance.model[3][1], a.instance.model[3][2])
                .distance_squared(camera_pos);
            let dist_b = Vec3::new(b.instance.model[3][0], b.instance.model[3][1], b.instance.model[3][2])
                .distance_squared(camera_pos);
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Sort commands back-to-front for transparent
    pub fn sort_back_to_front(&mut self) {
        let camera_pos = self.camera.position;
        self.commands.sort_by(|a, b| {
            let dist_a = Vec3::new(a.instance.model[3][0], a.instance.model[3][1], a.instance.model[3][2])
                .distance_squared(camera_pos);
            let dist_b = Vec3::new(b.instance.model[3][0], b.instance.model[3][1], b.instance.model[3][2])
                .distance_squared(camera_pos);
            dist_b.partial_cmp(&dist_a).unwrap_or(std::cmp::Ordering::Equal)
        });
    }
}
