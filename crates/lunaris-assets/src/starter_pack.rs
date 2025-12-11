//! Starter Asset Pack
//!
//! Free assets included with Lunaris Engine for quick prototyping.

use std::collections::HashMap;

// ==================== MATERIALS ====================

/// PBR Material preset
#[derive(Debug, Clone)]
pub struct MaterialPreset {
    pub name: String,
    pub albedo: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub normal_strength: f32,
    pub emission: [f32; 3],
    pub emission_strength: f32,
}

/// Standard material library
pub fn standard_materials() -> HashMap<String, MaterialPreset> {
    let mut materials = HashMap::new();

    // Metals
    materials.insert("gold".to_string(), MaterialPreset {
        name: "Gold".to_string(),
        albedo: [1.0, 0.766, 0.336, 1.0],
        metallic: 1.0,
        roughness: 0.3,
        normal_strength: 1.0,
        emission: [0.0, 0.0, 0.0],
        emission_strength: 0.0,
    });

    materials.insert("silver".to_string(), MaterialPreset {
        name: "Silver".to_string(),
        albedo: [0.972, 0.960, 0.915, 1.0],
        metallic: 1.0,
        roughness: 0.2,
        normal_strength: 1.0,
        emission: [0.0, 0.0, 0.0],
        emission_strength: 0.0,
    });

    materials.insert("copper".to_string(), MaterialPreset {
        name: "Copper".to_string(),
        albedo: [0.955, 0.637, 0.538, 1.0],
        metallic: 1.0,
        roughness: 0.4,
        normal_strength: 1.0,
        emission: [0.0, 0.0, 0.0],
        emission_strength: 0.0,
    });

    materials.insert("iron".to_string(), MaterialPreset {
        name: "Iron".to_string(),
        albedo: [0.56, 0.57, 0.58, 1.0],
        metallic: 1.0,
        roughness: 0.5,
        normal_strength: 1.0,
        emission: [0.0, 0.0, 0.0],
        emission_strength: 0.0,
    });

    // Non-metals
    materials.insert("wood".to_string(), MaterialPreset {
        name: "Wood".to_string(),
        albedo: [0.55, 0.35, 0.2, 1.0],
        metallic: 0.0,
        roughness: 0.7,
        normal_strength: 1.0,
        emission: [0.0, 0.0, 0.0],
        emission_strength: 0.0,
    });

    materials.insert("brick".to_string(), MaterialPreset {
        name: "Brick".to_string(),
        albedo: [0.65, 0.35, 0.28, 1.0],
        metallic: 0.0,
        roughness: 0.85,
        normal_strength: 1.5,
        emission: [0.0, 0.0, 0.0],
        emission_strength: 0.0,
    });

    materials.insert("concrete".to_string(), MaterialPreset {
        name: "Concrete".to_string(),
        albedo: [0.6, 0.6, 0.6, 1.0],
        metallic: 0.0,
        roughness: 0.9,
        normal_strength: 0.5,
        emission: [0.0, 0.0, 0.0],
        emission_strength: 0.0,
    });

    materials.insert("marble".to_string(), MaterialPreset {
        name: "Marble".to_string(),
        albedo: [0.95, 0.93, 0.9, 1.0],
        metallic: 0.0,
        roughness: 0.2,
        normal_strength: 0.3,
        emission: [0.0, 0.0, 0.0],
        emission_strength: 0.0,
    });

    materials.insert("grass".to_string(), MaterialPreset {
        name: "Grass".to_string(),
        albedo: [0.3, 0.5, 0.2, 1.0],
        metallic: 0.0,
        roughness: 0.8,
        normal_strength: 0.7,
        emission: [0.0, 0.0, 0.0],
        emission_strength: 0.0,
    });

    materials.insert("sand".to_string(), MaterialPreset {
        name: "Sand".to_string(),
        albedo: [0.85, 0.75, 0.55, 1.0],
        metallic: 0.0,
        roughness: 0.9,
        normal_strength: 0.3,
        emission: [0.0, 0.0, 0.0],
        emission_strength: 0.0,
    });

    // Special
    materials.insert("glass".to_string(), MaterialPreset {
        name: "Glass".to_string(),
        albedo: [0.9, 0.9, 0.95, 0.2],
        metallic: 0.0,
        roughness: 0.05,
        normal_strength: 0.1,
        emission: [0.0, 0.0, 0.0],
        emission_strength: 0.0,
    });

    materials.insert("water".to_string(), MaterialPreset {
        name: "Water".to_string(),
        albedo: [0.1, 0.3, 0.5, 0.6],
        metallic: 0.0,
        roughness: 0.05,
        normal_strength: 0.5,
        emission: [0.0, 0.0, 0.0],
        emission_strength: 0.0,
    });

    materials.insert("lava".to_string(), MaterialPreset {
        name: "Lava".to_string(),
        albedo: [0.1, 0.05, 0.02, 1.0],
        metallic: 0.0,
        roughness: 0.3,
        normal_strength: 2.0,
        emission: [1.0, 0.3, 0.05],
        emission_strength: 5.0,
    });

    materials.insert("neon_blue".to_string(), MaterialPreset {
        name: "Neon Blue".to_string(),
        albedo: [0.0, 0.0, 0.1, 1.0],
        metallic: 0.0,
        roughness: 0.1,
        normal_strength: 0.0,
        emission: [0.1, 0.5, 1.0],
        emission_strength: 10.0,
    });

    materials.insert("neon_pink".to_string(), MaterialPreset {
        name: "Neon Pink".to_string(),
        albedo: [0.1, 0.0, 0.05, 1.0],
        metallic: 0.0,
        roughness: 0.1,
        normal_strength: 0.0,
        emission: [1.0, 0.1, 0.5],
        emission_strength: 10.0,
    });

    materials
}

// ==================== PRIMITIVE MESHES ====================

/// Primitive mesh type
#[derive(Debug, Clone, Copy)]
pub enum PrimitiveMesh {
    Cube,
    Sphere,
    Cylinder,
    Cone,
    Torus,
    Plane,
    Capsule,
    Pyramid,
    Wedge,
    Prism,
}

/// Mesh data
#[derive(Debug, Clone)]
pub struct MeshData {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

/// Generate primitive mesh
pub fn generate_primitive(primitive: PrimitiveMesh, subdivisions: u32) -> MeshData {
    match primitive {
        PrimitiveMesh::Cube => generate_cube(),
        PrimitiveMesh::Sphere => generate_sphere(subdivisions),
        PrimitiveMesh::Cylinder => generate_cylinder(subdivisions),
        PrimitiveMesh::Cone => generate_cone(subdivisions),
        PrimitiveMesh::Plane => generate_plane(subdivisions),
        _ => generate_cube(), // Placeholder
    }
}

fn generate_cube() -> MeshData {
    let vertices = vec![
        // Front
        [-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5],
        // Back
        [0.5, -0.5, -0.5], [-0.5, -0.5, -0.5], [-0.5, 0.5, -0.5], [0.5, 0.5, -0.5],
        // Top
        [-0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5],
        // Bottom
        [-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, -0.5, 0.5], [-0.5, -0.5, 0.5],
        // Right
        [0.5, -0.5, 0.5], [0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [0.5, 0.5, 0.5],
        // Left
        [-0.5, -0.5, -0.5], [-0.5, -0.5, 0.5], [-0.5, 0.5, 0.5], [-0.5, 0.5, -0.5],
    ];

    let normals = vec![
        [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0],
        [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0],
        [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0],
        [0.0, -1.0, 0.0], [0.0, -1.0, 0.0], [0.0, -1.0, 0.0], [0.0, -1.0, 0.0],
        [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0],
    ];

    let uvs = vec![
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0],
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0],
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0],
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0],
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0],
        [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0],
    ];

    let indices = vec![
        0, 1, 2, 0, 2, 3,       // Front
        4, 5, 6, 4, 6, 7,       // Back
        8, 9, 10, 8, 10, 11,    // Top
        12, 13, 14, 12, 14, 15, // Bottom
        16, 17, 18, 16, 18, 19, // Right
        20, 21, 22, 20, 22, 23, // Left
    ];

    MeshData { vertices, normals, uvs, indices }
}

fn generate_sphere(subdivisions: u32) -> MeshData {
    let segments = subdivisions.max(8);
    let rings = segments / 2;
    
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    for ring in 0..=rings {
        let v = ring as f32 / rings as f32;
        let phi = v * std::f32::consts::PI;

        for segment in 0..=segments {
            let u = segment as f32 / segments as f32;
            let theta = u * std::f32::consts::TAU;

            let x = phi.sin() * theta.cos();
            let y = phi.cos();
            let z = phi.sin() * theta.sin();

            vertices.push([x * 0.5, y * 0.5, z * 0.5]);
            normals.push([x, y, z]);
            uvs.push([u, v]);
        }
    }

    for ring in 0..rings {
        for segment in 0..segments {
            let a = ring * (segments + 1) + segment;
            let b = a + segments + 1;

            indices.push(a);
            indices.push(b);
            indices.push(a + 1);

            indices.push(b);
            indices.push(b + 1);
            indices.push(a + 1);
        }
    }

    MeshData { vertices, normals, uvs, indices }
}

fn generate_cylinder(subdivisions: u32) -> MeshData {
    let segments = subdivisions.max(8);
    
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    // Side vertices
    for i in 0..=segments {
        let u = i as f32 / segments as f32;
        let theta = u * std::f32::consts::TAU;
        let x = theta.cos() * 0.5;
        let z = theta.sin() * 0.5;

        // Bottom
        vertices.push([x, -0.5, z]);
        normals.push([theta.cos(), 0.0, theta.sin()]);
        uvs.push([u, 0.0]);

        // Top
        vertices.push([x, 0.5, z]);
        normals.push([theta.cos(), 0.0, theta.sin()]);
        uvs.push([u, 1.0]);
    }

    // Side indices
    for i in 0..segments {
        let a = i * 2;
        indices.push(a);
        indices.push(a + 1);
        indices.push(a + 2);
        indices.push(a + 1);
        indices.push(a + 3);
        indices.push(a + 2);
    }

    MeshData { vertices, normals, uvs, indices }
}

fn generate_cone(subdivisions: u32) -> MeshData {
    let segments = subdivisions.max(8);
    
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    // Tip
    vertices.push([0.0, 0.5, 0.0]);
    normals.push([0.0, 1.0, 0.0]);
    uvs.push([0.5, 0.0]);

    // Base vertices
    for i in 0..=segments {
        let u = i as f32 / segments as f32;
        let theta = u * std::f32::consts::TAU;
        let x = theta.cos() * 0.5;
        let z = theta.sin() * 0.5;

        vertices.push([x, -0.5, z]);
        normals.push([theta.cos(), 0.5, theta.sin()].map(|v| v / 1.118)); // Normalize
        uvs.push([u, 1.0]);
    }

    // Side indices
    for i in 0..segments {
        indices.push(0);
        indices.push(i + 1);
        indices.push(i + 2);
    }

    MeshData { vertices, normals, uvs, indices }
}

fn generate_plane(subdivisions: u32) -> MeshData {
    let segments = subdivisions.max(1);
    
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    for y in 0..=segments {
        for x in 0..=segments {
            let u = x as f32 / segments as f32;
            let v = y as f32 / segments as f32;

            vertices.push([u - 0.5, 0.0, v - 0.5]);
            normals.push([0.0, 1.0, 0.0]);
            uvs.push([u, v]);
        }
    }

    for y in 0..segments {
        for x in 0..segments {
            let a = y * (segments + 1) + x;
            let b = a + segments + 1;

            indices.push(a);
            indices.push(b);
            indices.push(a + 1);
            indices.push(b);
            indices.push(b + 1);
            indices.push(a + 1);
        }
    }

    MeshData { vertices, normals, uvs, indices }
}

// ==================== PARTICLE EFFECTS ====================

/// Particle effect preset
#[derive(Debug, Clone)]
pub struct ParticleEffect {
    pub name: String,
    pub max_particles: u32,
    pub spawn_rate: f32,
    pub lifetime: (f32, f32),
    pub start_size: (f32, f32),
    pub end_size: (f32, f32),
    pub start_color: [f32; 4],
    pub end_color: [f32; 4],
    pub velocity: ([f32; 3], [f32; 3]),
    pub gravity: f32,
    pub blend_mode: BlendMode,
}

/// Blend mode
#[derive(Debug, Clone, Copy)]
pub enum BlendMode {
    Alpha,
    Additive,
    Multiply,
}

/// Standard particle effects
pub fn standard_particles() -> HashMap<String, ParticleEffect> {
    let mut effects = HashMap::new();

    effects.insert("fire".to_string(), ParticleEffect {
        name: "Fire".to_string(),
        max_particles: 200,
        spawn_rate: 50.0,
        lifetime: (0.5, 1.5),
        start_size: (0.1, 0.2),
        end_size: (0.0, 0.05),
        start_color: [1.0, 0.8, 0.2, 1.0],
        end_color: [1.0, 0.2, 0.0, 0.0],
        velocity: ([0.0, 1.0, 0.0], [0.3, 2.0, 0.3]),
        gravity: -0.5,
        blend_mode: BlendMode::Additive,
    });

    effects.insert("smoke".to_string(), ParticleEffect {
        name: "Smoke".to_string(),
        max_particles: 100,
        spawn_rate: 20.0,
        lifetime: (2.0, 4.0),
        start_size: (0.1, 0.2),
        end_size: (0.5, 1.0),
        start_color: [0.3, 0.3, 0.3, 0.8],
        end_color: [0.1, 0.1, 0.1, 0.0],
        velocity: ([0.0, 0.5, 0.0], [0.2, 1.0, 0.2]),
        gravity: -0.1,
        blend_mode: BlendMode::Alpha,
    });

    effects.insert("sparks".to_string(), ParticleEffect {
        name: "Sparks".to_string(),
        max_particles: 50,
        spawn_rate: 30.0,
        lifetime: (0.2, 0.5),
        start_size: (0.02, 0.05),
        end_size: (0.0, 0.01),
        start_color: [1.0, 0.9, 0.5, 1.0],
        end_color: [1.0, 0.5, 0.0, 0.0],
        velocity: ([-1.0, 0.0, -1.0], [1.0, 2.0, 1.0]),
        gravity: 5.0,
        blend_mode: BlendMode::Additive,
    });

    effects.insert("magic".to_string(), ParticleEffect {
        name: "Magic".to_string(),
        max_particles: 150,
        spawn_rate: 40.0,
        lifetime: (0.5, 1.0),
        start_size: (0.05, 0.1),
        end_size: (0.0, 0.02),
        start_color: [0.3, 0.5, 1.0, 1.0],
        end_color: [0.8, 0.2, 1.0, 0.0],
        velocity: ([-0.5, -0.5, -0.5], [0.5, 0.5, 0.5]),
        gravity: 0.0,
        blend_mode: BlendMode::Additive,
    });

    effects.insert("rain".to_string(), ParticleEffect {
        name: "Rain".to_string(),
        max_particles: 1000,
        spawn_rate: 500.0,
        lifetime: (0.5, 1.0),
        start_size: (0.01, 0.02),
        end_size: (0.01, 0.02),
        start_color: [0.7, 0.8, 0.9, 0.6],
        end_color: [0.7, 0.8, 0.9, 0.3],
        velocity: ([0.0, -10.0, 0.0], [0.5, -15.0, 0.5]),
        gravity: 0.0,
        blend_mode: BlendMode::Alpha,
    });

    effects.insert("snow".to_string(), ParticleEffect {
        name: "Snow".to_string(),
        max_particles: 500,
        spawn_rate: 100.0,
        lifetime: (3.0, 6.0),
        start_size: (0.02, 0.05),
        end_size: (0.02, 0.05),
        start_color: [1.0, 1.0, 1.0, 0.9],
        end_color: [1.0, 1.0, 1.0, 0.0],
        velocity: ([-0.5, -1.0, -0.5], [0.5, -2.0, 0.5]),
        gravity: 0.0,
        blend_mode: BlendMode::Alpha,
    });

    effects
}

// ==================== AUDIO PRESETS ====================

/// Sound effect category
#[derive(Debug, Clone)]
pub struct SoundCategory {
    pub name: String,
    pub sounds: Vec<SoundPreset>,
}

/// Sound preset
#[derive(Debug, Clone)]
pub struct SoundPreset {
    pub name: String,
    pub description: String,
    pub pitch_variation: f32,
    pub volume: f32,
    pub loop_: bool,
}

/// Standard sound categories
pub fn standard_sounds() -> Vec<SoundCategory> {
    vec![
        SoundCategory {
            name: "UI".to_string(),
            sounds: vec![
                SoundPreset { name: "click".to_string(), description: "Button click".to_string(), pitch_variation: 0.1, volume: 0.5, loop_: false },
                SoundPreset { name: "hover".to_string(), description: "Hover over element".to_string(), pitch_variation: 0.05, volume: 0.3, loop_: false },
                SoundPreset { name: "error".to_string(), description: "Error notification".to_string(), pitch_variation: 0.0, volume: 0.6, loop_: false },
                SoundPreset { name: "success".to_string(), description: "Success notification".to_string(), pitch_variation: 0.0, volume: 0.5, loop_: false },
            ],
        },
        SoundCategory {
            name: "Combat".to_string(),
            sounds: vec![
                SoundPreset { name: "sword_hit".to_string(), description: "Sword impact".to_string(), pitch_variation: 0.15, volume: 0.8, loop_: false },
                SoundPreset { name: "arrow_fire".to_string(), description: "Arrow release".to_string(), pitch_variation: 0.1, volume: 0.6, loop_: false },
                SoundPreset { name: "magic_cast".to_string(), description: "Spell cast".to_string(), pitch_variation: 0.2, volume: 0.7, loop_: false },
                SoundPreset { name: "explosion".to_string(), description: "Explosion".to_string(), pitch_variation: 0.1, volume: 1.0, loop_: false },
            ],
        },
        SoundCategory {
            name: "Environment".to_string(),
            sounds: vec![
                SoundPreset { name: "wind".to_string(), description: "Wind ambience".to_string(), pitch_variation: 0.05, volume: 0.4, loop_: true },
                SoundPreset { name: "rain".to_string(), description: "Rain ambience".to_string(), pitch_variation: 0.0, volume: 0.5, loop_: true },
                SoundPreset { name: "fire_loop".to_string(), description: "Fire crackling".to_string(), pitch_variation: 0.1, volume: 0.6, loop_: true },
                SoundPreset { name: "water_flow".to_string(), description: "Water stream".to_string(), pitch_variation: 0.05, volume: 0.5, loop_: true },
            ],
        },
        SoundCategory {
            name: "Characters".to_string(),
            sounds: vec![
                SoundPreset { name: "footstep_grass".to_string(), description: "Footstep on grass".to_string(), pitch_variation: 0.2, volume: 0.4, loop_: false },
                SoundPreset { name: "footstep_stone".to_string(), description: "Footstep on stone".to_string(), pitch_variation: 0.15, volume: 0.5, loop_: false },
                SoundPreset { name: "jump".to_string(), description: "Jump sound".to_string(), pitch_variation: 0.1, volume: 0.5, loop_: false },
                SoundPreset { name: "land".to_string(), description: "Landing sound".to_string(), pitch_variation: 0.15, volume: 0.6, loop_: false },
            ],
        },
    ]
}
