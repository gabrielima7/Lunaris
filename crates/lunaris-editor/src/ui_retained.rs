//! Retained UI System
//!
//! A retained-mode UI framework for complex editor interfaces.
//! Supports multi-window, virtual DOM diffing, and layout caching.

use glam::Vec2;
use std::collections::HashMap;
use std::sync::Arc;

// ==================== UI TREE ====================

/// Unique widget ID
pub type WidgetId = u64;

/// Widget in the retained tree
#[derive(Debug, Clone)]
pub struct Widget {
    /// Unique ID
    pub id: WidgetId,
    /// Widget type
    pub widget_type: WidgetType,
    /// Children
    pub children: Vec<WidgetId>,
    /// Parent
    pub parent: Option<WidgetId>,
    /// Layout
    pub layout: Layout,
    /// Computed bounds
    pub bounds: Rect,
    /// Style
    pub style: WidgetStyle,
    /// State
    pub state: WidgetState,
    /// Is visible
    pub visible: bool,
    /// Is enabled
    pub enabled: bool,
    /// Z-index
    pub z_index: i32,
}

/// Widget type
#[derive(Debug, Clone)]
pub enum WidgetType {
    /// Root window
    Window(WindowProps),
    /// Panel/Container
    Panel(PanelProps),
    /// Button
    Button(ButtonProps),
    /// Label
    Label(LabelProps),
    /// Text input
    TextInput(TextInputProps),
    /// Slider
    Slider(SliderProps),
    /// Checkbox
    Checkbox(CheckboxProps),
    /// Dropdown
    Dropdown(DropdownProps),
    /// Scroll area
    ScrollArea(ScrollAreaProps),
    /// Tree view
    TreeView(TreeViewProps),
    /// Tab bar
    TabBar(TabBarProps),
    /// Splitter
    Splitter(SplitterProps),
    /// Canvas (custom rendering)
    Canvas(CanvasProps),
    /// Spacer
    Spacer,
    /// Custom widget
    Custom(String),
}

/// Rectangle
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

// ==================== LAYOUT ====================

/// Layout properties
#[derive(Debug, Clone)]
pub struct Layout {
    /// Layout direction
    pub direction: LayoutDirection,
    /// Alignment
    pub align: Alignment,
    /// Cross-axis alignment
    pub cross_align: Alignment,
    /// Padding
    pub padding: Edges,
    /// Margin
    pub margin: Edges,
    /// Gap between children
    pub gap: f32,
    /// Size constraints
    pub size: SizeConstraint,
    /// Flex grow
    pub flex_grow: f32,
    /// Flex shrink
    pub flex_shrink: f32,
    /// Position mode
    pub position: PositionMode,
    /// Absolute position (if position is Absolute)
    pub absolute_pos: Option<Vec2>,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            direction: LayoutDirection::Vertical,
            align: Alignment::Start,
            cross_align: Alignment::Stretch,
            padding: Edges::all(0.0),
            margin: Edges::all(0.0),
            gap: 4.0,
            size: SizeConstraint::default(),
            flex_grow: 0.0,
            flex_shrink: 1.0,
            position: PositionMode::Relative,
            absolute_pos: None,
        }
    }
}

/// Layout direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayoutDirection {
    Horizontal,
    #[default]
    Vertical,
}

/// Alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    #[default]
    Start,
    Center,
    End,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

/// Position mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PositionMode {
    #[default]
    Relative,
    Absolute,
    Fixed,
}

/// Edge values (padding, margin, border)
#[derive(Debug, Clone, Copy, Default)]
pub struct Edges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Edges {
    pub fn all(value: f32) -> Self {
        Self { top: value, right: value, bottom: value, left: value }
    }

    pub fn symmetric(horizontal: f32, vertical: f32) -> Self {
        Self { top: vertical, right: horizontal, bottom: vertical, left: horizontal }
    }

    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

/// Size constraint
#[derive(Debug, Clone, Copy)]
pub struct SizeConstraint {
    pub width: Dimension,
    pub height: Dimension,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub aspect_ratio: Option<f32>,
}

impl Default for SizeConstraint {
    fn default() -> Self {
        Self {
            width: Dimension::Auto,
            height: Dimension::Auto,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            aspect_ratio: None,
        }
    }
}

/// Dimension
#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    /// Automatic size
    Auto,
    /// Fixed pixels
    Px(f32),
    /// Percentage of parent
    Percent(f32),
    /// Fill remaining space
    Fill,
}

// ==================== WIDGET PROPERTIES ====================

/// Window properties
#[derive(Debug, Clone)]
pub struct WindowProps {
    pub title: String,
    pub resizable: bool,
    pub closable: bool,
    pub minimizable: bool,
    pub maximizable: bool,
    pub modal: bool,
    pub has_titlebar: bool,
    pub decorations: bool,
}

impl Default for WindowProps {
    fn default() -> Self {
        Self {
            title: String::new(),
            resizable: true,
            closable: true,
            minimizable: true,
            maximizable: true,
            modal: false,
            has_titlebar: true,
            decorations: true,
        }
    }
}

/// Panel properties
#[derive(Debug, Clone, Default)]
pub struct PanelProps {
    pub title: Option<String>,
    pub collapsible: bool,
    pub collapsed: bool,
    pub bordered: bool,
}

/// Button properties
#[derive(Debug, Clone)]
pub struct ButtonProps {
    pub label: String,
    pub icon: Option<String>,
    pub tooltip: Option<String>,
    pub variant: ButtonVariant,
}

/// Button variant
#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Ghost,
    Link,
}

/// Label properties
#[derive(Debug, Clone)]
pub struct LabelProps {
    pub text: String,
    pub selectable: bool,
    pub wrap: bool,
}

/// Text input properties
#[derive(Debug, Clone)]
pub struct TextInputProps {
    pub value: String,
    pub placeholder: String,
    pub password: bool,
    pub multiline: bool,
    pub readonly: bool,
}

/// Slider properties
#[derive(Debug, Clone)]
pub struct SliderProps {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub show_value: bool,
}

/// Checkbox properties
#[derive(Debug, Clone)]
pub struct CheckboxProps {
    pub checked: bool,
    pub label: String,
    pub indeterminate: bool,
}

/// Dropdown properties
#[derive(Debug, Clone)]
pub struct DropdownProps {
    pub selected: usize,
    pub options: Vec<String>,
    pub searchable: bool,
}

/// Scroll area properties
#[derive(Debug, Clone, Default)]
pub struct ScrollAreaProps {
    pub scroll_x: bool,
    pub scroll_y: bool,
    pub offset: Vec2,
    pub content_size: Vec2,
}

/// Tree view properties
#[derive(Debug, Clone, Default)]
pub struct TreeViewProps {
    pub expanded: Vec<usize>,
    pub selected: Option<usize>,
}

/// Tab bar properties
#[derive(Debug, Clone)]
pub struct TabBarProps {
    pub tabs: Vec<String>,
    pub active: usize,
    pub closable: bool,
}

/// Splitter properties
#[derive(Debug, Clone)]
pub struct SplitterProps {
    pub ratio: f32,
    pub min_first: f32,
    pub min_second: f32,
    pub direction: LayoutDirection,
}

/// Canvas properties
#[derive(Debug, Clone, Default)]
pub struct CanvasProps {
    pub render_callback: String,
}

// ==================== WIDGET STATE ====================

/// Widget interaction state
#[derive(Debug, Clone, Default)]
pub struct WidgetState {
    pub hovered: bool,
    pub focused: bool,
    pub pressed: bool,
    pub dragging: bool,
    pub dirty: bool,
}

/// Widget style
#[derive(Debug, Clone)]
pub struct WidgetStyle {
    pub background: StyleColor,
    pub foreground: StyleColor,
    pub border_color: StyleColor,
    pub border_width: f32,
    pub border_radius: f32,
    pub font_size: f32,
    pub font_weight: FontWeight,
    pub opacity: f32,
    pub shadow: Option<Shadow>,
    pub transition: Option<Transition>,
}

impl Default for WidgetStyle {
    fn default() -> Self {
        Self {
            background: StyleColor::Transparent,
            foreground: StyleColor::Solid([0.9, 0.9, 0.9, 1.0]),
            border_color: StyleColor::Transparent,
            border_width: 0.0,
            border_radius: 0.0,
            font_size: 14.0,
            font_weight: FontWeight::Normal,
            opacity: 1.0,
            shadow: None,
            transition: None,
        }
    }
}

/// Style color
#[derive(Debug, Clone)]
pub enum StyleColor {
    Transparent,
    Solid([f32; 4]),
    Gradient { start: [f32; 4], end: [f32; 4], angle: f32 },
    Variable(String),
}

/// Font weight
#[derive(Debug, Clone, Copy, Default)]
pub enum FontWeight {
    Light,
    #[default]
    Normal,
    Medium,
    Bold,
}

/// Shadow
#[derive(Debug, Clone, Copy)]
pub struct Shadow {
    pub offset: Vec2,
    pub blur: f32,
    pub spread: f32,
    pub color: [f32; 4],
}

/// Transition
#[derive(Debug, Clone)]
pub struct Transition {
    pub property: String,
    pub duration: f32,
    pub easing: Easing,
}

/// Easing function
#[derive(Debug, Clone, Copy, Default)]
pub enum Easing {
    #[default]
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Cubic(f32, f32, f32, f32),
}

// ==================== UI TREE MANAGER ====================

/// Retained UI tree
pub struct UiTree {
    /// All widgets
    widgets: HashMap<WidgetId, Widget>,
    /// Root widgets (windows)
    roots: Vec<WidgetId>,
    /// Next widget ID
    next_id: WidgetId,
    /// Focused widget
    focused: Option<WidgetId>,
    /// Hovered widget
    hovered: Option<WidgetId>,
    /// Pressed widget
    pressed: Option<WidgetId>,
    /// Drag state
    drag_state: Option<DragState>,
    /// Layout cache
    layout_cache: HashMap<WidgetId, Rect>,
    /// Dirty widgets needing layout
    dirty: Vec<WidgetId>,
    /// Theme
    pub theme: Theme,
}

/// Drag state
#[derive(Debug, Clone)]
struct DragState {
    widget_id: WidgetId,
    start_pos: Vec2,
    current_pos: Vec2,
}

impl Default for UiTree {
    fn default() -> Self {
        Self::new()
    }
}

impl UiTree {
    /// Create new UI tree
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            roots: Vec::new(),
            next_id: 1,
            focused: None,
            hovered: None,
            pressed: None,
            drag_state: None,
            layout_cache: HashMap::new(),
            dirty: Vec::new(),
            theme: Theme::dark(),
        }
    }

    /// Create widget and return ID
    pub fn create(&mut self, widget_type: WidgetType) -> WidgetId {
        let id = self.next_id;
        self.next_id += 1;

        let widget = Widget {
            id,
            widget_type,
            children: Vec::new(),
            parent: None,
            layout: Layout::default(),
            bounds: Rect::default(),
            style: WidgetStyle::default(),
            state: WidgetState::default(),
            visible: true,
            enabled: true,
            z_index: 0,
        };

        self.widgets.insert(id, widget);
        self.mark_dirty(id);
        id
    }

    /// Add widget as root (window)
    pub fn add_root(&mut self, id: WidgetId) {
        if !self.roots.contains(&id) {
            self.roots.push(id);
        }
    }

    /// Remove root
    pub fn remove_root(&mut self, id: WidgetId) {
        self.roots.retain(|&r| r != id);
    }

    /// Add child to parent
    pub fn add_child(&mut self, parent_id: WidgetId, child_id: WidgetId) {
        if let Some(child) = self.widgets.get_mut(&child_id) {
            child.parent = Some(parent_id);
        }
        if let Some(parent) = self.widgets.get_mut(&parent_id) {
            if !parent.children.contains(&child_id) {
                parent.children.push(child_id);
            }
        }
        self.mark_dirty(parent_id);
    }

    /// Remove child from parent
    pub fn remove_child(&mut self, parent_id: WidgetId, child_id: WidgetId) {
        if let Some(parent) = self.widgets.get_mut(&parent_id) {
            parent.children.retain(|&c| c != child_id);
        }
        if let Some(child) = self.widgets.get_mut(&child_id) {
            child.parent = None;
        }
        self.mark_dirty(parent_id);
    }

    /// Get widget
    pub fn get(&self, id: WidgetId) -> Option<&Widget> {
        self.widgets.get(&id)
    }

    /// Get widget mutably
    pub fn get_mut(&mut self, id: WidgetId) -> Option<&mut Widget> {
        let widget = self.widgets.get_mut(&id)?;
        widget.state.dirty = true;
        Some(widget)
    }

    /// Mark widget as needing layout
    pub fn mark_dirty(&mut self, id: WidgetId) {
        if !self.dirty.contains(&id) {
            self.dirty.push(id);
        }
    }

    /// Delete widget and children
    pub fn delete(&mut self, id: WidgetId) {
        // Get children first
        let children: Vec<WidgetId> = self.widgets.get(&id)
            .map(|w| w.children.clone())
            .unwrap_or_default();

        // Delete children recursively
        for child_id in children {
            self.delete(child_id);
        }

        // Remove from parent
        if let Some(widget) = self.widgets.get(&id) {
            if let Some(parent_id) = widget.parent {
                if let Some(parent) = self.widgets.get_mut(&parent_id) {
                    parent.children.retain(|&c| c != id);
                }
            }
        }

        // Remove widget
        self.widgets.remove(&id);
        self.roots.retain(|&r| r != id);
        self.layout_cache.remove(&id);
    }

    /// Perform layout calculation
    pub fn layout(&mut self, screen_size: Vec2) {
        // Layout each root window
        for &root_id in &self.roots.clone() {
            self.layout_widget(root_id, Rect::new(0.0, 0.0, screen_size.x, screen_size.y));
        }
        self.dirty.clear();
    }

    fn layout_widget(&mut self, id: WidgetId, available: Rect) {
        let (layout, children, size_constraint) = {
            let widget = match self.widgets.get(&id) {
                Some(w) => w,
                None => return,
            };
            (widget.layout.clone(), widget.children.clone(), widget.layout.size.clone())
        };

        // Calculate this widget's bounds
        let width = match size_constraint.width {
            Dimension::Auto | Dimension::Fill => available.width - layout.margin.horizontal(),
            Dimension::Px(px) => px,
            Dimension::Percent(p) => available.width * p / 100.0,
        };
        let height = match size_constraint.height {
            Dimension::Auto | Dimension::Fill => available.height - layout.margin.vertical(),
            Dimension::Px(px) => px,
            Dimension::Percent(p) => available.height * p / 100.0,
        };

        let bounds = Rect::new(
            available.x + layout.margin.left,
            available.y + layout.margin.top,
            width.max(0.0),
            height.max(0.0),
        );

        // Update widget bounds
        if let Some(widget) = self.widgets.get_mut(&id) {
            widget.bounds = bounds;
        }
        self.layout_cache.insert(id, bounds);

        // Layout children
        let content_x = bounds.x + layout.padding.left;
        let content_y = bounds.y + layout.padding.top;
        let content_width = bounds.width - layout.padding.horizontal();
        let content_height = bounds.height - layout.padding.vertical();

        let mut cursor_x = content_x;
        let mut cursor_y = content_y;

        for child_id in children {
            let child_available = match layout.direction {
                LayoutDirection::Horizontal => Rect::new(
                    cursor_x,
                    content_y,
                    content_width - (cursor_x - content_x),
                    content_height,
                ),
                LayoutDirection::Vertical => Rect::new(
                    content_x,
                    cursor_y,
                    content_width,
                    content_height - (cursor_y - content_y),
                ),
            };

            self.layout_widget(child_id, child_available);

            // Advance cursor
            if let Some(child) = self.widgets.get(&child_id) {
                match layout.direction {
                    LayoutDirection::Horizontal => {
                        cursor_x = child.bounds.x + child.bounds.width + layout.gap;
                    }
                    LayoutDirection::Vertical => {
                        cursor_y = child.bounds.y + child.bounds.height + layout.gap;
                    }
                }
            }
        }
    }

    /// Handle input event
    pub fn handle_event(&mut self, event: &UiEvent) -> Option<UiAction> {
        match event {
            UiEvent::MouseMove(pos) => self.handle_mouse_move(*pos),
            UiEvent::MouseDown(pos, button) => self.handle_mouse_down(*pos, *button),
            UiEvent::MouseUp(pos, button) => self.handle_mouse_up(*pos, *button),
            UiEvent::Scroll(delta) => self.handle_scroll(*delta),
            UiEvent::KeyDown(key) => self.handle_key_down(*key),
            UiEvent::KeyUp(key) => self.handle_key_up(*key),
            UiEvent::Text(text) => self.handle_text(text),
        }
    }

    fn handle_mouse_move(&mut self, pos: Vec2) -> Option<UiAction> {
        // Update hover state
        let old_hovered = self.hovered;
        self.hovered = self.hit_test(pos);

        // Update drag
        if let Some(ref mut drag) = self.drag_state {
            drag.current_pos = pos;
        }

        // Clear old hover
        if let Some(old_id) = old_hovered {
            if Some(old_id) != self.hovered {
                if let Some(widget) = self.widgets.get_mut(&old_id) {
                    widget.state.hovered = false;
                }
            }
        }

        // Set new hover
        if let Some(new_id) = self.hovered {
            if let Some(widget) = self.widgets.get_mut(&new_id) {
                widget.state.hovered = true;
            }
        }

        None
    }

    fn handle_mouse_down(&mut self, pos: Vec2, _button: u8) -> Option<UiAction> {
        let hit = self.hit_test(pos);
        self.pressed = hit;
        self.focused = hit;

        if let Some(id) = hit {
            if let Some(widget) = self.widgets.get_mut(&id) {
                widget.state.pressed = true;
                widget.state.focused = true;

                // Start drag
                self.drag_state = Some(DragState {
                    widget_id: id,
                    start_pos: pos,
                    current_pos: pos,
                });

                // Generate click action
                return Some(UiAction::Click(id));
            }
        }

        None
    }

    fn handle_mouse_up(&mut self, _pos: Vec2, _button: u8) -> Option<UiAction> {
        if let Some(id) = self.pressed.take() {
            if let Some(widget) = self.widgets.get_mut(&id) {
                widget.state.pressed = false;
            }
        }

        self.drag_state = None;
        None
    }

    fn handle_scroll(&mut self, delta: Vec2) -> Option<UiAction> {
        if let Some(id) = self.hovered {
            return Some(UiAction::Scroll(id, delta));
        }
        None
    }

    fn handle_key_down(&mut self, key: u32) -> Option<UiAction> {
        if let Some(id) = self.focused {
            return Some(UiAction::KeyDown(id, key));
        }
        None
    }

    fn handle_key_up(&mut self, key: u32) -> Option<UiAction> {
        if let Some(id) = self.focused {
            return Some(UiAction::KeyUp(id, key));
        }
        None
    }

    fn handle_text(&mut self, text: &str) -> Option<UiAction> {
        if let Some(id) = self.focused {
            return Some(UiAction::TextInput(id, text.to_string()));
        }
        None
    }

    /// Hit test - find widget at position
    fn hit_test(&self, pos: Vec2) -> Option<WidgetId> {
        // Test roots in reverse order (top-most first)
        for &root_id in self.roots.iter().rev() {
            if let Some(hit) = self.hit_test_widget(root_id, pos) {
                return Some(hit);
            }
        }
        None
    }

    fn hit_test_widget(&self, id: WidgetId, pos: Vec2) -> Option<WidgetId> {
        let widget = self.widgets.get(&id)?;

        if !widget.visible || !widget.enabled {
            return None;
        }

        if !widget.bounds.contains(pos) {
            return None;
        }

        // Test children in reverse order
        for &child_id in widget.children.iter().rev() {
            if let Some(hit) = self.hit_test_widget(child_id, pos) {
                return Some(hit);
            }
        }

        Some(id)
    }

    /// Get all roots
    pub fn roots(&self) -> &[WidgetId] {
        &self.roots
    }
}

/// UI Event
#[derive(Debug, Clone)]
pub enum UiEvent {
    MouseMove(Vec2),
    MouseDown(Vec2, u8),
    MouseUp(Vec2, u8),
    Scroll(Vec2),
    KeyDown(u32),
    KeyUp(u32),
    Text(String),
}

/// UI Action (output from event handling)
#[derive(Debug, Clone)]
pub enum UiAction {
    Click(WidgetId),
    DoubleClick(WidgetId),
    Scroll(WidgetId, Vec2),
    KeyDown(WidgetId, u32),
    KeyUp(WidgetId, u32),
    TextInput(WidgetId, String),
    DragStart(WidgetId),
    DragEnd(WidgetId),
    Focus(WidgetId),
    Blur(WidgetId),
}

// ==================== THEME ====================

/// UI Theme
#[derive(Debug, Clone)]
pub struct Theme {
    pub colors: ThemeColors,
    pub metrics: ThemeMetrics,
    pub fonts: ThemeFonts,
}

/// Theme colors
#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub background: [f32; 4],
    pub surface: [f32; 4],
    pub primary: [f32; 4],
    pub secondary: [f32; 4],
    pub accent: [f32; 4],
    pub error: [f32; 4],
    pub warning: [f32; 4],
    pub success: [f32; 4],
    pub text_primary: [f32; 4],
    pub text_secondary: [f32; 4],
    pub text_disabled: [f32; 4],
    pub border: [f32; 4],
    pub divider: [f32; 4],
    pub hover: [f32; 4],
    pub pressed: [f32; 4],
    pub focus: [f32; 4],
}

/// Theme metrics
#[derive(Debug, Clone)]
pub struct ThemeMetrics {
    pub border_radius: f32,
    pub border_width: f32,
    pub spacing_xs: f32,
    pub spacing_sm: f32,
    pub spacing_md: f32,
    pub spacing_lg: f32,
    pub spacing_xl: f32,
    pub button_height: f32,
    pub input_height: f32,
    pub titlebar_height: f32,
    pub scrollbar_width: f32,
}

/// Theme fonts
#[derive(Debug, Clone)]
pub struct ThemeFonts {
    pub default: String,
    pub monospace: String,
    pub size_xs: f32,
    pub size_sm: f32,
    pub size_md: f32,
    pub size_lg: f32,
    pub size_xl: f32,
}

impl Theme {
    /// Dark theme
    pub fn dark() -> Self {
        Self {
            colors: ThemeColors {
                background: [0.11, 0.11, 0.12, 1.0],
                surface: [0.16, 0.16, 0.18, 1.0],
                primary: [0.35, 0.55, 0.95, 1.0],
                secondary: [0.45, 0.45, 0.50, 1.0],
                accent: [0.95, 0.55, 0.25, 1.0],
                error: [0.95, 0.30, 0.30, 1.0],
                warning: [0.95, 0.75, 0.25, 1.0],
                success: [0.35, 0.85, 0.45, 1.0],
                text_primary: [0.95, 0.95, 0.95, 1.0],
                text_secondary: [0.70, 0.70, 0.70, 1.0],
                text_disabled: [0.45, 0.45, 0.45, 1.0],
                border: [0.30, 0.30, 0.32, 1.0],
                divider: [0.25, 0.25, 0.27, 1.0],
                hover: [0.25, 0.25, 0.28, 1.0],
                pressed: [0.20, 0.20, 0.22, 1.0],
                focus: [0.35, 0.55, 0.95, 0.5],
            },
            metrics: ThemeMetrics {
                border_radius: 4.0,
                border_width: 1.0,
                spacing_xs: 2.0,
                spacing_sm: 4.0,
                spacing_md: 8.0,
                spacing_lg: 16.0,
                spacing_xl: 24.0,
                button_height: 28.0,
                input_height: 24.0,
                titlebar_height: 30.0,
                scrollbar_width: 12.0,
            },
            fonts: ThemeFonts {
                default: "Inter".to_string(),
                monospace: "JetBrains Mono".to_string(),
                size_xs: 10.0,
                size_sm: 12.0,
                size_md: 14.0,
                size_lg: 18.0,
                size_xl: 24.0,
            },
        }
    }

    /// Light theme
    pub fn light() -> Self {
        Self {
            colors: ThemeColors {
                background: [0.95, 0.95, 0.96, 1.0],
                surface: [1.0, 1.0, 1.0, 1.0],
                primary: [0.20, 0.45, 0.90, 1.0],
                secondary: [0.55, 0.55, 0.60, 1.0],
                accent: [0.90, 0.50, 0.20, 1.0],
                error: [0.90, 0.25, 0.25, 1.0],
                warning: [0.90, 0.70, 0.20, 1.0],
                success: [0.30, 0.80, 0.40, 1.0],
                text_primary: [0.10, 0.10, 0.10, 1.0],
                text_secondary: [0.45, 0.45, 0.45, 1.0],
                text_disabled: [0.65, 0.65, 0.65, 1.0],
                border: [0.80, 0.80, 0.82, 1.0],
                divider: [0.85, 0.85, 0.87, 1.0],
                hover: [0.90, 0.90, 0.92, 1.0],
                pressed: [0.85, 0.85, 0.88, 1.0],
                focus: [0.20, 0.45, 0.90, 0.3],
            },
            metrics: ThemeMetrics {
                border_radius: 4.0,
                border_width: 1.0,
                spacing_xs: 2.0,
                spacing_sm: 4.0,
                spacing_md: 8.0,
                spacing_lg: 16.0,
                spacing_xl: 24.0,
                button_height: 28.0,
                input_height: 24.0,
                titlebar_height: 30.0,
                scrollbar_width: 12.0,
            },
            fonts: ThemeFonts {
                default: "Inter".to_string(),
                monospace: "JetBrains Mono".to_string(),
                size_xs: 10.0,
                size_sm: 12.0,
                size_md: 14.0,
                size_lg: 18.0,
                size_xl: 24.0,
            },
        }
    }
}
