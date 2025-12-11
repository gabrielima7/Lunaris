//! FFT Ocean Simulation
//!
//! Tessendorf waves, foam, and Phillips spectrum.

use glam::{Vec2, Vec3, Vec4};
use std::f32::consts::PI;

/// FFT Ocean
pub struct FFTOcean {
    pub settings: OceanSettings,
    pub spectrum: OceanSpectrum,
    pub displacement_map: Vec<Vec3>,
    pub normal_map: Vec<Vec3>,
    pub foam_map: Vec<f32>,
    pub time: f32,
}

/// Ocean settings
pub struct OceanSettings {
    pub resolution: u32,
    pub size: f32,
    pub wind_speed: f32,
    pub wind_direction: Vec2,
    pub amplitude: f32,
    pub choppiness: f32,
    pub foam_threshold: f32,
    pub foam_decay: f32,
}

impl Default for OceanSettings {
    fn default() -> Self {
        Self {
            resolution: 256, size: 1000.0, wind_speed: 30.0, wind_direction: Vec2::new(1.0, 0.0).normalize(),
            amplitude: 1.0, choppiness: 1.5, foam_threshold: 0.5, foam_decay: 0.95,
        }
    }
}

/// Ocean spectrum
pub struct OceanSpectrum {
    pub spectrum_type: SpectrumType,
    pub h0: Vec<Complex>,
    pub frequencies: Vec<f32>,
}

/// Spectrum type
pub enum SpectrumType { Phillips, Jonswap, TMA }

/// Complex number
#[derive(Clone, Copy)]
pub struct Complex { pub re: f32, pub im: f32 }

impl Complex {
    pub fn new(re: f32, im: f32) -> Self { Self { re, im } }
    pub fn zero() -> Self { Self { re: 0.0, im: 0.0 } }
    pub fn exp(self) -> Self { let e = self.re.exp(); Self { re: e * self.im.cos(), im: e * self.im.sin() } }
    pub fn mul(self, other: Self) -> Self { Self { re: self.re * other.re - self.im * other.im, im: self.re * other.im + self.im * other.re } }
    pub fn conj(self) -> Self { Self { re: self.re, im: -self.im } }
    pub fn add(self, other: Self) -> Self { Self { re: self.re + other.re, im: self.im + other.im } }
    pub fn scale(self, s: f32) -> Self { Self { re: self.re * s, im: self.im * s } }
}

impl FFTOcean {
    pub fn new(settings: OceanSettings) -> Self {
        let n = settings.resolution as usize;
        let len = n * n;
        
        let mut ocean = Self {
            settings,
            spectrum: OceanSpectrum { spectrum_type: SpectrumType::Phillips, h0: vec![Complex::zero(); len], frequencies: vec![0.0; len] },
            displacement_map: vec![Vec3::ZERO; len],
            normal_map: vec![Vec3::Y; len],
            foam_map: vec![0.0; len],
            time: 0.0,
        };
        ocean.generate_spectrum();
        ocean
    }

    fn generate_spectrum(&mut self) {
        let n = self.settings.resolution as i32;
        let l = self.settings.size;
        
        for j in 0..n {
            for i in 0..n {
                let k = Vec2::new(
                    2.0 * PI * (i - n / 2) as f32 / l,
                    2.0 * PI * (j - n / 2) as f32 / l,
                );
                
                let phillips = self.phillips_spectrum(k);
                let idx = (j * n + i) as usize;
                
                // h0(k) = 1/sqrt(2) * (xi_r + i * xi_i) * sqrt(P(k))
                let xi_r = gaussian_random();
                let xi_i = gaussian_random();
                self.spectrum.h0[idx] = Complex::new(xi_r, xi_i).scale(phillips.sqrt() / 2.0f32.sqrt());
                self.spectrum.frequencies[idx] = self.dispersion(k.length());
            }
        }
    }

    fn phillips_spectrum(&self, k: Vec2) -> f32 {
        let k_len = k.length();
        if k_len < 0.0001 { return 0.0; }
        
        let g = 9.81;
        let l = self.settings.wind_speed * self.settings.wind_speed / g;
        let k_hat = k / k_len;
        let w_hat = self.settings.wind_direction;
        
        let k_dot_w = k_hat.dot(w_hat);
        let damping = 0.001;
        let l2 = l * l;
        let k2 = k_len * k_len;
        
        self.settings.amplitude * (-1.0 / (k2 * l2)).exp() / (k2 * k2) * k_dot_w.powi(2) * (-k2 * damping * damping).exp()
    }

    fn dispersion(&self, k: f32) -> f32 {
        let g = 9.81;
        (g * k).sqrt()
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
        let n = self.settings.resolution as i32;
        
        // Update h(k,t) = h0(k) * exp(i * w(k) * t) + h0*(-k) * exp(-i * w(k) * t)
        let mut h_tilde = vec![Complex::zero(); (n * n) as usize];
        let mut h_tilde_dx = vec![Complex::zero(); (n * n) as usize];
        let mut h_tilde_dz = vec![Complex::zero(); (n * n) as usize];
        
        for j in 0..n {
            for i in 0..n {
                let idx = (j * n + i) as usize;
                let idx_conj = ((n - 1 - j) * n + (n - 1 - i)) as usize;
                
                let w = self.spectrum.frequencies[idx];
                let phase = Complex::new(0.0, w * self.time).exp();
                let phase_conj = Complex::new(0.0, -w * self.time).exp();
                
                let h0 = self.spectrum.h0[idx];
                let h0_conj = self.spectrum.h0[idx_conj].conj();
                
                h_tilde[idx] = h0.mul(phase).add(h0_conj.mul(phase_conj));
                
                let k = Vec2::new(
                    2.0 * PI * (i - n / 2) as f32 / self.settings.size,
                    2.0 * PI * (j - n / 2) as f32 / self.settings.size,
                );
                let k_len = k.length().max(0.0001);
                
                h_tilde_dx[idx] = Complex::new(0.0, -k.x / k_len).mul(h_tilde[idx]);
                h_tilde_dz[idx] = Complex::new(0.0, -k.y / k_len).mul(h_tilde[idx]);
            }
        }
        
        // IFFT and update maps
        let heights = self.ifft_2d(&h_tilde);
        let dx = self.ifft_2d(&h_tilde_dx);
        let dz = self.ifft_2d(&h_tilde_dz);
        
        for j in 0..n {
            for i in 0..n {
                let idx = (j * n + i) as usize;
                let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
                
                self.displacement_map[idx] = Vec3::new(
                    dx[idx].re * self.settings.choppiness * sign,
                    heights[idx].re * sign,
                    dz[idx].re * self.settings.choppiness * sign,
                );
                
                // Calculate normal
                let eps = self.settings.size / n as f32;
                let dx_h = (self.displacement_map[(idx + 1) % (n * n) as usize].y - self.displacement_map[idx.saturating_sub(1)].y) / (2.0 * eps);
                let dz_h = (self.displacement_map[(idx + n as usize) % (n * n) as usize].y - self.displacement_map[idx.saturating_sub(n as usize)].y) / (2.0 * eps);
                self.normal_map[idx] = Vec3::new(-dx_h, 1.0, -dz_h).normalize();
                
                // Foam (based on jacobian)
                let jacobian = (1.0 + dx[idx].re.abs()) * (1.0 + dz[idx].re.abs()) - 1.0;
                if jacobian < self.settings.foam_threshold {
                    self.foam_map[idx] = (self.foam_map[idx] + 0.1).min(1.0);
                } else {
                    self.foam_map[idx] *= self.settings.foam_decay;
                }
            }
        }
    }

    fn ifft_2d(&self, data: &[Complex]) -> Vec<Complex> {
        // Simplified - would use proper FFT library
        data.to_vec()
    }

    pub fn get_height(&self, x: f32, z: f32) -> f32 {
        let n = self.settings.resolution as f32;
        let s = self.settings.size;
        let u = (x / s * n) as usize % self.settings.resolution as usize;
        let v = (z / s * n) as usize % self.settings.resolution as usize;
        self.displacement_map[v * self.settings.resolution as usize + u].y
    }
}

fn gaussian_random() -> f32 {
    let u1 = rand();
    let u2 = rand();
    (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos()
}

fn rand() -> f32 { 0.5 }
