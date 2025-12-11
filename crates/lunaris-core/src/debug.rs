//! Debug and Development Tools
//!
//! Visual debugger, console commands, cheats, and overlays.

use std::collections::HashMap;

/// Debug system
pub struct DebugSystem {
    pub console: DebugConsole,
    pub overlays: PerformanceOverlays,
    pub visual: VisualDebugger,
    pub cheats: CheatSystem,
    pub enabled: bool,
}

/// Debug console
pub struct DebugConsole {
    pub visible: bool,
    pub history: Vec<ConsoleEntry>,
    pub commands: HashMap<String, ConsoleCommand>,
    pub input_buffer: String,
    pub history_index: usize,
    pub command_history: Vec<String>,
    pub autocomplete: Vec<String>,
}

/// Console entry
pub struct ConsoleEntry {
    pub text: String,
    pub entry_type: EntryType,
    pub timestamp: f64,
}

/// Entry type
pub enum EntryType { Input, Output, Warning, Error, Info }

/// Console command
pub struct ConsoleCommand {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub handler: fn(&[&str]) -> Result<String, String>,
}

impl DebugConsole {
    pub fn new() -> Self {
        let mut console = Self {
            visible: false, history: Vec::new(), commands: HashMap::new(),
            input_buffer: String::new(), history_index: 0, command_history: Vec::new(),
            autocomplete: Vec::new(),
        };
        console.register_default_commands();
        console
    }

    fn register_default_commands(&mut self) {
        self.register("help", "Show available commands", "help [command]", |args| {
            Ok("Available: help, clear, quit, god, noclip, ghost, give, spawn, teleport, timescale, fov".into())
        });
        self.register("clear", "Clear console", "clear", |_| Ok("".into()));
        self.register("quit", "Quit game", "quit", |_| Ok("Quitting...".into()));
    }

    pub fn register(&mut self, name: &str, desc: &str, usage: &str, handler: fn(&[&str]) -> Result<String, String>) {
        self.commands.insert(name.into(), ConsoleCommand { name: name.into(), description: desc.into(), usage: usage.into(), handler });
    }

    pub fn execute(&mut self, input: &str) -> Result<String, String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() { return Ok(String::new()); }

        self.history.push(ConsoleEntry { text: format!("> {}", input), entry_type: EntryType::Input, timestamp: 0.0 });
        self.command_history.push(input.into());

        if let Some(cmd) = self.commands.get(parts[0]) {
            let result = (cmd.handler)(&parts[1..]);
            match &result {
                Ok(msg) => self.history.push(ConsoleEntry { text: msg.clone(), entry_type: EntryType::Output, timestamp: 0.0 }),
                Err(msg) => self.history.push(ConsoleEntry { text: msg.clone(), entry_type: EntryType::Error, timestamp: 0.0 }),
            }
            result
        } else {
            let err = format!("Unknown command: {}", parts[0]);
            self.history.push(ConsoleEntry { text: err.clone(), entry_type: EntryType::Error, timestamp: 0.0 });
            Err(err)
        }
    }

    pub fn toggle(&mut self) { self.visible = !self.visible; }
    pub fn log(&mut self, text: &str) { self.history.push(ConsoleEntry { text: text.into(), entry_type: EntryType::Info, timestamp: 0.0 }); }
    pub fn warn(&mut self, text: &str) { self.history.push(ConsoleEntry { text: text.into(), entry_type: EntryType::Warning, timestamp: 0.0 }); }
    pub fn error(&mut self, text: &str) { self.history.push(ConsoleEntry { text: text.into(), entry_type: EntryType::Error, timestamp: 0.0 }); }
}

/// Performance overlays
pub struct PerformanceOverlays {
    pub fps_counter: bool,
    pub frame_time_graph: bool,
    pub memory_usage: bool,
    pub gpu_stats: bool,
    pub draw_calls: bool,
    pub triangle_count: bool,
    pub network_stats: bool,
    pub physics_stats: bool,
    pub position: OverlayPosition,
}

/// Overlay position
pub enum OverlayPosition { TopLeft, TopRight, BottomLeft, BottomRight }

impl Default for PerformanceOverlays {
    fn default() -> Self {
        Self {
            fps_counter: true, frame_time_graph: true, memory_usage: false, gpu_stats: false,
            draw_calls: false, triangle_count: false, network_stats: false, physics_stats: false,
            position: OverlayPosition::TopLeft,
        }
    }
}

impl PerformanceOverlays {
    pub fn format(&self, stats: &PerformanceStats) -> String {
        let mut lines = Vec::new();
        if self.fps_counter { lines.push(format!("FPS: {:.0}", stats.fps)); }
        if self.frame_time_graph { lines.push(format!("Frame: {:.2}ms", stats.frame_time * 1000.0)); }
        if self.memory_usage { lines.push(format!("RAM: {:.0}MB", stats.memory_mb)); }
        if self.gpu_stats { lines.push(format!("GPU: {:.0}MB", stats.gpu_memory_mb)); }
        if self.draw_calls { lines.push(format!("Draw: {}", stats.draw_calls)); }
        if self.triangle_count { lines.push(format!("Tris: {}K", stats.triangles / 1000)); }
        lines.join("\n")
    }
}

/// Performance stats
pub struct PerformanceStats {
    pub fps: f32,
    pub frame_time: f32,
    pub memory_mb: f32,
    pub gpu_memory_mb: f32,
    pub draw_calls: u32,
    pub triangles: u64,
}

/// Visual debugger
pub struct VisualDebugger {
    pub draw_requests: Vec<DebugDraw>,
    pub persistent: Vec<DebugDraw>,
    pub enabled: bool,
}

/// Debug draw
pub struct DebugDraw {
    pub draw_type: DrawType,
    pub color: [f32; 4],
    pub duration: f32,
    pub depth_test: bool,
}

/// Draw type
pub enum DrawType {
    Line { start: [f32; 3], end: [f32; 3] },
    Box { center: [f32; 3], size: [f32; 3] },
    Sphere { center: [f32; 3], radius: f32 },
    Arrow { start: [f32; 3], end: [f32; 3] },
    Text { position: [f32; 3], text: String },
    Capsule { start: [f32; 3], end: [f32; 3], radius: f32 },
}

impl VisualDebugger {
    pub fn new() -> Self { Self { draw_requests: Vec::new(), persistent: Vec::new(), enabled: true } }

    pub fn line(&mut self, start: [f32; 3], end: [f32; 3], color: [f32; 4]) {
        self.draw_requests.push(DebugDraw { draw_type: DrawType::Line { start, end }, color, duration: 0.0, depth_test: true });
    }

    pub fn sphere(&mut self, center: [f32; 3], radius: f32, color: [f32; 4]) {
        self.draw_requests.push(DebugDraw { draw_type: DrawType::Sphere { center, radius }, color, duration: 0.0, depth_test: true });
    }

    pub fn box3d(&mut self, center: [f32; 3], size: [f32; 3], color: [f32; 4]) {
        self.draw_requests.push(DebugDraw { draw_type: DrawType::Box { center, size }, color, duration: 0.0, depth_test: true });
    }

    pub fn text(&mut self, position: [f32; 3], text: &str, color: [f32; 4]) {
        self.draw_requests.push(DebugDraw { draw_type: DrawType::Text { position, text: text.into() }, color, duration: 0.0, depth_test: false });
    }

    pub fn clear(&mut self) { self.draw_requests.clear(); }
}

/// Cheat system
pub struct CheatSystem {
    pub cheats: HashMap<String, CheatState>,
    pub enabled: bool,
}

/// Cheat state
pub struct CheatState {
    pub name: String,
    pub active: bool,
    pub value: CheatValue,
}

/// Cheat value
pub enum CheatValue { None, Float(f32), Int(i32), Bool(bool) }

impl CheatSystem {
    pub fn new() -> Self {
        let mut cheats = HashMap::new();
        cheats.insert("god".into(), CheatState { name: "God Mode".into(), active: false, value: CheatValue::None });
        cheats.insert("noclip".into(), CheatState { name: "No Clip".into(), active: false, value: CheatValue::None });
        cheats.insert("ghost".into(), CheatState { name: "Ghost".into(), active: false, value: CheatValue::None });
        cheats.insert("infinite_ammo".into(), CheatState { name: "Infinite Ammo".into(), active: false, value: CheatValue::None });
        cheats.insert("timescale".into(), CheatState { name: "Time Scale".into(), active: false, value: CheatValue::Float(1.0) });
        Self { cheats, enabled: true }
    }

    pub fn toggle(&mut self, name: &str) -> bool {
        if let Some(cheat) = self.cheats.get_mut(name) {
            cheat.active = !cheat.active;
            cheat.active
        } else { false }
    }

    pub fn set(&mut self, name: &str, value: CheatValue) {
        if let Some(cheat) = self.cheats.get_mut(name) { cheat.value = value; cheat.active = true; }
    }

    pub fn is_active(&self, name: &str) -> bool {
        self.enabled && self.cheats.get(name).map(|c| c.active).unwrap_or(false)
    }
}

impl DebugSystem {
    pub fn new() -> Self {
        Self { console: DebugConsole::new(), overlays: PerformanceOverlays::default(), visual: VisualDebugger::new(), cheats: CheatSystem::new(), enabled: true }
    }
}
