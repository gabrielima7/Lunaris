//! # Lunaris Editor
//!
//! Visual editor for the Lunaris Game Engine.
//!
//! Features visual scene editing, property inspection, and asset management.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod design_system;
pub mod dock;
pub mod gizmo;
pub mod panels;
pub mod professional_ui;
pub mod properties;
pub mod shortcuts;
pub mod terrain_erosion;
pub mod timeline;
pub mod ui;
pub mod ui_retained;
pub mod widgets;
pub mod window_manager;
pub mod world_builder;

pub use gizmo::{Gizmo, GizmoAxis, GizmoType};
pub use ui::{DrawCommand, UiContext, UiInput, UiStyle};

use lunaris_core::{input::Input, Result};

/// Editor configuration
#[derive(Debug, Clone)]
pub struct EditorConfig {
    /// Window title
    pub title: String,
    /// Initial width
    pub width: u32,
    /// Initial height
    pub height: u32,
    /// UI theme
    pub theme: Theme,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            title: String::from("Lunaris Editor"),
            width: 1600,
            height: 900,
            theme: Theme::Dark,
        }
    }
}

/// UI Theme
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    /// Dark theme
    #[default]
    Dark,
    /// Light theme
    Light,
}

/// Editor state
pub struct Editor {
    /// UI context
    pub ui: UiContext,
    /// Current scene path
    pub current_scene: Option<String>,
    /// Selected entity
    pub selected_entity: Option<u64>,
    /// Is playing
    pub is_playing: bool,
    /// Show hierarchy panel
    pub show_hierarchy: bool,
    /// Show inspector panel
    pub show_inspector: bool,
    /// Show console
    pub show_console: bool,
    /// Show asset browser
    pub show_assets: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {
    /// Create a new editor instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            ui: UiContext::new(),
            current_scene: None,
            selected_entity: None,
            is_playing: false,
            show_hierarchy: true,
            show_inspector: true,
            show_console: true,
            show_assets: true,
        }
    }

    /// Apply theme
    pub fn set_theme(&mut self, theme: Theme) {
        let style = match theme {
            Theme::Dark => UiStyle::dark(),
            Theme::Light => UiStyle::light(),
        };
        self.ui.set_style(style);
    }

    /// Update and render editor UI
    pub fn update(&mut self, input: &Input, screen_size: (u32, u32)) -> Vec<DrawCommand> {
        let ui_input = UiInput {
            mouse_pos: lunaris_core::math::Vec2::new(
                input.mouse_position().0,
                input.mouse_position().1,
            ),
            mouse_down: input.is_mouse_down(lunaris_core::input::MouseButton::Left),
            mouse_clicked: input.is_mouse_pressed(lunaris_core::input::MouseButton::Left),
            mouse_released: input.is_mouse_released(lunaris_core::input::MouseButton::Left),
            scroll: input.scroll_delta(),
            text_input: String::new(),
        };

        let screen = lunaris_core::math::Vec2::new(screen_size.0 as f32, screen_size.1 as f32);
        self.ui.begin_frame(screen, ui_input);

        // Draw editor UI
        self.draw_menu_bar();
        self.draw_toolbar();
        
        if self.show_hierarchy {
            self.draw_hierarchy_panel();
        }
        
        if self.show_inspector {
            self.draw_inspector_panel();
        }
        
        if self.show_console {
            self.draw_console_panel();
        }

        self.ui.end_frame()
    }

    fn draw_menu_bar(&mut self) {
        self.ui.horizontal(|ui| {
            if ui.button("File") {
                // File menu
            }
            if ui.button("Edit") {
                // Edit menu
            }
            if ui.button("View") {
                // View menu
            }
            if ui.button("Window") {
                // Window menu
            }
            if ui.button("Help") {
                // Help menu
            }
        });
    }

    fn draw_toolbar(&mut self) {
        self.ui.horizontal(|ui| {
            if ui.button("▶ Play") {
                self.is_playing = true;
            }
            if ui.button("⏸ Pause") {
                self.is_playing = false;
            }
            if ui.button("⏹ Stop") {
                self.is_playing = false;
            }
            ui.space(20.0);
            if ui.button("💾 Save") {
                tracing::info!("Saving scene...");
            }
        });
        self.ui.separator();
    }

    fn draw_hierarchy_panel(&mut self) {
        let bounds = lunaris_core::math::Rect::new(0.0, 80.0, 250.0, 400.0);
        self.ui.panel("Hierarchy", bounds, |ui| {
            ui.label("Scene: Main");
            ui.separator();
            
            // Sample hierarchy
            if ui.button("📦 Player") {
                self.selected_entity = Some(1);
            }
            if ui.button("  📦 Camera") {
                self.selected_entity = Some(2);
            }
            if ui.button("📦 Enemies") {
                self.selected_entity = Some(3);
            }
            if ui.button("📦 Environment") {
                self.selected_entity = Some(4);
            }
            if ui.button("💡 DirectionalLight") {
                self.selected_entity = Some(5);
            }
        });
    }

    fn draw_inspector_panel(&mut self) {
        let bounds = lunaris_core::math::Rect::new(1350.0, 80.0, 250.0, 600.0);
        self.ui.panel("Inspector", bounds, |ui| {
            if let Some(entity_id) = self.selected_entity {
                ui.label(&format!("Entity: {}", entity_id));
                ui.separator();
                
                ui.label("Transform");
                let mut x = 0.0_f32;
                let mut y = 0.0_f32;
                let mut z = 0.0_f32;
                ui.slider("X", &mut x, -100.0, 100.0);
                ui.slider("Y", &mut y, -100.0, 100.0);
                ui.slider("Z", &mut z, -100.0, 100.0);
                
                ui.separator();
                ui.label("Components");
                
                let mut visible = true;
                ui.checkbox("Visible", &mut visible);
                
                ui.separator();
                if ui.button("Add Component") {
                    tracing::info!("Add component clicked");
                }
            } else {
                ui.label("No entity selected");
            }
        });
    }

    fn draw_console_panel(&mut self) {
        let bounds = lunaris_core::math::Rect::new(260.0, 500.0, 1080.0, 180.0);
        self.ui.panel("Console", bounds, |ui| {
            ui.label("[INFO] Editor initialized");
            ui.label("[INFO] Scene loaded: Main");
            ui.label("[DEBUG] Frame time: 16.6ms");
        });
    }
}

/// Initialize the editor
///
/// # Errors
///
/// Returns an error if initialization fails
pub fn init() -> Result<()> {
    tracing::info!("Editor subsystem initialized");
    Ok(())
}
