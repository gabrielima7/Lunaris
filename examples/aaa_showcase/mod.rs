//! AAA Visual Showcase
//!
//! Demonstrates Lumen GI, Nanite virtual geometry, and all AAA rendering features.
//! This is the "wow factor" demo that shows Lunaris can compete with Unreal.

use glam::{Vec3, Vec4, Mat4, Quat};
use std::f32::consts::PI;

/// AAA Visual Showcase Demo
pub struct AAAShowcase {
    /// Scene state
    pub scene: ShowcaseScene,
    /// Camera
    pub camera: ShowcaseCamera,
    /// Time
    pub time: f32,
    /// Current showcase
    pub current_demo: ShowcaseDemo,
}

/// Available demos
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowcaseDemo {
    /// Lumen Global Illumination
    LumenGI,
    /// Nanite Virtual Geometry
    NaniteGeometry,
    /// Ray-Traced Reflections
    RTReflections,
    /// MetaHuman Quality Characters
    MetaHumans,
    /// Destruction Physics
    ChaosDestruction,
    /// Procedural World
    ProceduralWorld,
    /// Full Scene (all combined)
    FullScene,
}

/// Showcase scene
pub struct ShowcaseScene {
    /// Meshes
    pub meshes: Vec<ShowcaseMesh>,
    /// Lights
    pub lights: Vec<ShowcaseLight>,
    /// Environment
    pub environment: Environment,
    /// Post-processing
    pub post_process: PostProcessSettings,
}

/// Showcase mesh
pub struct ShowcaseMesh {
    pub name: String,
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub mesh_type: MeshType,
    pub material: MaterialData,
    /// Nanite enabled
    pub use_nanite: bool,
    /// Triangle count (for Nanite demo)
    pub triangle_count: u64,
}

/// Mesh type
pub enum MeshType {
    Primitive(PrimitiveType),
    StatueBust,
    ArchitecturalColumn,
    Tree,
    Rock,
    Building,
    Vehicle,
    Character,
    Terrain,
    Custom(String),
}

/// Primitive types
pub enum PrimitiveType {
    Sphere,
    Cube,
    Cylinder,
    Plane,
    Torus,
}

/// Material data
pub struct MaterialData {
    pub name: String,
    pub base_color: Vec4,
    pub metallic: f32,
    pub roughness: f32,
    pub emission: Vec3,
    pub emission_strength: f32,
    pub normal_strength: f32,
    pub ao_strength: f32,
    /// For glass, water, etc.
    pub transmission: f32,
    pub ior: f32,
    /// Subsurface scattering
    pub subsurface: f32,
    pub subsurface_color: Vec3,
}

impl MaterialData {
    /// Gold material
    pub fn gold() -> Self {
        Self {
            name: "Gold".into(),
            base_color: Vec4::new(1.0, 0.766, 0.336, 1.0),
            metallic: 1.0,
            roughness: 0.3,
            emission: Vec3::ZERO,
            emission_strength: 0.0,
            normal_strength: 1.0,
            ao_strength: 1.0,
            transmission: 0.0,
            ior: 1.5,
            subsurface: 0.0,
            subsurface_color: Vec3::ZERO,
        }
    }

    /// Chrome material
    pub fn chrome() -> Self {
        Self {
            name: "Chrome".into(),
            base_color: Vec4::new(0.95, 0.95, 0.95, 1.0),
            metallic: 1.0,
            roughness: 0.05,
            emission: Vec3::ZERO,
            emission_strength: 0.0,
            normal_strength: 1.0,
            ao_strength: 1.0,
            transmission: 0.0,
            ior: 1.5,
            subsurface: 0.0,
            subsurface_color: Vec3::ZERO,
        }
    }

    /// Marble material
    pub fn marble() -> Self {
        Self {
            name: "Marble".into(),
            base_color: Vec4::new(0.95, 0.93, 0.88, 1.0),
            metallic: 0.0,
            roughness: 0.2,
            emission: Vec3::ZERO,
            emission_strength: 0.0,
            normal_strength: 0.5,
            ao_strength: 1.0,
            transmission: 0.0,
            ior: 1.5,
            subsurface: 0.1,
            subsurface_color: Vec3::new(1.0, 0.9, 0.8),
        }
    }

    /// Emissive neon
    pub fn neon(color: Vec3) -> Self {
        Self {
            name: "Neon".into(),
            base_color: Vec4::new(color.x, color.y, color.z, 1.0),
            metallic: 0.0,
            roughness: 0.5,
            emission: color,
            emission_strength: 10.0,
            normal_strength: 0.0,
            ao_strength: 0.0,
            transmission: 0.0,
            ior: 1.5,
            subsurface: 0.0,
            subsurface_color: Vec3::ZERO,
        }
    }

    /// Glass material
    pub fn glass() -> Self {
        Self {
            name: "Glass".into(),
            base_color: Vec4::new(1.0, 1.0, 1.0, 0.1),
            metallic: 0.0,
            roughness: 0.0,
            emission: Vec3::ZERO,
            emission_strength: 0.0,
            normal_strength: 1.0,
            ao_strength: 0.0,
            transmission: 0.95,
            ior: 1.52,
            subsurface: 0.0,
            subsurface_color: Vec3::ZERO,
        }
    }

    /// Skin material
    pub fn skin() -> Self {
        Self {
            name: "Skin".into(),
            base_color: Vec4::new(0.8, 0.6, 0.5, 1.0),
            metallic: 0.0,
            roughness: 0.5,
            emission: Vec3::ZERO,
            emission_strength: 0.0,
            normal_strength: 1.0,
            ao_strength: 1.0,
            transmission: 0.0,
            ior: 1.4,
            subsurface: 0.5,
            subsurface_color: Vec3::new(1.0, 0.2, 0.1),
        }
    }
}

/// Light
pub struct ShowcaseLight {
    pub light_type: LightType,
    pub color: Vec3,
    pub intensity: f32,
    pub position: Vec3,
    pub direction: Vec3,
    pub radius: f32,
    pub cast_shadows: bool,
    pub volumetric: bool,
}

/// Light type
pub enum LightType {
    Directional,
    Point,
    Spot { inner_angle: f32, outer_angle: f32 },
    Area { width: f32, height: f32 },
    IES { profile: String },
}

/// Environment settings
pub struct Environment {
    pub sky_type: SkyType,
    pub sun_direction: Vec3,
    pub sun_intensity: f32,
    pub sun_color: Vec3,
    pub ambient_intensity: f32,
    pub fog: FogSettings,
    pub clouds: CloudSettings,
}

/// Sky type
pub enum SkyType {
    Procedural,
    HDRI(String),
    SolidColor(Vec3),
}

/// Fog settings
pub struct FogSettings {
    pub enabled: bool,
    pub density: f32,
    pub height_falloff: f32,
    pub color: Vec3,
    pub start_distance: f32,
    pub volumetric: bool,
}

/// Cloud settings
pub struct CloudSettings {
    pub enabled: bool,
    pub coverage: f32,
    pub altitude: f32,
    pub thickness: f32,
    pub wind_speed: f32,
}

/// Post-processing settings
pub struct PostProcessSettings {
    pub exposure: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub vignette: f32,
    pub bloom_intensity: f32,
    pub bloom_threshold: f32,
    pub chromatic_aberration: f32,
    pub film_grain: f32,
    pub dof_enabled: bool,
    pub dof_focus_distance: f32,
    pub dof_aperture: f32,
    pub motion_blur: f32,
    pub tonemapping: Tonemapping,
    pub color_grading: ColorGrading,
}

/// Tonemapping
pub enum Tonemapping {
    None,
    Reinhard,
    ACES,
    AgX,
    Neutral,
}

/// Color grading
pub struct ColorGrading {
    pub temperature: f32,
    pub tint: f32,
    pub shadows: Vec3,
    pub midtones: Vec3,
    pub highlights: Vec3,
    pub lut: Option<String>,
}

/// Camera
pub struct ShowcaseCamera {
    pub position: Vec3,
    pub target: Vec3,
    pub fov: f32,
    pub orbit_angle: f32,
    pub orbit_height: f32,
    pub orbit_distance: f32,
    pub auto_orbit: bool,
    pub orbit_speed: f32,
}

impl ShowcaseCamera {
    pub fn update(&mut self, dt: f32) {
        if self.auto_orbit {
            self.orbit_angle += self.orbit_speed * dt;
            
            self.position = self.target + Vec3::new(
                self.orbit_angle.cos() * self.orbit_distance,
                self.orbit_height,
                self.orbit_angle.sin() * self.orbit_distance,
            );
        }
    }
}

impl AAAShowcase {
    /// Create Lumen GI demo
    pub fn lumen_gi_demo() -> Self {
        let mut scene = ShowcaseScene::new();

        // Cornell box style room for GI demonstration
        // Back wall - emissive for color bleeding
        scene.add_mesh(ShowcaseMesh {
            name: "Back Wall".into(),
            position: Vec3::new(0.0, 5.0, -10.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(20.0, 10.0, 0.5),
            mesh_type: MeshType::Primitive(PrimitiveType::Cube),
            material: MaterialData {
                name: "White Wall".into(),
                base_color: Vec4::new(0.9, 0.9, 0.9, 1.0),
                metallic: 0.0,
                roughness: 0.9,
                ..Default::default()
            },
            use_nanite: false,
            triangle_count: 12,
        });

        // Left wall - RED for color bleeding demo
        scene.add_mesh(ShowcaseMesh {
            name: "Left Wall".into(),
            position: Vec3::new(-10.0, 5.0, 0.0),
            rotation: Quat::from_rotation_y(PI / 2.0),
            scale: Vec3::new(20.0, 10.0, 0.5),
            mesh_type: MeshType::Primitive(PrimitiveType::Cube),
            material: MaterialData {
                name: "Red Wall".into(),
                base_color: Vec4::new(0.9, 0.1, 0.1, 1.0),
                metallic: 0.0,
                roughness: 0.9,
                ..Default::default()
            },
            use_nanite: false,
            triangle_count: 12,
        });

        // Right wall - GREEN for color bleeding demo
        scene.add_mesh(ShowcaseMesh {
            name: "Right Wall".into(),
            position: Vec3::new(10.0, 5.0, 0.0),
            rotation: Quat::from_rotation_y(-PI / 2.0),
            scale: Vec3::new(20.0, 10.0, 0.5),
            mesh_type: MeshType::Primitive(PrimitiveType::Cube),
            material: MaterialData {
                name: "Green Wall".into(),
                base_color: Vec4::new(0.1, 0.9, 0.1, 1.0),
                metallic: 0.0,
                roughness: 0.9,
                ..Default::default()
            },
            use_nanite: false,
            triangle_count: 12,
        });

        // Reflective spheres to show indirect lighting
        for i in 0..3 {
            let x = (i as f32 - 1.0) * 5.0;
            scene.add_mesh(ShowcaseMesh {
                name: format!("Sphere {}", i),
                position: Vec3::new(x, 2.0, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::splat(2.0),
                mesh_type: MeshType::Primitive(PrimitiveType::Sphere),
                material: MaterialData::chrome(),
                use_nanite: false,
                triangle_count: 2000,
            });
        }

        // Area light
        scene.add_light(ShowcaseLight {
            light_type: LightType::Area { width: 4.0, height: 4.0 },
            color: Vec3::new(1.0, 0.95, 0.9),
            intensity: 50.0,
            position: Vec3::new(0.0, 9.5, 0.0),
            direction: Vec3::NEG_Y,
            radius: 0.0,
            cast_shadows: true,
            volumetric: false,
        });

        Self {
            scene,
            camera: ShowcaseCamera {
                position: Vec3::new(0.0, 5.0, 15.0),
                target: Vec3::new(0.0, 3.0, 0.0),
                fov: 60.0,
                orbit_angle: 0.0,
                orbit_height: 5.0,
                orbit_distance: 15.0,
                auto_orbit: true,
                orbit_speed: 0.1,
            },
            time: 0.0,
            current_demo: ShowcaseDemo::LumenGI,
        }
    }

    /// Create Nanite demo - massive geometry
    pub fn nanite_demo() -> Self {
        let mut scene = ShowcaseScene::new();

        // High-poly statue - millions of triangles
        scene.add_mesh(ShowcaseMesh {
            name: "Statue".into(),
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(3.0),
            mesh_type: MeshType::StatueBust,
            material: MaterialData::marble(),
            use_nanite: true,
            triangle_count: 50_000_000, // 50 million triangles!
        });

        // Array of detailed columns
        for i in 0..20 {
            let angle = (i as f32 / 20.0) * PI * 2.0;
            let radius = 15.0;
            scene.add_mesh(ShowcaseMesh {
                name: format!("Column {}", i),
                position: Vec3::new(
                    angle.cos() * radius,
                    0.0,
                    angle.sin() * radius,
                ),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(1.0, 8.0, 1.0),
                mesh_type: MeshType::ArchitecturalColumn,
                material: MaterialData::marble(),
                use_nanite: true,
                triangle_count: 2_000_000, // 2 million per column
            });
        }

        // Stats display
        let total_tris: u64 = scene.meshes.iter().map(|m| m.triangle_count).sum();
        println!("🔷 Nanite Demo: {} million triangles", total_tris / 1_000_000);

        // Dramatic lighting
        scene.add_light(ShowcaseLight {
            light_type: LightType::Directional,
            color: Vec3::new(1.0, 0.9, 0.8),
            intensity: 5.0,
            position: Vec3::ZERO,
            direction: Vec3::new(-0.5, -1.0, -0.3).normalize(),
            radius: 0.0,
            cast_shadows: true,
            volumetric: true,
        });

        Self {
            scene,
            camera: ShowcaseCamera {
                position: Vec3::new(20.0, 10.0, 20.0),
                target: Vec3::new(0.0, 4.0, 0.0),
                fov: 50.0,
                orbit_angle: 0.0,
                orbit_height: 10.0,
                orbit_distance: 25.0,
                auto_orbit: true,
                orbit_speed: 0.15,
            },
            time: 0.0,
            current_demo: ShowcaseDemo::NaniteGeometry,
        }
    }

    /// Create full AAA scene
    pub fn full_scene() -> Self {
        let mut scene = ShowcaseScene::new();

        // Ground with Nanite detail
        scene.add_mesh(ShowcaseMesh {
            name: "Terrain".into(),
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::new(100.0, 1.0, 100.0),
            mesh_type: MeshType::Terrain,
            material: MaterialData {
                name: "Ground".into(),
                base_color: Vec4::new(0.3, 0.25, 0.2, 1.0),
                metallic: 0.0,
                roughness: 0.9,
                ..Default::default()
            },
            use_nanite: true,
            triangle_count: 10_000_000,
        });

        // Futuristic building
        scene.add_mesh(ShowcaseMesh {
            name: "Building".into(),
            position: Vec3::new(0.0, 0.0, -30.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(20.0, 50.0, 20.0),
            mesh_type: MeshType::Building,
            material: MaterialData {
                name: "Building Glass".into(),
                base_color: Vec4::new(0.1, 0.15, 0.2, 0.3),
                metallic: 0.8,
                roughness: 0.1,
                transmission: 0.5,
                ..Default::default()
            },
            use_nanite: true,
            triangle_count: 5_000_000,
        });

        // Neon signs
        for (i, color) in [
            Vec3::new(1.0, 0.0, 0.5),  // Pink
            Vec3::new(0.0, 1.0, 1.0),  // Cyan
            Vec3::new(1.0, 0.5, 0.0),  // Orange
        ].iter().enumerate() {
            scene.add_mesh(ShowcaseMesh {
                name: format!("Neon {}", i),
                position: Vec3::new(-15.0 + i as f32 * 15.0, 8.0, -25.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(8.0, 2.0, 0.5),
                mesh_type: MeshType::Primitive(PrimitiveType::Cube),
                material: MaterialData::neon(*color),
                use_nanite: false,
                triangle_count: 12,
            });
        }

        // Vehicle
        scene.add_mesh(ShowcaseMesh {
            name: "Vehicle".into(),
            position: Vec3::new(15.0, 0.0, 10.0),
            rotation: Quat::from_rotation_y(-0.3),
            scale: Vec3::splat(2.0),
            mesh_type: MeshType::Vehicle,
            material: MaterialData {
                name: "Car Paint".into(),
                base_color: Vec4::new(0.8, 0.1, 0.1, 1.0),
                metallic: 0.9,
                roughness: 0.2,
                ..Default::default()
            },
            use_nanite: true,
            triangle_count: 500_000,
        });

        // MetaHuman character
        scene.add_mesh(ShowcaseMesh {
            name: "Character".into(),
            position: Vec3::new(-5.0, 0.0, 5.0),
            rotation: Quat::from_rotation_y(0.5),
            scale: Vec3::splat(1.0),
            mesh_type: MeshType::Character,
            material: MaterialData::skin(),
            use_nanite: true,
            triangle_count: 100_000,
        });

        // Rain particles (would be particle system)
        // Volumetric fog
        scene.environment.fog = FogSettings {
            enabled: true,
            density: 0.02,
            height_falloff: 0.5,
            color: Vec3::new(0.05, 0.05, 0.1),
            start_distance: 10.0,
            volumetric: true,
        };

        // Night time with neon
        scene.environment.sky_type = SkyType::Procedural;
        scene.environment.sun_intensity = 0.1;
        scene.environment.ambient_intensity = 0.05;

        // City lights
        for i in 0..10 {
            let x = (i as f32 - 5.0) * 10.0;
            scene.add_light(ShowcaseLight {
                light_type: LightType::Point,
                color: Vec3::new(1.0, 0.8, 0.6),
                intensity: 100.0,
                position: Vec3::new(x, 6.0, 0.0),
                direction: Vec3::ZERO,
                radius: 15.0,
                cast_shadows: i % 2 == 0, // Every other light casts shadows
                volumetric: true,
            });
        }

        // Cinematic post-processing
        scene.post_process = PostProcessSettings {
            exposure: 1.2,
            contrast: 1.1,
            saturation: 1.1,
            vignette: 0.3,
            bloom_intensity: 0.5,
            bloom_threshold: 1.0,
            chromatic_aberration: 0.01,
            film_grain: 0.05,
            dof_enabled: true,
            dof_focus_distance: 20.0,
            dof_aperture: 2.8,
            motion_blur: 0.5,
            tonemapping: Tonemapping::ACES,
            color_grading: ColorGrading {
                temperature: -5.0, // Cooler
                tint: 0.0,
                shadows: Vec3::new(0.1, 0.1, 0.2),
                midtones: Vec3::ONE,
                highlights: Vec3::new(1.0, 0.95, 0.9),
                lut: None,
            },
        };

        Self {
            scene,
            camera: ShowcaseCamera {
                position: Vec3::new(30.0, 8.0, 30.0),
                target: Vec3::new(0.0, 5.0, -10.0),
                fov: 45.0,
                orbit_angle: 0.0,
                orbit_height: 8.0,
                orbit_distance: 40.0,
                auto_orbit: true,
                orbit_speed: 0.05,
            },
            time: 0.0,
            current_demo: ShowcaseDemo::FullScene,
        }
    }

    /// Update showcase
    pub fn update(&mut self, dt: f32) {
        self.time += dt;
        self.camera.update(dt);

        // Animate neon lights
        for mesh in &mut self.scene.meshes {
            if mesh.name.starts_with("Neon") {
                let pulse = (self.time * 3.0).sin() * 0.5 + 0.5;
                mesh.material.emission_strength = 5.0 + pulse * 10.0;
            }
        }
    }

    /// Get stats
    pub fn stats(&self) -> ShowcaseStats {
        let total_triangles: u64 = self.scene.meshes.iter()
            .map(|m| m.triangle_count)
            .sum();
        
        let nanite_triangles: u64 = self.scene.meshes.iter()
            .filter(|m| m.use_nanite)
            .map(|m| m.triangle_count)
            .sum();

        ShowcaseStats {
            total_meshes: self.scene.meshes.len(),
            total_lights: self.scene.lights.len(),
            total_triangles,
            nanite_triangles,
            nanite_percentage: (nanite_triangles as f32 / total_triangles as f32) * 100.0,
        }
    }
}

/// Showcase stats
#[derive(Debug)]
pub struct ShowcaseStats {
    pub total_meshes: usize,
    pub total_lights: usize,
    pub total_triangles: u64,
    pub nanite_triangles: u64,
    pub nanite_percentage: f32,
}

impl ShowcaseScene {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            lights: Vec::new(),
            environment: Environment::default(),
            post_process: PostProcessSettings::default(),
        }
    }

    pub fn add_mesh(&mut self, mesh: ShowcaseMesh) {
        self.meshes.push(mesh);
    }

    pub fn add_light(&mut self, light: ShowcaseLight) {
        self.lights.push(light);
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            sky_type: SkyType::Procedural,
            sun_direction: Vec3::new(-0.5, -1.0, -0.3).normalize(),
            sun_intensity: 10.0,
            sun_color: Vec3::new(1.0, 0.95, 0.9),
            ambient_intensity: 0.3,
            fog: FogSettings::default(),
            clouds: CloudSettings::default(),
        }
    }
}

impl Default for FogSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            density: 0.01,
            height_falloff: 1.0,
            color: Vec3::new(0.5, 0.6, 0.7),
            start_distance: 50.0,
            volumetric: false,
        }
    }
}

impl Default for CloudSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            coverage: 0.5,
            altitude: 2000.0,
            thickness: 500.0,
            wind_speed: 10.0,
        }
    }
}

impl Default for PostProcessSettings {
    fn default() -> Self {
        Self {
            exposure: 1.0,
            contrast: 1.0,
            saturation: 1.0,
            vignette: 0.0,
            bloom_intensity: 0.3,
            bloom_threshold: 1.5,
            chromatic_aberration: 0.0,
            film_grain: 0.0,
            dof_enabled: false,
            dof_focus_distance: 10.0,
            dof_aperture: 5.6,
            motion_blur: 0.0,
            tonemapping: Tonemapping::ACES,
            color_grading: ColorGrading::default(),
        }
    }
}

impl Default for ColorGrading {
    fn default() -> Self {
        Self {
            temperature: 0.0,
            tint: 0.0,
            shadows: Vec3::ONE * 0.5,
            midtones: Vec3::ONE,
            highlights: Vec3::ONE * 1.5,
            lut: None,
        }
    }
}

impl Default for MaterialData {
    fn default() -> Self {
        Self {
            name: "Default".into(),
            base_color: Vec4::new(0.8, 0.8, 0.8, 1.0),
            metallic: 0.0,
            roughness: 0.5,
            emission: Vec3::ZERO,
            emission_strength: 0.0,
            normal_strength: 1.0,
            ao_strength: 1.0,
            transmission: 0.0,
            ior: 1.5,
            subsurface: 0.0,
            subsurface_color: Vec3::ZERO,
        }
    }
}
