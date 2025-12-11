//! Input Recording
//!
//! Demo recording, input macros, and automated testing inputs.

use std::collections::HashMap;

/// Input recorder
pub struct InputRecorder {
    pub state: RecorderState,
    pub recording: InputRecording,
    pub playback: PlaybackState,
    pub macros: HashMap<String, InputMacro>,
}

/// Recorder state
pub enum RecorderState { Idle, Recording, Playing }

/// Input recording
pub struct InputRecording {
    pub name: String,
    pub frames: Vec<InputFrame>,
    pub duration: f32,
    pub metadata: RecordingMetadata,
}

/// Recording metadata
pub struct RecordingMetadata {
    pub version: String,
    pub date: String,
    pub game_version: String,
    pub seed: u64,
    pub level: String,
}

/// Input frame
#[derive(Clone)]
pub struct InputFrame {
    pub time: f32,
    pub inputs: FrameInputs,
    pub random_seed: Option<u64>,
}

/// Frame inputs
#[derive(Clone, Default)]
pub struct FrameInputs {
    pub axes: HashMap<String, f32>,
    pub buttons_pressed: Vec<String>,
    pub buttons_released: Vec<String>,
    pub buttons_held: Vec<String>,
    pub mouse_position: [f32; 2],
    pub mouse_delta: [f32; 2],
}

/// Playback state
pub struct PlaybackState {
    pub current_frame: usize,
    pub time: f32,
    pub speed: f32,
    pub loop_playback: bool,
    pub paused: bool,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self { current_frame: 0, time: 0.0, speed: 1.0, loop_playback: false, paused: false }
    }
}

impl InputRecorder {
    pub fn new() -> Self {
        Self {
            state: RecorderState::Idle,
            recording: InputRecording { name: String::new(), frames: Vec::new(), duration: 0.0, metadata: RecordingMetadata::default() },
            playback: PlaybackState::default(),
            macros: HashMap::new(),
        }
    }

    pub fn start_recording(&mut self, name: &str) {
        self.recording = InputRecording {
            name: name.into(),
            frames: Vec::new(),
            duration: 0.0,
            metadata: RecordingMetadata { version: "1.0".into(), date: "2024-01-01".into(), game_version: "1.0.0".into(), seed: 0, level: String::new() },
        };
        self.state = RecorderState::Recording;
    }

    pub fn stop_recording(&mut self) -> &InputRecording {
        self.state = RecorderState::Idle;
        &self.recording
    }

    pub fn record_frame(&mut self, time: f32, inputs: FrameInputs) {
        if !matches!(self.state, RecorderState::Recording) { return; }
        self.recording.frames.push(InputFrame { time, inputs, random_seed: None });
        self.recording.duration = time;
    }

    pub fn start_playback(&mut self) {
        self.playback = PlaybackState::default();
        self.state = RecorderState::Playing;
    }

    pub fn stop_playback(&mut self) {
        self.state = RecorderState::Idle;
    }

    pub fn update_playback(&mut self, dt: f32) -> Option<&FrameInputs> {
        if !matches!(self.state, RecorderState::Playing) || self.playback.paused { return None; }
        
        self.playback.time += dt * self.playback.speed;
        
        // Find current frame
        while self.playback.current_frame < self.recording.frames.len() {
            let frame = &self.recording.frames[self.playback.current_frame];
            if frame.time <= self.playback.time {
                self.playback.current_frame += 1;
                return Some(&frame.inputs);
            } else { break; }
        }

        // End of recording
        if self.playback.current_frame >= self.recording.frames.len() {
            if self.playback.loop_playback {
                self.playback.current_frame = 0;
                self.playback.time = 0.0;
            } else {
                self.state = RecorderState::Idle;
            }
        }
        None
    }

    pub fn save(&self, path: &str) -> Result<(), String> {
        // Would serialize to file
        Ok(())
    }

    pub fn load(&mut self, path: &str) -> Result<(), String> {
        // Would deserialize from file
        Ok(())
    }
}

impl Default for RecordingMetadata {
    fn default() -> Self {
        Self { version: "1.0".into(), date: String::new(), game_version: String::new(), seed: 0, level: String::new() }
    }
}

/// Input macro
pub struct InputMacro {
    pub name: String,
    pub inputs: Vec<MacroInput>,
    pub loop_macro: bool,
}

/// Macro input
pub struct MacroInput {
    pub delay: f32,
    pub input: MacroInputType,
}

/// Macro input type
pub enum MacroInputType {
    Press(String),
    Release(String),
    Hold(String, f32),
    Axis(String, f32),
    MouseMove([f32; 2]),
    Wait(f32),
}

impl InputRecorder {
    pub fn register_macro(&mut self, name: &str, inputs: Vec<MacroInput>) {
        self.macros.insert(name.into(), InputMacro { name: name.into(), inputs, loop_macro: false });
    }

    pub fn execute_macro(&self, name: &str) -> Option<&InputMacro> {
        self.macros.get(name)
    }
}

/// Macro player
pub struct MacroPlayer {
    pub current_macro: Option<String>,
    pub step: usize,
    pub time: f32,
    pub playing: bool,
}

impl MacroPlayer {
    pub fn new() -> Self { Self { current_macro: None, step: 0, time: 0.0, playing: false } }

    pub fn play(&mut self, name: &str) {
        self.current_macro = Some(name.into());
        self.step = 0;
        self.time = 0.0;
        self.playing = true;
    }

    pub fn update(&mut self, dt: f32, macros: &HashMap<String, InputMacro>) -> Option<&MacroInputType> {
        if !self.playing { return None; }
        let name = self.current_macro.as_ref()?;
        let m = macros.get(name)?;
        
        if self.step >= m.inputs.len() {
            if m.loop_macro { self.step = 0; self.time = 0.0; }
            else { self.playing = false; return None; }
        }

        self.time += dt;
        let input = &m.inputs[self.step];
        if self.time >= input.delay {
            self.step += 1;
            self.time = 0.0;
            return Some(&input.input);
        }
        None
    }
}
