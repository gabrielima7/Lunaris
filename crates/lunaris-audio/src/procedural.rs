//! Procedural Audio
//!
//! Synthesis and real-time audio generation.

use std::f32::consts::PI;

/// Oscillator waveform
#[derive(Debug, Clone, Copy, Default)]
pub enum Waveform {
    /// Sine wave
    #[default]
    Sine,
    /// Square wave
    Square,
    /// Sawtooth wave
    Sawtooth,
    /// Triangle wave
    Triangle,
    /// White noise
    Noise,
    /// Pulse with duty cycle
    Pulse(f32),
}

/// Basic oscillator
#[derive(Debug, Clone)]
pub struct Oscillator {
    /// Waveform type
    pub waveform: Waveform,
    /// Frequency (Hz)
    pub frequency: f32,
    /// Amplitude (0-1)
    pub amplitude: f32,
    /// Phase
    phase: f32,
    /// Sample rate
    sample_rate: f32,
}

impl Oscillator {
    /// Create a new oscillator
    #[must_use]
    pub fn new(waveform: Waveform, frequency: f32, sample_rate: f32) -> Self {
        Self {
            waveform,
            frequency,
            amplitude: 1.0,
            phase: 0.0,
            sample_rate,
        }
    }

    /// Generate next sample
    pub fn next_sample(&mut self) -> f32 {
        let sample = match self.waveform {
            Waveform::Sine => (self.phase * 2.0 * PI).sin(),
            Waveform::Square => if self.phase < 0.5 { 1.0 } else { -1.0 },
            Waveform::Sawtooth => 2.0 * self.phase - 1.0,
            Waveform::Triangle => {
                if self.phase < 0.5 {
                    4.0 * self.phase - 1.0
                } else {
                    -4.0 * self.phase + 3.0
                }
            }
            Waveform::Noise => {
                let seed = (self.phase * 1000.0) as u32;
                ((seed.wrapping_mul(1103515245).wrapping_add(12345)) as f32 / u32::MAX as f32) * 2.0 - 1.0
            }
            Waveform::Pulse(duty) => if self.phase < duty { 1.0 } else { -1.0 },
        };

        // Advance phase
        self.phase += self.frequency / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sample * self.amplitude
    }

    /// Reset phase
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }
}

/// ADSR envelope
#[derive(Debug, Clone)]
pub struct Envelope {
    /// Attack time (seconds)
    pub attack: f32,
    /// Decay time (seconds)
    pub decay: f32,
    /// Sustain level (0-1)
    pub sustain: f32,
    /// Release time (seconds)
    pub release: f32,
    /// Current value
    value: f32,
    /// Current stage
    stage: EnvelopeStage,
    /// Time in current stage
    time: f32,
    /// Sample rate
    sample_rate: f32,
}

/// Envelope stage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvelopeStage {
    /// Not started
    Idle,
    /// Attack phase
    Attack,
    /// Decay phase
    Decay,
    /// Sustain phase
    Sustain,
    /// Release phase
    Release,
}

impl Envelope {
    /// Create a new envelope
    #[must_use]
    pub fn new(sample_rate: f32) -> Self {
        Self {
            attack: 0.01,
            decay: 0.1,
            sustain: 0.7,
            release: 0.3,
            value: 0.0,
            stage: EnvelopeStage::Idle,
            time: 0.0,
            sample_rate,
        }
    }

    /// Start the envelope (note on)
    pub fn trigger(&mut self) {
        self.stage = EnvelopeStage::Attack;
        self.time = 0.0;
    }

    /// Release the envelope (note off)
    pub fn release(&mut self) {
        if self.stage != EnvelopeStage::Idle && self.stage != EnvelopeStage::Release {
            self.stage = EnvelopeStage::Release;
            self.time = 0.0;
        }
    }

    /// Get next sample
    pub fn next_sample(&mut self) -> f32 {
        let dt = 1.0 / self.sample_rate;
        
        match self.stage {
            EnvelopeStage::Idle => {}
            EnvelopeStage::Attack => {
                self.value = self.time / self.attack;
                if self.value >= 1.0 {
                    self.value = 1.0;
                    self.stage = EnvelopeStage::Decay;
                    self.time = 0.0;
                }
            }
            EnvelopeStage::Decay => {
                self.value = 1.0 - (1.0 - self.sustain) * (self.time / self.decay);
                if self.value <= self.sustain {
                    self.value = self.sustain;
                    self.stage = EnvelopeStage::Sustain;
                }
            }
            EnvelopeStage::Sustain => {
                self.value = self.sustain;
            }
            EnvelopeStage::Release => {
                self.value = self.sustain * (1.0 - self.time / self.release);
                if self.value <= 0.0 {
                    self.value = 0.0;
                    self.stage = EnvelopeStage::Idle;
                }
            }
        }

        self.time += dt;
        self.value
    }

    /// Is envelope finished
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.stage == EnvelopeStage::Idle && self.value <= 0.0
    }
}

/// LFO (Low Frequency Oscillator)
#[derive(Debug, Clone)]
pub struct LFO {
    /// Oscillator
    oscillator: Oscillator,
    /// Depth (modulation amount)
    pub depth: f32,
    /// Offset
    pub offset: f32,
}

impl LFO {
    /// Create a new LFO
    #[must_use]
    pub fn new(frequency: f32, sample_rate: f32) -> Self {
        Self {
            oscillator: Oscillator::new(Waveform::Sine, frequency, sample_rate),
            depth: 1.0,
            offset: 0.0,
        }
    }

    /// Get next value
    pub fn next_value(&mut self) -> f32 {
        self.oscillator.next_sample() * self.depth + self.offset
    }

    /// Set frequency
    pub fn set_frequency(&mut self, freq: f32) {
        self.oscillator.frequency = freq;
    }

    /// Set waveform
    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.oscillator.waveform = waveform;
    }
}

/// Biquad filter
#[derive(Debug, Clone)]
pub struct BiquadFilter {
    /// Filter type
    pub filter_type: FilterType,
    /// Cutoff frequency
    pub cutoff: f32,
    /// Resonance (Q)
    pub resonance: f32,
    /// Sample rate
    sample_rate: f32,
    /// Coefficients
    a0: f32, a1: f32, a2: f32, b1: f32, b2: f32,
    /// Previous samples
    x1: f32, x2: f32, y1: f32, y2: f32,
}

/// Filter type
#[derive(Debug, Clone, Copy, Default)]
pub enum FilterType {
    /// Low pass
    #[default]
    LowPass,
    /// High pass
    HighPass,
    /// Band pass
    BandPass,
    /// Notch
    Notch,
    /// All pass
    AllPass,
}

impl BiquadFilter {
    /// Create a new filter
    #[must_use]
    pub fn new(filter_type: FilterType, cutoff: f32, resonance: f32, sample_rate: f32) -> Self {
        let mut filter = Self {
            filter_type,
            cutoff,
            resonance,
            sample_rate,
            a0: 0.0, a1: 0.0, a2: 0.0, b1: 0.0, b2: 0.0,
            x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0,
        };
        filter.update_coefficients();
        filter
    }

    /// Update filter coefficients
    pub fn update_coefficients(&mut self) {
        let omega = 2.0 * PI * self.cutoff / self.sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * self.resonance);

        match self.filter_type {
            FilterType::LowPass => {
                let b0 = (1.0 - cos_omega) / 2.0;
                let b1 = 1.0 - cos_omega;
                let b2 = (1.0 - cos_omega) / 2.0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_omega;
                let a2 = 1.0 - alpha;
                
                self.a0 = b0 / a0;
                self.a1 = b1 / a0;
                self.a2 = b2 / a0;
                self.b1 = a1 / a0;
                self.b2 = a2 / a0;
            }
            FilterType::HighPass => {
                let b0 = (1.0 + cos_omega) / 2.0;
                let b1 = -(1.0 + cos_omega);
                let b2 = (1.0 + cos_omega) / 2.0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_omega;
                let a2 = 1.0 - alpha;
                
                self.a0 = b0 / a0;
                self.a1 = b1 / a0;
                self.a2 = b2 / a0;
                self.b1 = a1 / a0;
                self.b2 = a2 / a0;
            }
            FilterType::BandPass => {
                let b0 = alpha;
                let b1 = 0.0;
                let b2 = -alpha;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_omega;
                let a2 = 1.0 - alpha;
                
                self.a0 = b0 / a0;
                self.a1 = b1 / a0;
                self.a2 = b2 / a0;
                self.b1 = a1 / a0;
                self.b2 = a2 / a0;
            }
            _ => {
                self.a0 = 1.0;
                self.a1 = 0.0;
                self.a2 = 0.0;
                self.b1 = 0.0;
                self.b2 = 0.0;
            }
        }
    }

    /// Process a sample
    pub fn process(&mut self, input: f32) -> f32 {
        let output = self.a0 * input + self.a1 * self.x1 + self.a2 * self.x2
                   - self.b1 * self.y1 - self.b2 * self.y2;
        
        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;
        
        output
    }

    /// Reset filter state
    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

/// Simple synthesizer
pub struct Synth {
    /// Oscillators
    oscillators: Vec<Oscillator>,
    /// Envelope
    pub envelope: Envelope,
    /// Filter
    pub filter: BiquadFilter,
    /// LFO for vibrato
    vibrato: LFO,
    /// Master volume
    pub volume: f32,
    /// Sample rate
    sample_rate: f32,
}

impl Synth {
    /// Create a new synth
    #[must_use]
    pub fn new(sample_rate: f32) -> Self {
        Self {
            oscillators: vec![
                Oscillator::new(Waveform::Sawtooth, 440.0, sample_rate),
            ],
            envelope: Envelope::new(sample_rate),
            filter: BiquadFilter::new(FilterType::LowPass, 2000.0, 0.7, sample_rate),
            vibrato: LFO::new(5.0, sample_rate),
            volume: 0.5,
            sample_rate,
        }
    }

    /// Note on
    pub fn note_on(&mut self, frequency: f32) {
        for osc in &mut self.oscillators {
            osc.frequency = frequency;
        }
        self.envelope.trigger();
    }

    /// Note off
    pub fn note_off(&mut self) {
        self.envelope.release();
    }

    /// Generate next sample
    pub fn next_sample(&mut self) -> f32 {
        let vibrato = self.vibrato.next_value() * 0.01;
        
        let mut sample = 0.0;
        for osc in &mut self.oscillators {
            let freq = osc.frequency * (1.0 + vibrato);
            osc.frequency = freq;
            sample += osc.next_sample();
        }
        
        sample /= self.oscillators.len() as f32;
        sample = self.filter.process(sample);
        sample *= self.envelope.next_sample();
        sample *= self.volume;
        
        sample
    }

    /// Generate buffer
    pub fn generate(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {
            *sample = self.next_sample();
        }
    }

    /// Is sound finished
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.envelope.is_finished()
    }
}

/// Noise generator types
#[derive(Debug, Clone, Copy)]
pub enum NoiseType {
    /// White noise
    White,
    /// Pink noise
    Pink,
    /// Brown noise
    Brown,
}

/// Noise generator
pub struct NoiseGenerator {
    /// Noise type
    pub noise_type: NoiseType,
    /// State for colored noise
    pink_state: [f32; 7],
    brown_state: f32,
    /// Random seed
    seed: u32,
}

impl NoiseGenerator {
    /// Create a new noise generator
    #[must_use]
    pub fn new(noise_type: NoiseType) -> Self {
        Self {
            noise_type,
            pink_state: [0.0; 7],
            brown_state: 0.0,
            seed: 12345,
        }
    }

    /// Generate next sample
    pub fn next_sample(&mut self) -> f32 {
        match self.noise_type {
            NoiseType::White => self.white(),
            NoiseType::Pink => self.pink(),
            NoiseType::Brown => self.brown(),
        }
    }

    fn white(&mut self) -> f32 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        (self.seed as f32 / u32::MAX as f32) * 2.0 - 1.0
    }

    fn pink(&mut self) -> f32 {
        let white = self.white();
        
        // Voss-McCartney algorithm
        self.pink_state[0] = 0.99886 * self.pink_state[0] + white * 0.0555179;
        self.pink_state[1] = 0.99332 * self.pink_state[1] + white * 0.0750759;
        self.pink_state[2] = 0.96900 * self.pink_state[2] + white * 0.1538520;
        self.pink_state[3] = 0.86650 * self.pink_state[3] + white * 0.3104856;
        self.pink_state[4] = 0.55000 * self.pink_state[4] + white * 0.5329522;
        self.pink_state[5] = -0.7616 * self.pink_state[5] - white * 0.0168980;
        
        let output = self.pink_state[0] + self.pink_state[1] + self.pink_state[2]
                   + self.pink_state[3] + self.pink_state[4] + self.pink_state[5]
                   + self.pink_state[6] + white * 0.5362;
        self.pink_state[6] = white * 0.115926;
        
        output * 0.11
    }

    fn brown(&mut self) -> f32 {
        let white = self.white();
        self.brown_state = (self.brown_state + white * 0.02).clamp(-1.0, 1.0);
        self.brown_state * 3.5
    }
}
