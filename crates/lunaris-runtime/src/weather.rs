//! Weather System
//!
//! Dynamic weather, storms, and volumetric effects.

use glam::{Vec3, Vec2};

/// Weather system
pub struct WeatherSystem {
    pub current: WeatherState,
    pub target: WeatherState,
    pub transition_time: f32,
    pub transition_progress: f32,
    pub time_of_day: f32,
    pub presets: Vec<WeatherPreset>,
}

/// Weather state
#[derive(Clone)]
pub struct WeatherState {
    pub weather_type: WeatherType,
    pub intensity: f32,
    pub wind_direction: Vec2,
    pub wind_speed: f32,
    pub cloud_coverage: f32,
    pub fog_density: f32,
    pub precipitation: PrecipitationState,
    pub temperature: f32,
}

/// Weather type
#[derive(Clone, Copy, PartialEq)]
pub enum WeatherType { Clear, Cloudy, Overcast, Rain, HeavyRain, Thunderstorm, Snow, Blizzard, Fog, Sandstorm }

/// Precipitation
#[derive(Clone)]
pub struct PrecipitationState {
    pub enabled: bool,
    pub precip_type: PrecipitationType,
    pub density: f32,
    pub size: f32,
    pub splash: bool,
    pub accumulation: f32,
}

/// Precipitation type
#[derive(Clone, Copy)]
pub enum PrecipitationType { Rain, Snow, Hail, Sleet }

/// Weather preset
pub struct WeatherPreset {
    pub name: String,
    pub state: WeatherState,
}

impl Default for WeatherState {
    fn default() -> Self {
        Self {
            weather_type: WeatherType::Clear, intensity: 0.0, wind_direction: Vec2::X, wind_speed: 5.0,
            cloud_coverage: 0.3, fog_density: 0.0, precipitation: PrecipitationState::none(), temperature: 20.0,
        }
    }
}

impl PrecipitationState {
    pub fn none() -> Self { Self { enabled: false, precip_type: PrecipitationType::Rain, density: 0.0, size: 1.0, splash: false, accumulation: 0.0 } }
    pub fn rain(intensity: f32) -> Self { Self { enabled: true, precip_type: PrecipitationType::Rain, density: intensity, size: 1.0, splash: true, accumulation: 0.0 } }
    pub fn snow(intensity: f32) -> Self { Self { enabled: true, precip_type: PrecipitationType::Snow, density: intensity, size: 1.5, splash: false, accumulation: intensity * 0.1 } }
}

impl WeatherSystem {
    pub fn new() -> Self {
        Self {
            current: WeatherState::default(),
            target: WeatherState::default(),
            transition_time: 60.0,
            transition_progress: 1.0,
            time_of_day: 12.0,
            presets: Self::default_presets(),
        }
    }

    fn default_presets() -> Vec<WeatherPreset> {
        vec![
            WeatherPreset { name: "Clear".into(), state: WeatherState { weather_type: WeatherType::Clear, cloud_coverage: 0.1, ..Default::default() } },
            WeatherPreset { name: "Cloudy".into(), state: WeatherState { weather_type: WeatherType::Cloudy, cloud_coverage: 0.6, ..Default::default() } },
            WeatherPreset { name: "Rain".into(), state: WeatherState { weather_type: WeatherType::Rain, intensity: 0.5, cloud_coverage: 0.8, precipitation: PrecipitationState::rain(0.5), ..Default::default() } },
            WeatherPreset { name: "Storm".into(), state: WeatherState { weather_type: WeatherType::Thunderstorm, intensity: 1.0, cloud_coverage: 1.0, wind_speed: 20.0, precipitation: PrecipitationState::rain(1.0), ..Default::default() } },
            WeatherPreset { name: "Snow".into(), state: WeatherState { weather_type: WeatherType::Snow, intensity: 0.5, cloud_coverage: 0.9, temperature: -5.0, precipitation: PrecipitationState::snow(0.5), ..Default::default() } },
            WeatherPreset { name: "Fog".into(), state: WeatherState { weather_type: WeatherType::Fog, fog_density: 0.1, cloud_coverage: 0.5, ..Default::default() } },
        ]
    }

    pub fn set_weather(&mut self, weather_type: WeatherType, transition_time: f32) {
        if let Some(preset) = self.presets.iter().find(|p| p.state.weather_type == weather_type) {
            self.target = preset.state.clone();
            self.transition_time = transition_time;
            self.transition_progress = 0.0;
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time_of_day = (self.time_of_day + dt / 60.0) % 24.0;
        
        if self.transition_progress < 1.0 {
            self.transition_progress = (self.transition_progress + dt / self.transition_time).min(1.0);
            let t = self.transition_progress;
            self.current.intensity = lerp(self.current.intensity, self.target.intensity, t);
            self.current.cloud_coverage = lerp(self.current.cloud_coverage, self.target.cloud_coverage, t);
            self.current.fog_density = lerp(self.current.fog_density, self.target.fog_density, t);
            self.current.wind_speed = lerp(self.current.wind_speed, self.target.wind_speed, t);
            self.current.precipitation.density = lerp(self.current.precipitation.density, self.target.precipitation.density, t);
            if t >= 1.0 { self.current = self.target.clone(); }
        }
    }

    pub fn get_sun_direction(&self) -> Vec3 {
        let angle = (self.time_of_day - 6.0) / 12.0 * std::f32::consts::PI;
        Vec3::new(angle.cos(), angle.sin().abs().max(0.1), 0.3).normalize()
    }

    pub fn is_night(&self) -> bool { self.time_of_day < 6.0 || self.time_of_day > 20.0 }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 { a + (b - a) * t }

/// Lightning system
pub struct LightningSystem {
    pub bolts: Vec<LightningBolt>,
    pub flash_intensity: f32,
    pub next_bolt_time: f32,
}

/// Lightning bolt
pub struct LightningBolt {
    pub start: Vec3,
    pub end: Vec3,
    pub branches: Vec<(Vec3, Vec3)>,
    pub lifetime: f32,
    pub age: f32,
}

impl LightningSystem {
    pub fn new() -> Self { Self { bolts: Vec::new(), flash_intensity: 0.0, next_bolt_time: 5.0 } }

    pub fn update(&mut self, dt: f32, storm_intensity: f32) {
        self.flash_intensity = (self.flash_intensity - dt * 5.0).max(0.0);
        self.bolts.retain_mut(|b| { b.age += dt; b.age < b.lifetime });
        
        self.next_bolt_time -= dt * storm_intensity;
        if self.next_bolt_time <= 0.0 {
            self.spawn_bolt(Vec3::new(rand() * 200.0 - 100.0, 100.0, rand() * 200.0 - 100.0));
            self.next_bolt_time = 2.0 + rand() * 8.0;
        }
    }

    pub fn spawn_bolt(&mut self, position: Vec3) {
        self.bolts.push(LightningBolt {
            start: position, end: Vec3::new(position.x + (rand() - 0.5) * 20.0, 0.0, position.z + (rand() - 0.5) * 20.0),
            branches: Vec::new(), lifetime: 0.3, age: 0.0,
        });
        self.flash_intensity = 1.0;
    }
}

fn rand() -> f32 { 0.5 } // Placeholder
