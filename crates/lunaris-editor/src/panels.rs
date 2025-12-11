//! Editor panels

use crate::ui::UiContext;
use lunaris_core::math::Rect;

/// Hierarchy panel showing scene tree
pub struct HierarchyPanel {
    /// Panel bounds
    pub bounds: Rect,
    /// Selected entity ID
    pub selected: Option<u64>,
    /// Expanded nodes
    pub expanded: Vec<u64>,
}

impl Default for HierarchyPanel {
    fn default() -> Self {
        Self {
            bounds: Rect::new(0.0, 80.0, 250.0, 400.0),
            selected: None,
            expanded: Vec::new(),
        }
    }
}

impl HierarchyPanel {
    /// Create a new hierarchy panel
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Draw the panel
    pub fn draw(&mut self, ui: &mut UiContext, _entities: &[(u64, String, Option<u64>)]) {
        ui.panel("Hierarchy", self.bounds, |ui| {
            ui.label("Scene Objects");
            ui.separator();
            // Entities would be drawn here
        });
    }
}

/// Inspector panel showing entity properties
pub struct InspectorPanel {
    /// Panel bounds
    pub bounds: Rect,
}

impl Default for InspectorPanel {
    fn default() -> Self {
        Self {
            bounds: Rect::new(1350.0, 80.0, 250.0, 600.0),
        }
    }
}

impl InspectorPanel {
    /// Create a new inspector panel
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Draw the panel
    pub fn draw(&mut self, ui: &mut UiContext, _selected_entity: Option<u64>) {
        ui.panel("Inspector", self.bounds, |ui| {
            ui.label("Properties");
        });
    }
}

/// Console panel for logging
pub struct ConsolePanel {
    /// Panel bounds
    pub bounds: Rect,
    /// Log messages
    pub messages: Vec<LogMessage>,
    /// Max messages to keep
    pub max_messages: usize,
    /// Filter level
    pub filter_level: LogLevel,
}

/// Log message
#[derive(Debug, Clone)]
pub struct LogMessage {
    /// Message content
    pub content: String,
    /// Log level
    pub level: LogLevel,
    /// Timestamp
    pub timestamp: std::time::Instant,
}

/// Log level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogLevel {
    /// Trace level
    Trace,
    /// Debug level
    Debug,
    /// Info level
    #[default]
    Info,
    /// Warning level
    Warn,
    /// Error level
    Error,
}

impl Default for ConsolePanel {
    fn default() -> Self {
        Self {
            bounds: Rect::new(260.0, 520.0, 1080.0, 160.0),
            messages: Vec::new(),
            max_messages: 1000,
            filter_level: LogLevel::Info,
        }
    }
}

impl ConsolePanel {
    /// Create a new console panel
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a log message
    pub fn log(&mut self, level: LogLevel, content: impl Into<String>) {
        self.messages.push(LogMessage {
            content: content.into(),
            level,
            timestamp: std::time::Instant::now(),
        });

        if self.messages.len() > self.max_messages {
            self.messages.remove(0);
        }
    }

    /// Clear all messages
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    /// Draw the panel
    pub fn draw(&mut self, ui: &mut UiContext) {
        ui.panel("Console", self.bounds, |ui| {
            for msg in &self.messages {
                let prefix = match msg.level {
                    LogLevel::Trace => "[TRACE]",
                    LogLevel::Debug => "[DEBUG]",
                    LogLevel::Info => "[INFO]",
                    LogLevel::Warn => "[WARN]",
                    LogLevel::Error => "[ERROR]",
                };
                ui.label(&format!("{} {}", prefix, msg.content));
            }
        });
    }
}

/// Asset browser panel
pub struct AssetBrowserPanel {
    /// Panel bounds
    pub bounds: Rect,
    /// Current directory
    pub current_path: String,
    /// Selected asset
    pub selected: Option<String>,
}

impl Default for AssetBrowserPanel {
    fn default() -> Self {
        Self {
            bounds: Rect::new(0.0, 520.0, 250.0, 160.0),
            current_path: String::from("assets"),
            selected: None,
        }
    }
}

impl AssetBrowserPanel {
    /// Create a new asset browser panel
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Draw the panel
    pub fn draw(&mut self, ui: &mut UiContext) {
        ui.panel("Assets", self.bounds, |ui| {
            ui.label(&self.current_path);
            ui.separator();
            ui.label("📁 textures");
            ui.label("📁 models");
            ui.label("📁 audio");
            ui.label("📁 scripts");
        });
    }
}

/// Viewport panel showing the game view
pub struct ViewportPanel {
    /// Panel bounds
    pub bounds: Rect,
    /// Camera mode
    pub camera_mode: ViewportCameraMode,
    /// Grid visible
    pub show_grid: bool,
    /// Gizmos visible
    pub show_gizmos: bool,
}

/// Viewport camera mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewportCameraMode {
    /// Free camera
    #[default]
    Free,
    /// Orbit around selection
    Orbit,
    /// First person
    FirstPerson,
    /// Top-down 2D view
    TopDown,
}

impl Default for ViewportPanel {
    fn default() -> Self {
        Self {
            bounds: Rect::new(260.0, 80.0, 1080.0, 430.0),
            camera_mode: ViewportCameraMode::Free,
            show_grid: true,
            show_gizmos: true,
        }
    }
}

impl ViewportPanel {
    /// Create a new viewport panel
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
