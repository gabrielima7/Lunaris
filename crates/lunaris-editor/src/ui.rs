//! Immediate Mode UI System
//!
//! A simple, efficient UI system inspired by Dear ImGUI and egui.

use lunaris_core::math::{Color, Rect, Vec2};
use std::collections::HashMap;

/// UI Context for immediate mode rendering
pub struct UiContext {
    /// Current cursor position
    cursor: Vec2,
    /// Current layout direction
    layout: LayoutDirection,
    /// Layout stack for nested layouts
    layout_stack: Vec<LayoutState>,
    /// Widget states (for stateful widgets)
    widget_states: HashMap<u64, WidgetState>,
    /// Current frame's draw commands
    draw_commands: Vec<DrawCommand>,
    /// Input state
    input: UiInput,
    /// Style settings
    style: UiStyle,
    /// Screen size
    screen_size: Vec2,
    /// Current unique ID for widgets
    id_counter: u64,
}

/// Layout direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayoutDirection {
    /// Horizontal layout (left to right)
    #[default]
    Horizontal,
    /// Vertical layout (top to bottom)
    Vertical,
}

/// Layout state for nested layouts
#[derive(Debug, Clone)]
struct LayoutState {
    cursor: Vec2,
    direction: LayoutDirection,
    bounds: Rect,
    spacing: f32,
}

/// Widget state (for buttons, checkboxes, etc.)
#[derive(Debug, Clone, Default)]
struct WidgetState {
    hot: bool,
    active: bool,
    value: WidgetValue,
}

/// Widget value storage
#[derive(Debug, Clone, Default)]
enum WidgetValue {
    #[default]
    None,
    Bool(bool),
    Float(f32),
    String(String),
}

/// UI input state
#[derive(Debug, Clone, Default)]
pub struct UiInput {
    /// Mouse position
    pub mouse_pos: Vec2,
    /// Mouse button down
    pub mouse_down: bool,
    /// Mouse just clicked
    pub mouse_clicked: bool,
    /// Mouse just released
    pub mouse_released: bool,
    /// Scroll delta
    pub scroll: f32,
    /// Text input this frame
    pub text_input: String,
}

/// UI styling
#[derive(Debug, Clone)]
pub struct UiStyle {
    /// Background color
    pub background: Color,
    /// Text color
    pub text: Color,
    /// Primary color (buttons, highlights)
    pub primary: Color,
    /// Secondary color
    pub secondary: Color,
    /// Border color
    pub border: Color,
    /// Hover color
    pub hover: Color,
    /// Active/pressed color
    pub active: Color,
    /// Font size
    pub font_size: f32,
    /// Padding
    pub padding: f32,
    /// Spacing between widgets
    pub spacing: f32,
    /// Border radius
    pub border_radius: f32,
    /// Border width
    pub border_width: f32,
}

impl Default for UiStyle {
    fn default() -> Self {
        Self {
            background: Color::new(0.15, 0.15, 0.18, 1.0),
            text: Color::new(0.95, 0.95, 0.95, 1.0),
            primary: Color::new(0.26, 0.59, 0.98, 1.0),
            secondary: Color::new(0.24, 0.24, 0.27, 1.0),
            border: Color::new(0.3, 0.3, 0.32, 1.0),
            hover: Color::new(0.32, 0.32, 0.35, 1.0),
            active: Color::new(0.2, 0.2, 0.22, 1.0),
            font_size: 14.0,
            padding: 8.0,
            spacing: 4.0,
            border_radius: 4.0,
            border_width: 1.0,
        }
    }
}

impl UiStyle {
    /// Dark theme
    #[must_use]
    pub fn dark() -> Self {
        Self::default()
    }

    /// Light theme
    #[must_use]
    pub fn light() -> Self {
        Self {
            background: Color::new(0.95, 0.95, 0.95, 1.0),
            text: Color::new(0.1, 0.1, 0.1, 1.0),
            primary: Color::new(0.26, 0.59, 0.98, 1.0),
            secondary: Color::new(0.85, 0.85, 0.87, 1.0),
            border: Color::new(0.7, 0.7, 0.72, 1.0),
            hover: Color::new(0.8, 0.8, 0.82, 1.0),
            active: Color::new(0.75, 0.75, 0.77, 1.0),
            ..Default::default()
        }
    }
}

/// Draw command for UI rendering
#[derive(Debug, Clone)]
pub enum DrawCommand {
    /// Draw a rectangle
    Rect {
        bounds: Rect,
        color: Color,
        border_radius: f32,
    },
    /// Draw a bordered rectangle
    BorderedRect {
        bounds: Rect,
        fill: Color,
        border: Color,
        border_width: f32,
        border_radius: f32,
    },
    /// Draw text
    Text {
        position: Vec2,
        text: String,
        color: Color,
        size: f32,
    },
    /// Draw an image/texture
    Image {
        bounds: Rect,
        texture_id: u64,
        tint: Color,
    },
    /// Draw a line
    Line {
        start: Vec2,
        end: Vec2,
        color: Color,
        width: f32,
    },
    /// Set clip rectangle
    SetClip(Option<Rect>),
}

impl Default for UiContext {
    fn default() -> Self {
        Self::new()
    }
}

impl UiContext {
    /// Create a new UI context
    #[must_use]
    pub fn new() -> Self {
        Self {
            cursor: Vec2::ZERO,
            layout: LayoutDirection::Vertical,
            layout_stack: Vec::new(),
            widget_states: HashMap::new(),
            draw_commands: Vec::new(),
            input: UiInput::default(),
            style: UiStyle::default(),
            screen_size: Vec2::new(1280.0, 720.0),
            id_counter: 0,
        }
    }

    /// Begin a new frame
    pub fn begin_frame(&mut self, screen_size: Vec2, input: UiInput) {
        self.screen_size = screen_size;
        self.input = input;
        self.draw_commands.clear();
        self.cursor = Vec2::ZERO;
        self.layout_stack.clear();
        self.id_counter = 0;
    }

    /// End the frame and get draw commands
    #[must_use]
    pub fn end_frame(&mut self) -> Vec<DrawCommand> {
        std::mem::take(&mut self.draw_commands)
    }

    /// Set the style
    pub fn set_style(&mut self, style: UiStyle) {
        self.style = style;
    }

    /// Get a unique ID for a widget
    fn next_id(&mut self) -> u64 {
        self.id_counter += 1;
        self.id_counter
    }

    /// Advance cursor after widget
    fn advance_cursor(&mut self, size: Vec2) {
        match self.layout {
            LayoutDirection::Horizontal => {
                self.cursor.x += size.x + self.style.spacing;
            }
            LayoutDirection::Vertical => {
                self.cursor.y += size.y + self.style.spacing;
            }
        }
    }

    /// Check if mouse is over a rect
    fn is_hovered(&self, bounds: Rect) -> bool {
        bounds.contains(self.input.mouse_pos)
    }

    // ===== LAYOUT WIDGETS =====

    /// Begin a horizontal layout
    pub fn horizontal(&mut self, f: impl FnOnce(&mut Self)) {
        let prev_layout = self.layout;
        let prev_cursor = self.cursor;
        
        self.layout = LayoutDirection::Horizontal;
        f(self);
        
        self.layout = prev_layout;
        self.cursor.x = prev_cursor.x;
        self.cursor.y += self.style.font_size + self.style.padding * 2.0 + self.style.spacing;
    }

    /// Begin a vertical layout
    pub fn vertical(&mut self, f: impl FnOnce(&mut Self)) {
        let prev_layout = self.layout;
        
        self.layout = LayoutDirection::Vertical;
        f(self);
        
        self.layout = prev_layout;
    }

    /// Add spacing
    pub fn space(&mut self, amount: f32) {
        match self.layout {
            LayoutDirection::Horizontal => self.cursor.x += amount,
            LayoutDirection::Vertical => self.cursor.y += amount,
        }
    }

    // ===== CONTAINER WIDGETS =====

    /// Draw a panel/window
    pub fn panel(&mut self, title: &str, bounds: Rect, f: impl FnOnce(&mut Self)) {
        // Background
        self.draw_commands.push(DrawCommand::BorderedRect {
            bounds,
            fill: self.style.background,
            border: self.style.border,
            border_width: self.style.border_width,
            border_radius: self.style.border_radius,
        });

        // Title bar
        let title_height = self.style.font_size + self.style.padding * 2.0;
        let title_bar = Rect::new(bounds.x, bounds.y, bounds.width, title_height);
        
        self.draw_commands.push(DrawCommand::Rect {
            bounds: title_bar,
            color: self.style.secondary,
            border_radius: self.style.border_radius,
        });

        self.draw_commands.push(DrawCommand::Text {
            position: Vec2::new(bounds.x + self.style.padding, bounds.y + self.style.padding),
            text: title.to_string(),
            color: self.style.text,
            size: self.style.font_size,
        });

        // Content area
        let content_start = Vec2::new(
            bounds.x + self.style.padding,
            bounds.y + title_height + self.style.padding,
        );

        let prev_cursor = self.cursor;
        self.cursor = content_start;

        // Set clip rect
        let content_bounds = Rect::new(
            bounds.x,
            bounds.y + title_height,
            bounds.width,
            bounds.height - title_height,
        );
        self.draw_commands.push(DrawCommand::SetClip(Some(content_bounds)));

        f(self);

        // Reset clip
        self.draw_commands.push(DrawCommand::SetClip(None));
        self.cursor = prev_cursor;
    }

    // ===== BASIC WIDGETS =====

    /// Draw a label
    pub fn label(&mut self, text: &str) {
        self.draw_commands.push(DrawCommand::Text {
            position: self.cursor,
            text: text.to_string(),
            color: self.style.text,
            size: self.style.font_size,
        });

        let size = Vec2::new(text.len() as f32 * self.style.font_size * 0.6, self.style.font_size);
        self.advance_cursor(size);
    }

    /// Draw a button, returns true if clicked
    pub fn button(&mut self, text: &str) -> bool {
        let id = self.next_id();
        let text_width = text.len() as f32 * self.style.font_size * 0.6;
        let size = Vec2::new(
            text_width + self.style.padding * 2.0,
            self.style.font_size + self.style.padding * 2.0,
        );
        let bounds = Rect::new(self.cursor.x, self.cursor.y, size.x, size.y);

        let hovered = self.is_hovered(bounds);
        let clicked = hovered && self.input.mouse_clicked;

        // Choose color based on state
        let bg_color = if clicked || (hovered && self.input.mouse_down) {
            self.style.active
        } else if hovered {
            self.style.hover
        } else {
            self.style.secondary
        };

        self.draw_commands.push(DrawCommand::BorderedRect {
            bounds,
            fill: bg_color,
            border: self.style.border,
            border_width: self.style.border_width,
            border_radius: self.style.border_radius,
        });

        self.draw_commands.push(DrawCommand::Text {
            position: Vec2::new(
                bounds.x + self.style.padding,
                bounds.y + self.style.padding,
            ),
            text: text.to_string(),
            color: self.style.text,
            size: self.style.font_size,
        });

        self.advance_cursor(size);
        clicked
    }

    /// Draw a checkbox
    pub fn checkbox(&mut self, label: &str, checked: &mut bool) -> bool {
        let id = self.next_id();
        let box_size = self.style.font_size + 4.0;
        let bounds = Rect::new(self.cursor.x, self.cursor.y, box_size, box_size);

        let hovered = self.is_hovered(bounds);
        if hovered && self.input.mouse_clicked {
            *checked = !*checked;
        }

        // Draw checkbox box
        let bg_color = if *checked { self.style.primary } else { self.style.secondary };
        self.draw_commands.push(DrawCommand::BorderedRect {
            bounds,
            fill: bg_color,
            border: self.style.border,
            border_width: self.style.border_width,
            border_radius: 2.0,
        });

        // Draw checkmark if checked
        if *checked {
            self.draw_commands.push(DrawCommand::Text {
                position: Vec2::new(bounds.x + 2.0, bounds.y),
                text: "✓".to_string(),
                color: self.style.text,
                size: self.style.font_size,
            });
        }

        // Draw label
        self.draw_commands.push(DrawCommand::Text {
            position: Vec2::new(
                bounds.x + box_size + self.style.spacing,
                bounds.y + 2.0,
            ),
            text: label.to_string(),
            color: self.style.text,
            size: self.style.font_size,
        });

        let total_width = box_size + self.style.spacing + label.len() as f32 * self.style.font_size * 0.6;
        self.advance_cursor(Vec2::new(total_width, box_size));
        
        hovered && self.input.mouse_clicked
    }

    /// Draw a slider
    pub fn slider(&mut self, label: &str, value: &mut f32, min: f32, max: f32) -> bool {
        let id = self.next_id();
        let slider_width = 150.0;
        let slider_height = self.style.font_size;
        
        // Draw label
        self.draw_commands.push(DrawCommand::Text {
            position: self.cursor,
            text: label.to_string(),
            color: self.style.text,
            size: self.style.font_size,
        });

        let label_width = label.len() as f32 * self.style.font_size * 0.6 + self.style.spacing;
        let slider_x = self.cursor.x + label_width;
        let bounds = Rect::new(slider_x, self.cursor.y, slider_width, slider_height);

        // Draw track
        self.draw_commands.push(DrawCommand::Rect {
            bounds,
            color: self.style.secondary,
            border_radius: slider_height / 2.0,
        });

        // Calculate handle position
        let t = (*value - min) / (max - min);
        let handle_x = bounds.x + t * (bounds.width - slider_height);
        let handle_bounds = Rect::new(handle_x, bounds.y, slider_height, slider_height);

        // Draw handle
        let hovered = self.is_hovered(bounds);
        let handle_color = if hovered && self.input.mouse_down {
            self.style.primary
        } else if hovered {
            self.style.hover
        } else {
            self.style.primary
        };

        self.draw_commands.push(DrawCommand::Rect {
            bounds: handle_bounds,
            color: handle_color,
            border_radius: slider_height / 2.0,
        });

        // Handle interaction
        let changed = if hovered && self.input.mouse_down {
            let local_x = (self.input.mouse_pos.x - bounds.x) / bounds.width;
            *value = min + local_x.clamp(0.0, 1.0) * (max - min);
            true
        } else {
            false
        };

        // Draw value
        let value_text = format!("{:.2}", value);
        self.draw_commands.push(DrawCommand::Text {
            position: Vec2::new(
                slider_x + slider_width + self.style.spacing,
                self.cursor.y,
            ),
            text: value_text,
            color: self.style.text,
            size: self.style.font_size,
        });

        let total_width = label_width + slider_width + 50.0;
        self.advance_cursor(Vec2::new(total_width, slider_height));
        
        changed
    }

    /// Draw a progress bar
    pub fn progress_bar(&mut self, progress: f32, width: f32) {
        let height = self.style.font_size;
        let bounds = Rect::new(self.cursor.x, self.cursor.y, width, height);

        // Background
        self.draw_commands.push(DrawCommand::Rect {
            bounds,
            color: self.style.secondary,
            border_radius: height / 2.0,
        });

        // Fill
        let fill_width = width * progress.clamp(0.0, 1.0);
        if fill_width > 0.0 {
            self.draw_commands.push(DrawCommand::Rect {
                bounds: Rect::new(bounds.x, bounds.y, fill_width, height),
                color: self.style.primary,
                border_radius: height / 2.0,
            });
        }

        self.advance_cursor(Vec2::new(width, height));
    }

    /// Separator line
    pub fn separator(&mut self) {
        let width = self.screen_size.x - self.cursor.x - self.style.padding;
        self.draw_commands.push(DrawCommand::Line {
            start: self.cursor,
            end: Vec2::new(self.cursor.x + width, self.cursor.y),
            color: self.style.border,
            width: 1.0,
        });
        self.advance_cursor(Vec2::new(width, self.style.spacing));
    }
}
