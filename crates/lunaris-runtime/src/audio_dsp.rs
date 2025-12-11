//! Audio DSP Effects
//!
//! Distortion, chorus, flanger, compressor, vocoder, pitch shift.

use std::collections::VecDeque;

/// DSP processor chain
pub struct DSPChain {
    pub effects: Vec<DSPEffect>,
    pub sample_rate: f32,
    pub enabled: bool,
}

/// DSP effect
pub enum DSPEffect {
    Distortion(Distortion),
    Chorus(Chorus),
    Flanger(Flanger),
    Phaser(Phaser),
    Delay(Delay),
    Reverb(ConvolutionReverb),
    Compressor(Compressor),
    Limiter(Limiter),
    EQ(ParametricEQ),
    PitchShift(PitchShifter),
    Vocoder(Vocoder),
    Filter(BiquadFilter),
}

/// Distortion
pub struct Distortion {
    pub drive: f32,
    pub mix: f32,
    pub mode: DistortionMode,
    pub tone: f32,
}

/// Distortion mode
pub enum DistortionMode { SoftClip, HardClip, Tube, Fuzz, BitCrush(u32) }

impl Distortion {
    pub fn process(&self, input: f32) -> f32 {
        let driven = input * self.drive;
        let distorted = match self.mode {
            DistortionMode::SoftClip => (driven * 1.5).tanh(),
            DistortionMode::HardClip => driven.clamp(-1.0, 1.0),
            DistortionMode::Tube => driven / (1.0 + driven.abs()),
            DistortionMode::Fuzz => driven.signum() * (1.0 - (-driven.abs() * 5.0).exp()),
            DistortionMode::BitCrush(bits) => {
                let steps = (1 << bits) as f32;
                (driven * steps).round() / steps
            }
        };
        input * (1.0 - self.mix) + distorted * self.mix
    }
}

/// Chorus
pub struct Chorus {
    pub rate: f32,
    pub depth: f32,
    pub mix: f32,
    pub voices: u32,
    delay_buffer: VecDeque<f32>,
    phase: f32,
}

impl Chorus {
    pub fn new(sample_rate: f32) -> Self {
        Self { rate: 1.5, depth: 0.002, mix: 0.5, voices: 3, delay_buffer: VecDeque::from(vec![0.0; (sample_rate * 0.05) as usize]), phase: 0.0 }
    }

    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        self.delay_buffer.push_back(input);
        self.delay_buffer.pop_front();
        
        self.phase += self.rate / sample_rate;
        if self.phase >= 1.0 { self.phase -= 1.0; }
        
        let mut output = input;
        for v in 0..self.voices {
            let voice_phase = self.phase + (v as f32 / self.voices as f32);
            let delay_time = 0.02 + (voice_phase * std::f32::consts::TAU).sin() * self.depth;
            let delay_samples = (delay_time * sample_rate) as usize;
            if delay_samples < self.delay_buffer.len() {
                output += self.delay_buffer[self.delay_buffer.len() - 1 - delay_samples] / self.voices as f32;
            }
        }
        input * (1.0 - self.mix) + output * self.mix
    }
}

/// Flanger
pub struct Flanger {
    pub rate: f32,
    pub depth: f32,
    pub feedback: f32,
    pub mix: f32,
    delay_buffer: VecDeque<f32>,
    phase: f32,
}

impl Flanger {
    pub fn new(sample_rate: f32) -> Self {
        Self { rate: 0.5, depth: 0.002, feedback: 0.7, mix: 0.5, delay_buffer: VecDeque::from(vec![0.0; (sample_rate * 0.02) as usize]), phase: 0.0 }
    }

    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        self.phase += self.rate / sample_rate;
        if self.phase >= 1.0 { self.phase -= 1.0; }
        
        let delay_time = 0.001 + (self.phase * std::f32::consts::TAU).sin() * 0.5 * self.depth + 0.5 * self.depth;
        let delay_samples = (delay_time * sample_rate) as usize;
        
        let delayed = if delay_samples < self.delay_buffer.len() { self.delay_buffer[self.delay_buffer.len() - 1 - delay_samples] } else { 0.0 };
        let output = input + delayed * self.feedback;
        
        self.delay_buffer.push_back(output);
        self.delay_buffer.pop_front();
        
        input * (1.0 - self.mix) + output * self.mix
    }
}

/// Phaser
pub struct Phaser {
    pub rate: f32,
    pub depth: f32,
    pub stages: u32,
    pub feedback: f32,
    all_pass: Vec<f32>,
    phase: f32,
}

/// Delay
pub struct Delay {
    pub time: f32,
    pub feedback: f32,
    pub mix: f32,
    pub ping_pong: bool,
    buffer_l: VecDeque<f32>,
    buffer_r: VecDeque<f32>,
}

/// Convolution reverb stub (would use IR)
pub struct ConvolutionReverb {
    pub ir_path: String,
    pub mix: f32,
    pub predelay: f32,
}

/// Compressor
pub struct Compressor {
    pub threshold: f32,
    pub ratio: f32,
    pub attack: f32,
    pub release: f32,
    pub knee: f32,
    pub makeup_gain: f32,
    envelope: f32,
}

impl Compressor {
    pub fn new() -> Self {
        Self { threshold: -20.0, ratio: 4.0, attack: 0.01, release: 0.1, knee: 6.0, makeup_gain: 0.0, envelope: 0.0 }
    }

    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        let input_db = 20.0 * (input.abs() + 0.0001).log10();
        let over = input_db - self.threshold;
        
        let gain_reduction = if over <= -self.knee / 2.0 { 0.0 }
            else if over >= self.knee / 2.0 { over * (1.0 - 1.0 / self.ratio) }
            else { (over + self.knee / 2.0).powi(2) / (2.0 * self.knee) * (1.0 - 1.0 / self.ratio) };
        
        let target_env = gain_reduction;
        let coeff = if target_env > self.envelope { (-1.0 / (self.attack * sample_rate)).exp() }
            else { (-1.0 / (self.release * sample_rate)).exp() };
        self.envelope = self.envelope * coeff + target_env * (1.0 - coeff);
        
        let gain = 10.0f32.powf((-self.envelope + self.makeup_gain) / 20.0);
        input * gain
    }
}

/// Limiter
pub struct Limiter {
    pub ceiling: f32,
    pub release: f32,
    gain: f32,
}

impl Limiter {
    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        let target = if input.abs() > self.ceiling { self.ceiling / input.abs() } else { 1.0 };
        self.gain = self.gain.min(target);
        self.gain += (1.0 - self.gain) * (1.0 / (self.release * sample_rate));
        input * self.gain
    }
}

/// Parametric EQ
pub struct ParametricEQ {
    pub bands: Vec<EQBand>,
}

/// EQ band
pub struct EQBand {
    pub frequency: f32,
    pub gain: f32,
    pub q: f32,
    pub band_type: BandType,
}

/// Band type
pub enum BandType { LowShelf, HighShelf, Peak, LowPass, HighPass, Notch }

/// Pitch shifter (granular)
pub struct PitchShifter {
    pub semitones: f32,
    pub grain_size: f32,
    pub overlap: f32,
    buffer: VecDeque<f32>,
}

/// Vocoder
pub struct Vocoder {
    pub bands: u32,
    pub attack: f32,
    pub release: f32,
    band_envelopes: Vec<f32>,
}

/// Biquad filter
pub struct BiquadFilter {
    pub filter_type: FilterType,
    pub frequency: f32,
    pub q: f32,
    pub gain: f32,
    x1: f32, x2: f32, y1: f32, y2: f32,
    b0: f32, b1: f32, b2: f32, a1: f32, a2: f32,
}

/// Filter type
pub enum FilterType { LowPass, HighPass, BandPass, Notch, AllPass, LowShelf, HighShelf, Peak }

impl BiquadFilter {
    pub fn process(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.b1 * self.x1 + self.b2 * self.x2 - self.a1 * self.y1 - self.a2 * self.y2;
        self.x2 = self.x1; self.x1 = input;
        self.y2 = self.y1; self.y1 = output;
        output
    }
}

impl DSPChain {
    pub fn new(sample_rate: f32) -> Self { Self { effects: Vec::new(), sample_rate, enabled: true } }

    pub fn add(&mut self, effect: DSPEffect) { self.effects.push(effect); }

    pub fn process(&mut self, mut sample: f32) -> f32 {
        if !self.enabled { return sample; }
        for effect in &mut self.effects {
            sample = match effect {
                DSPEffect::Distortion(d) => d.process(sample),
                DSPEffect::Chorus(c) => c.process(sample, self.sample_rate),
                DSPEffect::Compressor(c) => c.process(sample, self.sample_rate),
                _ => sample,
            };
        }
        sample
    }
}
