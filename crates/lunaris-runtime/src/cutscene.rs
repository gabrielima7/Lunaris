//! Cutscene System
//!
//! Pre-rendered/real-time mixing, letterboxing, and skip handling.

use glam::Vec2;

/// Cutscene player
pub struct CutscenePlayer {
    pub cutscenes: Vec<Cutscene>,
    pub current: Option<CutsceneState>,
    pub settings: CutsceneSettings,
}

/// Cutscene
pub struct Cutscene {
    pub id: String,
    pub name: String,
    pub content: CutsceneContent,
    pub duration: f32,
    pub skippable: bool,
    pub skip_delay: f32,
    pub subtitles: Vec<Subtitle>,
}

/// Cutscene content
pub enum CutsceneContent {
    Video { path: String, loop_video: bool },
    Sequence { sequence_id: u64 },
    Mixed { video: String, sequence_id: u64, blend: f32 },
}

/// Cutscene state
pub struct CutsceneState {
    pub cutscene_id: String,
    pub time: f32,
    pub paused: bool,
    pub skip_held: f32,
    pub letterbox: f32,
}

/// Cutscene settings
pub struct CutsceneSettings {
    pub letterbox_ratio: f32,
    pub letterbox_speed: f32,
    pub skip_hold_time: f32,
    pub show_skip_prompt: bool,
    pub subtitles_enabled: bool,
}

/// Subtitle
pub struct Subtitle {
    pub start: f32,
    pub end: f32,
    pub text: String,
    pub speaker: Option<String>,
    pub position: SubtitlePosition,
}

/// Subtitle position
pub enum SubtitlePosition { Bottom, Top, Custom(Vec2) }

impl Default for CutsceneSettings {
    fn default() -> Self {
        Self { letterbox_ratio: 2.35, letterbox_speed: 1.0, skip_hold_time: 1.0, show_skip_prompt: true, subtitles_enabled: true }
    }
}

impl CutscenePlayer {
    pub fn new() -> Self {
        Self { cutscenes: Vec::new(), current: None, settings: CutsceneSettings::default() }
    }

    pub fn register(&mut self, cutscene: Cutscene) {
        self.cutscenes.push(cutscene);
    }

    pub fn play(&mut self, id: &str) -> Result<(), String> {
        if !self.cutscenes.iter().any(|c| c.id == id) { return Err("Cutscene not found".into()); }
        self.current = Some(CutsceneState { cutscene_id: id.into(), time: 0.0, paused: false, skip_held: 0.0, letterbox: 0.0 });
        Ok(())
    }

    pub fn stop(&mut self) {
        self.current = None;
    }

    pub fn pause(&mut self) {
        if let Some(state) = &mut self.current { state.paused = true; }
    }

    pub fn resume(&mut self) {
        if let Some(state) = &mut self.current { state.paused = false; }
    }

    pub fn update(&mut self, dt: f32, skip_pressed: bool) -> CutsceneEvent {
        let Some(state) = &mut self.current else { return CutsceneEvent::None };
        let cutscene = self.cutscenes.iter().find(|c| c.id == state.cutscene_id);
        let Some(cutscene) = cutscene else { return CutsceneEvent::None };
        
        // Letterbox animation
        let target_letterbox = 1.0;
        state.letterbox += (target_letterbox - state.letterbox) * self.settings.letterbox_speed * dt;
        
        if state.paused { return CutsceneEvent::None; }
        
        // Skip handling
        if cutscene.skippable && state.time >= cutscene.skip_delay {
            if skip_pressed {
                state.skip_held += dt;
                if state.skip_held >= self.settings.skip_hold_time {
                    self.current = None;
                    return CutsceneEvent::Skipped;
                }
                return CutsceneEvent::SkipProgress(state.skip_held / self.settings.skip_hold_time);
            } else {
                state.skip_held = 0.0;
            }
        }
        
        // Time advance
        state.time += dt;
        if state.time >= cutscene.duration {
            self.current = None;
            return CutsceneEvent::Finished;
        }
        
        CutsceneEvent::Playing
    }

    pub fn get_subtitle(&self) -> Option<&Subtitle> {
        let state = self.current.as_ref()?;
        let cutscene = self.cutscenes.iter().find(|c| c.id == state.cutscene_id)?;
        cutscene.subtitles.iter().find(|s| state.time >= s.start && state.time < s.end)
    }

    pub fn get_letterbox(&self) -> f32 {
        self.current.as_ref().map(|s| s.letterbox).unwrap_or(0.0)
    }

    pub fn is_playing(&self) -> bool { self.current.is_some() }
}

/// Cutscene event
pub enum CutsceneEvent { None, Playing, Skipped, Finished, SkipProgress(f32) }
