//! Accessibility System
//!
//! Color-blind modes, text-to-speech, remapping, and subtitles.

use std::collections::HashMap;

/// Accessibility system
pub struct Accessibility {
    pub vision: VisionSettings,
    pub audio: AudioAccessibility,
    pub motor: MotorAccessibility,
    pub cognitive: CognitiveAccessibility,
}

/// Vision settings
pub struct VisionSettings {
    pub color_blind_mode: ColorBlindMode,
    pub color_blind_strength: f32,
    pub high_contrast: bool,
    pub text_size: TextSize,
    pub ui_scale: f32,
    pub screen_reader: bool,
    pub reduce_motion: bool,
    pub flash_reduction: bool,
}

/// Color blind mode
#[derive(Clone, Copy)]
pub enum ColorBlindMode { None, Protanopia, Deuteranopia, Tritanopia, Achromatopsia }

/// Text size
#[derive(Clone, Copy)]
pub enum TextSize { Small, Normal, Large, XLarge }

/// Audio accessibility
pub struct AudioAccessibility {
    pub subtitles: SubtitleSettings,
    pub closed_captions: bool,
    pub visual_sound_cues: bool,
    pub mono_audio: bool,
    pub text_to_speech: bool,
    pub speech_rate: f32,
}

/// Subtitle settings
pub struct SubtitleSettings {
    pub enabled: bool,
    pub size: TextSize,
    pub background: bool,
    pub background_opacity: f32,
    pub speaker_names: bool,
    pub speaker_colors: bool,
    pub direction_indicators: bool,
}

/// Motor accessibility
pub struct MotorAccessibility {
    pub one_handed_mode: bool,
    pub hold_to_toggle: Vec<String>,
    pub auto_aim: bool,
    pub auto_aim_strength: f32,
    pub qte_assist: bool,
    pub input_buffering: f32,
    pub sticky_keys: bool,
}

/// Cognitive accessibility
pub struct CognitiveAccessibility {
    pub simplified_ui: bool,
    pub hints_enabled: bool,
    pub hint_frequency: HintFrequency,
    pub objective_reminders: bool,
    pub navigation_assist: bool,
    pub difficulty_adjust: bool,
}

/// Hint frequency
pub enum HintFrequency { Minimal, Normal, Frequent, Always }

impl Default for Accessibility {
    fn default() -> Self {
        Self {
            vision: VisionSettings {
                color_blind_mode: ColorBlindMode::None, color_blind_strength: 1.0, high_contrast: false,
                text_size: TextSize::Normal, ui_scale: 1.0, screen_reader: false, reduce_motion: false, flash_reduction: false,
            },
            audio: AudioAccessibility {
                subtitles: SubtitleSettings { enabled: false, size: TextSize::Normal, background: true, background_opacity: 0.7, speaker_names: true, speaker_colors: true, direction_indicators: false },
                closed_captions: false, visual_sound_cues: false, mono_audio: false, text_to_speech: false, speech_rate: 1.0,
            },
            motor: MotorAccessibility {
                one_handed_mode: false, hold_to_toggle: Vec::new(), auto_aim: false, auto_aim_strength: 0.5, qte_assist: false, input_buffering: 0.1, sticky_keys: false,
            },
            cognitive: CognitiveAccessibility {
                simplified_ui: false, hints_enabled: true, hint_frequency: HintFrequency::Normal, objective_reminders: true, navigation_assist: false, difficulty_adjust: false,
            },
        }
    }
}

impl Accessibility {
    pub fn new() -> Self { Self::default() }

    pub fn apply_color_blind(&self, color: [f32; 3]) -> [f32; 3] {
        if matches!(self.vision.color_blind_mode, ColorBlindMode::None) { return color; }
        
        let matrix = match self.vision.color_blind_mode {
            ColorBlindMode::Protanopia => [[0.567, 0.433, 0.0], [0.558, 0.442, 0.0], [0.0, 0.242, 0.758]],
            ColorBlindMode::Deuteranopia => [[0.625, 0.375, 0.0], [0.7, 0.3, 0.0], [0.0, 0.3, 0.7]],
            ColorBlindMode::Tritanopia => [[0.95, 0.05, 0.0], [0.0, 0.433, 0.567], [0.0, 0.475, 0.525]],
            ColorBlindMode::Achromatopsia => [[0.299, 0.587, 0.114], [0.299, 0.587, 0.114], [0.299, 0.587, 0.114]],
            ColorBlindMode::None => [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        };
        
        let s = self.vision.color_blind_strength;
        [
            color[0] * (1.0 - s + s * matrix[0][0]) + color[1] * s * matrix[0][1] + color[2] * s * matrix[0][2],
            color[0] * s * matrix[1][0] + color[1] * (1.0 - s + s * matrix[1][1]) + color[2] * s * matrix[1][2],
            color[0] * s * matrix[2][0] + color[1] * s * matrix[2][1] + color[2] * (1.0 - s + s * matrix[2][2]),
        ]
    }

    pub fn get_text_scale(&self) -> f32 {
        match self.vision.text_size {
            TextSize::Small => 0.8,
            TextSize::Normal => 1.0,
            TextSize::Large => 1.25,
            TextSize::XLarge => 1.5,
        } * self.vision.ui_scale
    }
}

/// Input remapping
pub struct InputRemapping {
    pub mappings: HashMap<String, Vec<InputBinding>>,
    pub presets: Vec<RemapPreset>,
    pub current_preset: String,
}

/// Input binding
pub struct InputBinding {
    pub device: InputDevice,
    pub key: String,
    pub modifiers: Vec<String>,
}

/// Input device
pub enum InputDevice { Keyboard, Mouse, Gamepad, Touch }

/// Remap preset
pub struct RemapPreset {
    pub name: String,
    pub mappings: HashMap<String, Vec<InputBinding>>,
}

impl InputRemapping {
    pub fn new() -> Self {
        Self { mappings: HashMap::new(), presets: Vec::new(), current_preset: "default".into() }
    }

    pub fn remap(&mut self, action: &str, binding: InputBinding) {
        self.mappings.entry(action.into()).or_insert_with(Vec::new).push(binding);
    }

    pub fn clear(&mut self, action: &str) {
        self.mappings.remove(action);
    }

    pub fn save_preset(&mut self, name: &str) {
        self.presets.push(RemapPreset { name: name.into(), mappings: self.mappings.clone() });
    }

    pub fn load_preset(&mut self, name: &str) -> Result<(), String> {
        if let Some(preset) = self.presets.iter().find(|p| p.name == name) {
            self.mappings = preset.mappings.clone();
            self.current_preset = name.into();
            Ok(())
        } else { Err("Preset not found".into()) }
    }
}
