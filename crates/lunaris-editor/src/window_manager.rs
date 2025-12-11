//! Multi-Window Manager
//!
//! Support for multiple editor windows, floating panels, and modal dialogs.

use glam::Vec2;
use std::collections::HashMap;

/// Window ID
pub type WindowId = u64;

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}

/// Window type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowType {
    /// Main editor window
    Main,
    /// Floating tool window
    Tool,
    /// Modal dialog
    Modal,
    /// Popup menu
    Popup,
    /// Tooltip
    Tooltip,
}

/// Window configuration
#[derive(Debug, Clone)]
pub struct WindowConfig {
    /// Title
    pub title: String,
    /// Initial position
    pub position: Vec2,
    /// Initial size
    pub size: Vec2,
    /// Minimum size
    pub min_size: Vec2,
    /// Maximum size
    pub max_size: Option<Vec2>,
    /// Is resizable
    pub resizable: bool,
    /// Has decorations (titlebar, border)
    pub decorated: bool,
    /// Is transparent
    pub transparent: bool,
    /// Always on top
    pub always_on_top: bool,
    /// Window type
    pub window_type: WindowType,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            position: Vec2::new(100.0, 100.0),
            size: Vec2::new(800.0, 600.0),
            min_size: Vec2::new(200.0, 150.0),
            max_size: None,
            resizable: true,
            decorated: true,
            transparent: false,
            always_on_top: false,
            window_type: WindowType::Tool,
        }
    }
}

/// Editor window
#[derive(Debug, Clone)]
pub struct EditorWindow {
    /// Window ID
    pub id: WindowId,
    /// Configuration
    pub config: WindowConfig,
    /// Current state
    pub state: WindowState,
    /// Current position
    pub position: Vec2,
    /// Current size
    pub size: Vec2,
    /// Is focused
    pub focused: bool,
    /// Is visible
    pub visible: bool,
    /// Content ID (for dock system)
    pub content_id: Option<String>,
    /// Z-order
    pub z_order: u32,
    /// Saved position (for restore from minimize/maximize)
    saved_bounds: Option<(Vec2, Vec2)>,
}

impl EditorWindow {
    /// Create new window
    pub fn new(id: WindowId, config: WindowConfig) -> Self {
        Self {
            id,
            position: config.position,
            size: config.size,
            config,
            state: WindowState::Normal,
            focused: false,
            visible: true,
            content_id: None,
            z_order: 0,
            saved_bounds: None,
        }
    }

    /// Get bounds
    pub fn bounds(&self) -> super::ui_retained::Rect {
        super::ui_retained::Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }

    /// Minimize
    pub fn minimize(&mut self) {
        if self.state != WindowState::Minimized {
            self.saved_bounds = Some((self.position, self.size));
            self.state = WindowState::Minimized;
            self.visible = false;
        }
    }

    /// Maximize
    pub fn maximize(&mut self, screen_size: Vec2) {
        if self.state != WindowState::Maximized {
            self.saved_bounds = Some((self.position, self.size));
            self.position = Vec2::ZERO;
            self.size = screen_size;
            self.state = WindowState::Maximized;
        }
    }

    /// Restore
    pub fn restore(&mut self) {
        if let Some((pos, size)) = self.saved_bounds.take() {
            self.position = pos;
            self.size = size;
        }
        self.state = WindowState::Normal;
        self.visible = true;
    }

    /// Toggle maximize
    pub fn toggle_maximize(&mut self, screen_size: Vec2) {
        if self.state == WindowState::Maximized {
            self.restore();
        } else {
            self.maximize(screen_size);
        }
    }
}

/// Window manager
pub struct WindowManager {
    /// All windows
    windows: HashMap<WindowId, EditorWindow>,
    /// Window order (front to back)
    order: Vec<WindowId>,
    /// Next window ID
    next_id: WindowId,
    /// Focused window
    focused: Option<WindowId>,
    /// Modal stack
    modal_stack: Vec<WindowId>,
    /// Drag state
    drag: Option<WindowDrag>,
    /// Resize state
    resize: Option<WindowResize>,
    /// Screen size
    pub screen_size: Vec2,
}

/// Window drag state
#[derive(Debug, Clone)]
struct WindowDrag {
    window_id: WindowId,
    start_pos: Vec2,
    start_window_pos: Vec2,
}

/// Window resize state
#[derive(Debug, Clone)]
struct WindowResize {
    window_id: WindowId,
    edge: ResizeEdge,
    start_pos: Vec2,
    start_bounds: (Vec2, Vec2),
}

/// Resize edge
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeEdge {
    Top,
    Right,
    Bottom,
    Left,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowManager {
    /// Create new window manager
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            order: Vec::new(),
            next_id: 1,
            focused: None,
            modal_stack: Vec::new(),
            drag: None,
            resize: None,
            screen_size: Vec2::new(1920.0, 1080.0),
        }
    }

    /// Create window
    pub fn create(&mut self, config: WindowConfig) -> WindowId {
        let id = self.next_id;
        self.next_id += 1;

        let is_modal = config.window_type == WindowType::Modal;
        let window = EditorWindow::new(id, config);

        self.windows.insert(id, window);
        self.order.push(id);

        if is_modal {
            self.modal_stack.push(id);
        }

        self.focus(id);
        id
    }

    /// Close window
    pub fn close(&mut self, id: WindowId) {
        self.windows.remove(&id);
        self.order.retain(|&w| w != id);
        self.modal_stack.retain(|&w| w != id);

        if self.focused == Some(id) {
            self.focused = self.order.last().copied();
        }
    }

    /// Get window
    pub fn get(&self, id: WindowId) -> Option<&EditorWindow> {
        self.windows.get(&id)
    }

    /// Get window mutably
    pub fn get_mut(&mut self, id: WindowId) -> Option<&mut EditorWindow> {
        self.windows.get_mut(&id)
    }

    /// Focus window
    pub fn focus(&mut self, id: WindowId) {
        // Can't focus if modal is open and this isn't the modal
        if let Some(&modal_id) = self.modal_stack.last() {
            if modal_id != id {
                return;
            }
        }

        if let Some(old_id) = self.focused {
            if let Some(window) = self.windows.get_mut(&old_id) {
                window.focused = false;
            }
        }

        if let Some(window) = self.windows.get_mut(&id) {
            window.focused = true;
            self.focused = Some(id);

            // Move to front
            self.order.retain(|&w| w != id);
            self.order.push(id);

            // Update z-order
            for (i, &win_id) in self.order.iter().enumerate() {
                if let Some(win) = self.windows.get_mut(&win_id) {
                    win.z_order = i as u32;
                }
            }
        }
    }

    /// Get focused window
    pub fn focused(&self) -> Option<WindowId> {
        self.focused
    }

    /// Start dragging window
    pub fn start_drag(&mut self, window_id: WindowId, mouse_pos: Vec2) {
        if let Some(window) = self.windows.get(&window_id) {
            if window.state != WindowState::Normal {
                return; // Can't drag maximized/minimized windows
            }

            self.drag = Some(WindowDrag {
                window_id,
                start_pos: mouse_pos,
                start_window_pos: window.position,
            });
        }
    }

    /// Update drag
    pub fn update_drag(&mut self, mouse_pos: Vec2) {
        if let Some(ref drag) = self.drag {
            let delta = mouse_pos - drag.start_pos;
            if let Some(window) = self.windows.get_mut(&drag.window_id) {
                window.position = drag.start_window_pos + delta;

                // Keep on screen
                window.position.x = window.position.x.max(0.0);
                window.position.y = window.position.y.max(0.0);
            }
        }
    }

    /// End drag
    pub fn end_drag(&mut self) {
        self.drag = None;
    }

    /// Start resizing window
    pub fn start_resize(&mut self, window_id: WindowId, edge: ResizeEdge, mouse_pos: Vec2) {
        if let Some(window) = self.windows.get(&window_id) {
            if !window.config.resizable || window.state != WindowState::Normal {
                return;
            }

            self.resize = Some(WindowResize {
                window_id,
                edge,
                start_pos: mouse_pos,
                start_bounds: (window.position, window.size),
            });
        }
    }

    /// Update resize
    pub fn update_resize(&mut self, mouse_pos: Vec2) {
        if let Some(ref resize) = self.resize.clone() {
            let delta = mouse_pos - resize.start_pos;
            let (start_pos, start_size) = resize.start_bounds;

            if let Some(window) = self.windows.get_mut(&resize.window_id) {
                let min = window.config.min_size;
                let max = window.config.max_size.unwrap_or(Vec2::splat(f32::MAX));

                match resize.edge {
                    ResizeEdge::Right => {
                        window.size.x = (start_size.x + delta.x).clamp(min.x, max.x);
                    }
                    ResizeEdge::Bottom => {
                        window.size.y = (start_size.y + delta.y).clamp(min.y, max.y);
                    }
                    ResizeEdge::Left => {
                        let new_width = (start_size.x - delta.x).clamp(min.x, max.x);
                        let width_change = start_size.x - new_width;
                        window.position.x = start_pos.x + width_change;
                        window.size.x = new_width;
                    }
                    ResizeEdge::Top => {
                        let new_height = (start_size.y - delta.y).clamp(min.y, max.y);
                        let height_change = start_size.y - new_height;
                        window.position.y = start_pos.y + height_change;
                        window.size.y = new_height;
                    }
                    ResizeEdge::BottomRight => {
                        window.size.x = (start_size.x + delta.x).clamp(min.x, max.x);
                        window.size.y = (start_size.y + delta.y).clamp(min.y, max.y);
                    }
                    ResizeEdge::BottomLeft => {
                        let new_width = (start_size.x - delta.x).clamp(min.x, max.x);
                        let width_change = start_size.x - new_width;
                        window.position.x = start_pos.x + width_change;
                        window.size.x = new_width;
                        window.size.y = (start_size.y + delta.y).clamp(min.y, max.y);
                    }
                    ResizeEdge::TopRight => {
                        let new_height = (start_size.y - delta.y).clamp(min.y, max.y);
                        let height_change = start_size.y - new_height;
                        window.position.y = start_pos.y + height_change;
                        window.size.y = new_height;
                        window.size.x = (start_size.x + delta.x).clamp(min.x, max.x);
                    }
                    ResizeEdge::TopLeft => {
                        let new_width = (start_size.x - delta.x).clamp(min.x, max.x);
                        let new_height = (start_size.y - delta.y).clamp(min.y, max.y);
                        let width_change = start_size.x - new_width;
                        let height_change = start_size.y - new_height;
                        window.position = start_pos + Vec2::new(width_change, height_change);
                        window.size = Vec2::new(new_width, new_height);
                    }
                }
            }
        }
    }

    /// End resize
    pub fn end_resize(&mut self) {
        self.resize = None;
    }

    /// Get resize edge at position
    pub fn get_resize_edge(&self, window_id: WindowId, pos: Vec2) -> Option<ResizeEdge> {
        const EDGE_SIZE: f32 = 8.0;

        let window = self.windows.get(&window_id)?;
        if !window.config.resizable {
            return None;
        }

        let bounds = window.bounds();
        let in_left = pos.x < bounds.x + EDGE_SIZE;
        let in_right = pos.x > bounds.x + bounds.width - EDGE_SIZE;
        let in_top = pos.y < bounds.y + EDGE_SIZE;
        let in_bottom = pos.y > bounds.y + bounds.height - EDGE_SIZE;

        match (in_top, in_right, in_bottom, in_left) {
            (true, true, false, false) => Some(ResizeEdge::TopRight),
            (true, false, false, true) => Some(ResizeEdge::TopLeft),
            (false, true, true, false) => Some(ResizeEdge::BottomRight),
            (false, false, true, true) => Some(ResizeEdge::BottomLeft),
            (true, false, false, false) => Some(ResizeEdge::Top),
            (false, true, false, false) => Some(ResizeEdge::Right),
            (false, false, true, false) => Some(ResizeEdge::Bottom),
            (false, false, false, true) => Some(ResizeEdge::Left),
            _ => None,
        }
    }

    /// Hit test - find window at position
    pub fn hit_test(&self, pos: Vec2) -> Option<WindowId> {
        // Test in reverse order (front to back)
        for &id in self.order.iter().rev() {
            if let Some(window) = self.windows.get(&id) {
                if window.visible && window.bounds().contains(pos) {
                    return Some(id);
                }
            }
        }
        None
    }

    /// Get all visible windows in z-order
    pub fn visible_windows(&self) -> Vec<&EditorWindow> {
        self.order.iter()
            .filter_map(|&id| self.windows.get(&id))
            .filter(|w| w.visible)
            .collect()
    }

    /// Tile windows
    pub fn tile_horizontal(&mut self) {
        let count = self.windows.len() as f32;
        if count == 0.0 {
            return;
        }

        let width = self.screen_size.x / count;

        for (i, &id) in self.order.iter().enumerate() {
            if let Some(window) = self.windows.get_mut(&id) {
                window.position = Vec2::new(i as f32 * width, 0.0);
                window.size = Vec2::new(width, self.screen_size.y);
                window.state = WindowState::Normal;
            }
        }
    }

    /// Tile windows vertically
    pub fn tile_vertical(&mut self) {
        let count = self.windows.len() as f32;
        if count == 0.0 {
            return;
        }

        let height = self.screen_size.y / count;

        for (i, &id) in self.order.iter().enumerate() {
            if let Some(window) = self.windows.get_mut(&id) {
                window.position = Vec2::new(0.0, i as f32 * height);
                window.size = Vec2::new(self.screen_size.x, height);
                window.state = WindowState::Normal;
            }
        }
    }

    /// Cascade windows
    pub fn cascade(&mut self) {
        const OFFSET: f32 = 30.0;

        for (i, &id) in self.order.iter().enumerate() {
            if let Some(window) = self.windows.get_mut(&id) {
                window.position = Vec2::new(i as f32 * OFFSET, i as f32 * OFFSET);
                window.size = Vec2::new(800.0, 600.0);
                window.state = WindowState::Normal;
            }
        }
    }
}

/// Dialog result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogResult {
    Ok,
    Cancel,
    Yes,
    No,
    Custom(u32),
}

/// Message box type
#[derive(Debug, Clone, Copy)]
pub enum MessageBoxType {
    Info,
    Warning,
    Error,
    Question,
}

/// Show message box
pub fn message_box(
    window_manager: &mut WindowManager,
    title: &str,
    message: &str,
    box_type: MessageBoxType,
) -> WindowId {
    let config = WindowConfig {
        title: title.to_string(),
        position: Vec2::new(
            window_manager.screen_size.x / 2.0 - 200.0,
            window_manager.screen_size.y / 2.0 - 75.0,
        ),
        size: Vec2::new(400.0, 150.0),
        min_size: Vec2::new(300.0, 100.0),
        resizable: false,
        window_type: WindowType::Modal,
        ..Default::default()
    };

    window_manager.create(config)
}

/// Show file dialog
pub fn file_dialog(
    window_manager: &mut WindowManager,
    title: &str,
    save: bool,
) -> WindowId {
    let config = WindowConfig {
        title: title.to_string(),
        position: Vec2::new(
            window_manager.screen_size.x / 2.0 - 300.0,
            window_manager.screen_size.y / 2.0 - 200.0,
        ),
        size: Vec2::new(600.0, 400.0),
        window_type: WindowType::Modal,
        ..Default::default()
    };

    window_manager.create(config)
}
