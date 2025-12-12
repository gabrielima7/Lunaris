//! Terrain Erosion System
//!
//! Hydraulic and thermal erosion simulation for realistic terrain sculpting.

use glam::Vec2;

// ==================== EROSION CONFIG ====================

/// Erosion configuration
#[derive(Debug, Clone)]
pub struct ErosionConfig {
    /// Number of erosion iterations
    pub iterations: u32,
    /// Random seed
    pub seed: u64,
    /// Erosion mode
    pub mode: ErosionMode,
    /// Hydraulic settings
    pub hydraulic: HydraulicErosionSettings,
    /// Thermal settings
    pub thermal: ThermalErosionSettings,
    /// Wind settings
    pub wind: WindErosionSettings,
}

impl Default for ErosionConfig {
    fn default() -> Self {
        Self {
            iterations: 50000,
            seed: 12345,
            mode: ErosionMode::Hydraulic,
            hydraulic: HydraulicErosionSettings::default(),
            thermal: ThermalErosionSettings::default(),
            wind: WindErosionSettings::default(),
        }
    }
}

/// Erosion mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErosionMode {
    /// Water-based erosion
    Hydraulic,
    /// Temperature-based erosion
    Thermal,
    /// Wind-based erosion
    Wind,
    /// Combined erosion
    Combined,
}

// ==================== HYDRAULIC EROSION ====================

/// Hydraulic erosion settings
#[derive(Debug, Clone)]
pub struct HydraulicErosionSettings {
    /// Erosion radius
    pub erosion_radius: i32,
    /// Inertia (0-1)
    pub inertia: f32,
    /// Sediment capacity factor
    pub sediment_capacity: f32,
    /// Minimum sediment capacity
    pub min_sediment_capacity: f32,
    /// Erosion speed
    pub erosion_speed: f32,
    /// Deposition speed
    pub deposition_speed: f32,
    /// Evaporation rate
    pub evaporation: f32,
    /// Gravity
    pub gravity: f32,
    /// Max droplet lifetime
    pub max_lifetime: u32,
    /// Initial water volume
    pub initial_water: f32,
    /// Initial droplet speed
    pub initial_speed: f32,
}

impl Default for HydraulicErosionSettings {
    fn default() -> Self {
        Self {
            erosion_radius: 3,
            inertia: 0.05,
            sediment_capacity: 4.0,
            min_sediment_capacity: 0.01,
            erosion_speed: 0.3,
            deposition_speed: 0.3,
            evaporation: 0.01,
            gravity: 4.0,
            max_lifetime: 30,
            initial_water: 1.0,
            initial_speed: 1.0,
        }
    }
}

/// Water droplet for hydraulic erosion
#[derive(Debug, Clone)]
struct WaterDroplet {
    position: Vec2,
    direction: Vec2,
    speed: f32,
    water: f32,
    sediment: f32,
}

impl WaterDroplet {
    fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            direction: Vec2::ZERO,
            speed: 1.0,
            water: 1.0,
            sediment: 0.0,
        }
    }
}

// ==================== THERMAL EROSION ====================

/// Thermal erosion settings
#[derive(Debug, Clone)]
pub struct ThermalErosionSettings {
    /// Talus angle (angle of repose) in radians
    pub talus_angle: f32,
    /// Amount of material to move per iteration
    pub erosion_rate: f32,
    /// Cell size
    pub cell_size: f32,
    /// Number of iterations per frame
    pub iterations_per_frame: u32,
}

impl Default for ThermalErosionSettings {
    fn default() -> Self {
        Self {
            talus_angle: 0.6, // ~35 degrees
            erosion_rate: 0.5,
            cell_size: 1.0,
            iterations_per_frame: 10,
        }
    }
}

// ==================== WIND EROSION ====================

/// Wind erosion settings
#[derive(Debug, Clone)]
pub struct WindErosionSettings {
    /// Wind direction
    pub direction: Vec2,
    /// Wind strength
    pub strength: f32,
    /// Suspension rate
    pub suspension: f32,
    /// Abrasion rate
    pub abrasion: f32,
    /// Deposition rate  
    pub deposition: f32,
    /// Particle size (affects transport)
    pub particle_size: f32,
}

impl Default for WindErosionSettings {
    fn default() -> Self {
        Self {
            direction: Vec2::new(1.0, 0.0),
            strength: 1.0,
            suspension: 0.1,
            abrasion: 0.05,
            deposition: 0.1,
            particle_size: 0.01,
        }
    }
}

// ==================== EROSION SIMULATOR ====================

/// Terrain erosion simulator
pub struct ErosionSimulator {
    /// Configuration
    pub config: ErosionConfig,
    /// Heightmap width
    width: usize,
    /// Heightmap height  
    height: usize,
    /// Erosion brush weights (precomputed)
    brush_weights: Vec<Vec<f32>>,
    /// Brush indices
    brush_indices: Vec<Vec<(i32, i32)>>,
    /// Progress (0-1)
    pub progress: f32,
    /// Current iteration
    current_iteration: u32,
    /// Random state
    rng_state: u64,
}

impl ErosionSimulator {
    /// Create new erosion simulator
    #[must_use]
    pub fn new(width: usize, height: usize, config: ErosionConfig) -> Self {
        let mut simulator = Self {
            width,
            height,
            config,
            brush_weights: Vec::new(),
            brush_indices: Vec::new(),
            progress: 0.0,
            current_iteration: 0,
            rng_state: 12345,
        };
        simulator.precompute_brush();
        simulator.rng_state = simulator.config.seed;
        simulator
    }

    fn precompute_brush(&mut self) {
        let radius = self.config.hydraulic.erosion_radius;
        
        self.brush_weights.clear();
        self.brush_indices.clear();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut weights = Vec::new();
                let mut indices = Vec::new();
                let mut weight_sum = 0.0;

                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        let dist_sq = (dx * dx + dy * dy) as f32;
                        let radius_sq = (radius * radius) as f32;

                        if dist_sq <= radius_sq {
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;

                            if nx >= 0 && nx < self.width as i32 && 
                               ny >= 0 && ny < self.height as i32 {
                                let weight = 1.0 - dist_sq.sqrt() / radius as f32;
                                weights.push(weight);
                                indices.push((dx, dy));
                                weight_sum += weight;
                            }
                        }
                    }
                }

                // Normalize weights
                for w in &mut weights {
                    *w /= weight_sum;
                }

                self.brush_weights.push(weights);
                self.brush_indices.push(indices);
            }
        }
    }

    fn random(&mut self) -> f32 {
        // Simple xorshift
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;
        (self.rng_state as f32) / (u64::MAX as f32)
    }

    /// Apply hydraulic erosion
    pub fn erode_hydraulic(&mut self, heightmap: &mut [f32], iterations: u32) {
        // Clone settings to avoid borrow conflicts
        let inertia = self.config.hydraulic.inertia;
        let initial_speed = self.config.hydraulic.initial_speed;
        let initial_water = self.config.hydraulic.initial_water;
        let max_lifetime = self.config.hydraulic.max_lifetime;
        let sediment_capacity = self.config.hydraulic.sediment_capacity;
        let min_sediment_capacity = self.config.hydraulic.min_sediment_capacity;
        let erosion_speed = self.config.hydraulic.erosion_speed;
        let deposition_speed = self.config.hydraulic.deposition_speed;
        let evaporation = self.config.hydraulic.evaporation;
        let gravity = self.config.hydraulic.gravity;
        let total_iterations = self.config.iterations;

        for _ in 0..iterations {
            // Random starting position
            let x = self.random() * (self.width - 1) as f32;
            let y = self.random() * (self.height - 1) as f32;

            let mut droplet = WaterDroplet::new(x, y);
            droplet.speed = initial_speed;
            droplet.water = initial_water;

            for _ in 0..max_lifetime {
                let xi = droplet.position.x as usize;
                let yi = droplet.position.y as usize;

                if xi >= self.width - 1 || yi >= self.height - 1 {
                    break;
                }

                // Get gradient
                let (gradient, height) = self.get_gradient_and_height(heightmap, droplet.position);

                // Update direction with inertia
                droplet.direction = droplet.direction * inertia 
                    - gradient * (1.0 - inertia);
                
                let len = droplet.direction.length();
                if len < 0.0001 {
                    // Random direction if stuck
                    let angle = self.random() * std::f32::consts::TAU;
                    droplet.direction = Vec2::new(angle.cos(), angle.sin());
                } else {
                    droplet.direction /= len;
                }

                // Move droplet
                let new_pos = droplet.position + droplet.direction;

                if new_pos.x < 0.0 || new_pos.x >= self.width as f32 - 1.0 ||
                   new_pos.y < 0.0 || new_pos.y >= self.height as f32 - 1.0 {
                    break;
                }

                // Height difference
                let new_height = self.sample_height(heightmap, new_pos);
                let delta_height = new_height - height;

                // Calculate sediment capacity
                let capacity = (-delta_height * droplet.speed * droplet.water * sediment_capacity)
                    .max(min_sediment_capacity);

                if droplet.sediment > capacity || delta_height > 0.0 {
                    // Deposit sediment
                    let deposit_amount = if delta_height > 0.0 {
                        droplet.sediment.min(delta_height)
                    } else {
                        (droplet.sediment - capacity) * deposition_speed
                    };

                    droplet.sediment -= deposit_amount;
                    self.deposit(heightmap, droplet.position, deposit_amount);
                } else {
                    // Erode
                    let erode_amount = ((capacity - droplet.sediment) * erosion_speed)
                        .min(-delta_height);

                    self.erode(heightmap, droplet.position, erode_amount);
                    droplet.sediment += erode_amount;
                }

                // Update speed and water
                droplet.speed = (droplet.speed * droplet.speed + delta_height.abs() * gravity)
                    .sqrt();
                droplet.water *= 1.0 - evaporation;

                droplet.position = new_pos;

                if droplet.water < 0.001 {
                    break;
                }
            }

            self.current_iteration += 1;
            self.progress = self.current_iteration as f32 / total_iterations as f32;
        }
    }

    fn get_gradient_and_height(&self, heightmap: &[f32], pos: Vec2) -> (Vec2, f32) {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let fx = pos.x.fract();
        let fy = pos.y.fract();

        let h00 = heightmap[y * self.width + x];
        let h10 = heightmap[y * self.width + x + 1];
        let h01 = heightmap[(y + 1) * self.width + x];
        let h11 = heightmap[(y + 1) * self.width + x + 1];

        let gx = (h10 - h00) * (1.0 - fy) + (h11 - h01) * fy;
        let gy = (h01 - h00) * (1.0 - fx) + (h11 - h10) * fx;

        let height = h00 * (1.0 - fx) * (1.0 - fy) 
            + h10 * fx * (1.0 - fy)
            + h01 * (1.0 - fx) * fy
            + h11 * fx * fy;

        (Vec2::new(gx, gy), height)
    }

    fn sample_height(&self, heightmap: &[f32], pos: Vec2) -> f32 {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let fx = pos.x.fract();
        let fy = pos.y.fract();

        let h00 = heightmap[y * self.width + x];
        let h10 = heightmap[y * self.width + x + 1];
        let h01 = heightmap[(y + 1) * self.width + x];
        let h11 = heightmap[(y + 1) * self.width + x + 1];

        h00 * (1.0 - fx) * (1.0 - fy) 
            + h10 * fx * (1.0 - fy)
            + h01 * (1.0 - fx) * fy
            + h11 * fx * fy
    }

    fn deposit(&self, heightmap: &mut [f32], pos: Vec2, amount: f32) {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let fx = pos.x.fract();
        let fy = pos.y.fract();

        heightmap[y * self.width + x] += amount * (1.0 - fx) * (1.0 - fy);
        heightmap[y * self.width + x + 1] += amount * fx * (1.0 - fy);
        heightmap[(y + 1) * self.width + x] += amount * (1.0 - fx) * fy;
        heightmap[(y + 1) * self.width + x + 1] += amount * fx * fy;
    }

    fn erode(&self, heightmap: &mut [f32], pos: Vec2, amount: f32) {
        let xi = pos.x as usize;
        let yi = pos.y as usize;
        let idx = yi * self.width + xi;

        if idx >= self.brush_weights.len() {
            return;
        }

        let weights = &self.brush_weights[idx];
        let indices = &self.brush_indices[idx];

        for (i, &(dx, dy)) in indices.iter().enumerate() {
            let nx = xi as i32 + dx;
            let ny = yi as i32 + dy;

            if nx >= 0 && nx < self.width as i32 && 
               ny >= 0 && ny < self.height as i32 {
                let nidx = ny as usize * self.width + nx as usize;
                heightmap[nidx] -= amount * weights[i];
            }
        }
    }

    /// Apply thermal erosion
    pub fn erode_thermal(&mut self, heightmap: &mut [f32], iterations: u32) {
        let settings = &self.config.thermal;
        let max_diff = settings.talus_angle * settings.cell_size;

        for _ in 0..iterations {
            let mut changes = vec![0.0f32; heightmap.len()];

            for y in 1..self.height - 1 {
                for x in 1..self.width - 1 {
                    let idx = y * self.width + x;
                    let h = heightmap[idx];

                    // Check 8 neighbors
                    let neighbors = [
                        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
                        (x - 1, y),                 (x + 1, y),
                        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
                    ];

                    let mut max_slope = 0.0f32;
                    let mut steepest = None;
                    let mut total_diff = 0.0f32;
                    let mut diff_neighbors = Vec::new();

                    for &(nx, ny) in &neighbors {
                        let nidx = ny * self.width + nx;
                        let nh = heightmap[nidx];
                        let diff = h - nh;

                        if diff > max_diff {
                            total_diff += diff - max_diff;
                            diff_neighbors.push((nidx, diff - max_diff));
                        }

                        if diff > max_slope {
                            max_slope = diff;
                            steepest = Some(nidx);
                        }
                    }

                    if total_diff > 0.0 {
                        let move_amount = total_diff * settings.erosion_rate * 0.5;
                        changes[idx] -= move_amount;

                        for (nidx, diff) in diff_neighbors {
                            changes[nidx] += move_amount * (diff / total_diff);
                        }
                    }
                }
            }

            // Apply changes
            for (i, change) in changes.iter().enumerate() {
                heightmap[i] += change;
            }

            self.current_iteration += 1;
            self.progress = self.current_iteration as f32 / self.config.iterations as f32;
        }
    }

    /// Apply wind erosion
    pub fn erode_wind(&mut self, heightmap: &mut [f32], iterations: u32) {
        let settings = &self.config.wind;
        let dir = settings.direction.normalize();

        for _ in 0..iterations {
            let mut suspension = vec![0.0f32; heightmap.len()];
            let mut deposition = vec![0.0f32; heightmap.len()];

            // Calculate suspension
            for y in 1..self.height - 1 {
                for x in 1..self.width - 1 {
                    let idx = y * self.width + x;
                    let h = heightmap[idx];

                    // Check windward direction
                    let wx = (x as f32 - dir.x) as usize;
                    let wy = (y as f32 - dir.y) as usize;

                    if wx < self.width && wy < self.height {
                        let widx = wy * self.width + wx;
                        let wh = heightmap[widx];

                        // Exposure to wind
                        let exposure = (h - wh).max(0.0) * settings.strength;
                        let suspend_amount = exposure * settings.suspension;

                        suspension[idx] = suspend_amount;
                    }
                }
            }

            // Transport and deposit
            for y in 1..self.height - 1 {
                for x in 1..self.width - 1 {
                    let idx = y * self.width + x;

                    // Get leeward position
                    let lx = ((x as f32 + dir.x * 2.0) as usize).min(self.width - 1);
                    let ly = ((y as f32 + dir.y * 2.0) as usize).min(self.height - 1);
                    let lidx = ly * self.width + lx;

                    // Deposit in lee
                    deposition[lidx] += suspension[idx] * settings.deposition;
                }
            }

            // Apply changes
            for i in 0..heightmap.len() {
                heightmap[i] += deposition[i] - suspension[i] * settings.deposition;
            }

            self.current_iteration += 1;
            self.progress = self.current_iteration as f32 / self.config.iterations as f32;
        }
    }

    /// Run erosion based on config mode
    pub fn run(&mut self, heightmap: &mut [f32]) {
        let iterations = self.config.iterations;

        match self.config.mode {
            ErosionMode::Hydraulic => {
                self.erode_hydraulic(heightmap, iterations);
            }
            ErosionMode::Thermal => {
                self.erode_thermal(heightmap, iterations);
            }
            ErosionMode::Wind => {
                self.erode_wind(heightmap, iterations);
            }
            ErosionMode::Combined => {
                self.erode_hydraulic(heightmap, iterations / 3);
                self.erode_thermal(heightmap, iterations / 3);
                self.erode_wind(heightmap, iterations / 3);
            }
        }
    }

    /// Run single step (for real-time preview)
    pub fn step(&mut self, heightmap: &mut [f32], iterations: u32) {
        match self.config.mode {
            ErosionMode::Hydraulic => self.erode_hydraulic(heightmap, iterations),
            ErosionMode::Thermal => self.erode_thermal(heightmap, iterations),
            ErosionMode::Wind => self.erode_wind(heightmap, iterations),
            ErosionMode::Combined => {
                self.erode_hydraulic(heightmap, iterations / 3);
                self.erode_thermal(heightmap, iterations / 3);
                self.erode_wind(heightmap, iterations / 3);
            }
        }
    }

    /// Reset progress
    pub fn reset(&mut self) {
        self.current_iteration = 0;
        self.progress = 0.0;
        self.rng_state = self.config.seed;
    }
}

/// Erosion preset
#[derive(Debug, Clone)]
pub struct ErosionPreset {
    /// Preset name
    pub name: String,
    /// Configuration
    pub config: ErosionConfig,
}

impl ErosionPreset {
    /// Mountain erosion (deep valleys, sharp ridges)
    pub fn mountain() -> Self {
        Self {
            name: "Mountain".to_string(),
            config: ErosionConfig {
                iterations: 100000,
                mode: ErosionMode::Combined,
                hydraulic: HydraulicErosionSettings {
                    erosion_speed: 0.5,
                    sediment_capacity: 6.0,
                    ..Default::default()
                },
                thermal: ThermalErosionSettings {
                    talus_angle: 0.7,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }

    /// Desert erosion (wind-swept dunes)
    pub fn desert() -> Self {
        Self {
            name: "Desert".to_string(),
            config: ErosionConfig {
                iterations: 50000,
                mode: ErosionMode::Wind,
                wind: WindErosionSettings {
                    strength: 2.0,
                    suspension: 0.2,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }

    /// Coastal erosion (smooth, weathered)
    pub fn coastal() -> Self {
        Self {
            name: "Coastal".to_string(),
            config: ErosionConfig {
                iterations: 75000,
                mode: ErosionMode::Hydraulic,
                hydraulic: HydraulicErosionSettings {
                    erosion_radius: 4,
                    erosion_speed: 0.4,
                    evaporation: 0.02,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }

    /// Volcanic erosion (lava flows)
    pub fn volcanic() -> Self {
        Self {
            name: "Volcanic".to_string(),
            config: ErosionConfig {
                iterations: 30000,
                mode: ErosionMode::Thermal,
                thermal: ThermalErosionSettings {
                    talus_angle: 0.4,
                    erosion_rate: 0.7,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
