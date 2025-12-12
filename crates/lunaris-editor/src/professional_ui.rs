//! Professional Toolbar and Panels
//!
//! Polished, production-ready editor UI components.

use glam::Vec2;
use super::design_system::*;

// ==================== TOOLBAR ====================

/// Professional toolbar
pub struct ProfessionalToolbar {
    pub groups: Vec<ToolbarGroup>,
    pub height: f32,
    pub padding: f32,
}

/// Toolbar group
pub struct ToolbarGroup {
    pub items: Vec<ToolbarItem>,
    pub separator_after: bool,
}

/// Toolbar item
pub enum ToolbarItem {
    /// Icon button
    IconButton {
        icon: String,
        tooltip: String,
        action_id: String,
        enabled: bool,
        active: bool,
    },
    /// Text button
    TextButton {
        label: String,
        tooltip: String,
        action_id: String,
        enabled: bool,
    },
    /// Dropdown button
    Dropdown {
        icon: Option<String>,
        label: String,
        tooltip: String,
        items: Vec<DropdownItem>,
        selected: usize,
    },
    /// Toggle button
    Toggle {
        icon_on: String,
        icon_off: String,
        tooltip: String,
        action_id: String,
        enabled: bool,
        active: bool,
    },
    /// Spacer
    Spacer,
    /// Flexible spacer
    FlexSpacer,
}

/// Dropdown item
pub struct DropdownItem {
    pub label: String,
    pub icon: Option<String>,
    pub shortcut: Option<String>,
    pub enabled: bool,
    pub action_id: String,
}

impl ProfessionalToolbar {
    /// Create default editor toolbar
    pub fn default_editor() -> Self {
        Self {
            height: 40.0,
            padding: 8.0,
            groups: vec![
                // File operations
                ToolbarGroup {
                    items: vec![
                        ToolbarItem::IconButton {
                            icon: "file".to_string(),
                            tooltip: "New Scene (Ctrl+N)".to_string(),
                            action_id: "file.new".to_string(),
                            enabled: true,
                            active: false,
                        },
                        ToolbarItem::IconButton {
                            icon: "folder".to_string(),
                            tooltip: "Open Scene (Ctrl+O)".to_string(),
                            action_id: "file.open".to_string(),
                            enabled: true,
                            active: false,
                        },
                        ToolbarItem::IconButton {
                            icon: "save".to_string(),
                            tooltip: "Save Scene (Ctrl+S)".to_string(),
                            action_id: "file.save".to_string(),
                            enabled: true,
                            active: false,
                        },
                    ],
                    separator_after: true,
                },
                // Edit operations
                ToolbarGroup {
                    items: vec![
                        ToolbarItem::IconButton {
                            icon: "undo".to_string(),
                            tooltip: "Undo (Ctrl+Z)".to_string(),
                            action_id: "edit.undo".to_string(),
                            enabled: true,
                            active: false,
                        },
                        ToolbarItem::IconButton {
                            icon: "redo".to_string(),
                            tooltip: "Redo (Ctrl+Y)".to_string(),
                            action_id: "edit.redo".to_string(),
                            enabled: true,
                            active: false,
                        },
                    ],
                    separator_after: true,
                },
                // Transform tools
                ToolbarGroup {
                    items: vec![
                        ToolbarItem::Toggle {
                            icon_on: "move".to_string(),
                            icon_off: "move".to_string(),
                            tooltip: "Move Tool (W)".to_string(),
                            action_id: "tool.move".to_string(),
                            enabled: true,
                            active: true,
                        },
                        ToolbarItem::Toggle {
                            icon_on: "rotate".to_string(),
                            icon_off: "rotate".to_string(),
                            tooltip: "Rotate Tool (E)".to_string(),
                            action_id: "tool.rotate".to_string(),
                            enabled: true,
                            active: false,
                        },
                        ToolbarItem::Toggle {
                            icon_on: "scale".to_string(),
                            icon_off: "scale".to_string(),
                            tooltip: "Scale Tool (R)".to_string(),
                            action_id: "tool.scale".to_string(),
                            enabled: true,
                            active: false,
                        },
                    ],
                    separator_after: true,
                },
                // Playback
                ToolbarGroup {
                    items: vec![
                        ToolbarItem::FlexSpacer,
                        ToolbarItem::IconButton {
                            icon: "skip-back".to_string(),
                            tooltip: "Go to Start".to_string(),
                            action_id: "play.start".to_string(),
                            enabled: true,
                            active: false,
                        },
                        ToolbarItem::Toggle {
                            icon_on: "pause".to_string(),
                            icon_off: "play".to_string(),
                            tooltip: "Play/Pause (Space)".to_string(),
                            action_id: "play.toggle".to_string(),
                            enabled: true,
                            active: false,
                        },
                        ToolbarItem::IconButton {
                            icon: "stop".to_string(),
                            tooltip: "Stop (Esc)".to_string(),
                            action_id: "play.stop".to_string(),
                            enabled: true,
                            active: false,
                        },
                        ToolbarItem::FlexSpacer,
                    ],
                    separator_after: false,
                },
                // Right side
                ToolbarGroup {
                    items: vec![
                        ToolbarItem::FlexSpacer,
                        ToolbarItem::Dropdown {
                            icon: Some("settings".to_string()),
                            label: "Layout".to_string(),
                            tooltip: "Change Layout".to_string(),
                            items: vec![
                                DropdownItem {
                                    label: "Default".to_string(),
                                    icon: None,
                                    shortcut: None,
                                    enabled: true,
                                    action_id: "layout.default".to_string(),
                                },
                                DropdownItem {
                                    label: "2x2 Grid".to_string(),
                                    icon: None,
                                    shortcut: None,
                                    enabled: true,
                                    action_id: "layout.grid".to_string(),
                                },
                                DropdownItem {
                                    label: "Wide".to_string(),
                                    icon: None,
                                    shortcut: None,
                                    enabled: true,
                                    action_id: "layout.wide".to_string(),
                                },
                            ],
                            selected: 0,
                        },
                    ],
                    separator_after: false,
                },
            ],
        }
    }
}

// ==================== PROFESSIONAL PANELS ====================

/// Panel header
pub struct PanelHeader {
    pub title: String,
    pub icon: Option<String>,
    pub collapsible: bool,
    pub collapsed: bool,
    pub closable: bool,
    pub menu_items: Vec<DropdownItem>,
}

/// Hierarchy panel with tree view
pub struct HierarchyPanel {
    pub header: PanelHeader,
    pub items: Vec<HierarchyItem>,
    pub selected: Option<u64>,
    pub multi_select: Vec<u64>,
    pub search_query: String,
    pub filter_type: Option<String>,
    pub drag_source: Option<u64>,
    pub drop_target: Option<u64>,
}

/// Hierarchy item
#[derive(Debug, Clone)]
pub struct HierarchyItem {
    pub id: u64,
    pub name: String,
    pub icon: String,
    pub icon_color: Option<[f32; 4]>,
    pub parent: Option<u64>,
    pub children: Vec<u64>,
    pub expanded: bool,
    pub visible: bool,
    pub locked: bool,
    pub has_prefab: bool,
    pub is_prefab_root: bool,
    pub depth: u32,
}

impl HierarchyPanel {
    /// Create new hierarchy panel
    pub fn new() -> Self {
        Self {
            header: PanelHeader {
                title: "Hierarchy".to_string(),
                icon: Some("hierarchy".to_string()),
                collapsible: true,
                collapsed: false,
                closable: true,
                menu_items: vec![
                    DropdownItem {
                        label: "Create Empty".to_string(),
                        icon: Some("plus".to_string()),
                        shortcut: Some("Ctrl+Shift+N".to_string()),
                        enabled: true,
                        action_id: "hierarchy.create_empty".to_string(),
                    },
                    DropdownItem {
                        label: "Create Cube".to_string(),
                        icon: Some("cube".to_string()),
                        shortcut: None,
                        enabled: true,
                        action_id: "hierarchy.create_cube".to_string(),
                    },
                    DropdownItem {
                        label: "Create Sphere".to_string(),
                        icon: Some("sphere".to_string()),
                        shortcut: None,
                        enabled: true,
                        action_id: "hierarchy.create_sphere".to_string(),
                    },
                    DropdownItem {
                        label: "Create Light".to_string(),
                        icon: Some("light".to_string()),
                        shortcut: None,
                        enabled: true,
                        action_id: "hierarchy.create_light".to_string(),
                    },
                    DropdownItem {
                        label: "Create Camera".to_string(),
                        icon: Some("camera".to_string()),
                        shortcut: None,
                        enabled: true,
                        action_id: "hierarchy.create_camera".to_string(),
                    },
                ],
            },
            items: Vec::new(),
            selected: None,
            multi_select: Vec::new(),
            search_query: String::new(),
            filter_type: None,
            drag_source: None,
            drop_target: None,
        }
    }

    /// Toggle item expansion
    pub fn toggle_expand(&mut self, id: u64) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.expanded = !item.expanded;
        }
    }

    /// Select item
    pub fn select(&mut self, id: u64, shift: bool, ctrl: bool) {
        if ctrl {
            // Toggle selection
            if self.multi_select.contains(&id) {
                self.multi_select.retain(|&i| i != id);
            } else {
                self.multi_select.push(id);
            }
            self.selected = Some(id);
        } else if shift {
            // Range selection (simplified)
            if let Some(sel) = self.selected {
                // Add range to multi_select
                self.multi_select.push(id);
            }
            self.selected = Some(id);
        } else {
            // Single selection
            self.multi_select.clear();
            self.selected = Some(id);
        }
    }

    /// Filter items by search
    pub fn filter(&self) -> Vec<&HierarchyItem> {
        let query = self.search_query.to_lowercase();
        self.items.iter()
            .filter(|item| {
                if query.is_empty() {
                    return true;
                }
                item.name.to_lowercase().contains(&query)
            })
            .filter(|item| {
                match &self.filter_type {
                    Some(t) => item.icon == *t,
                    None => true,
                }
            })
            .collect()
    }
}

/// Inspector panel
pub struct InspectorPanel {
    pub header: PanelHeader,
    pub sections: Vec<InspectorSection>,
    pub show_debug: bool,
}

/// Inspector section (component)
pub struct InspectorSection {
    pub title: String,
    pub icon: String,
    pub expanded: bool,
    pub removable: bool,
    pub enabled: bool,
    pub properties: Vec<InspectorProperty>,
}

/// Inspector property
pub struct InspectorProperty {
    pub name: String,
    pub property_type: PropertyType,
    pub tooltip: String,
    pub read_only: bool,
}

/// Property type
pub enum PropertyType {
    Bool(bool),
    Int { value: i32, min: Option<i32>, max: Option<i32>, step: i32 },
    Float { value: f32, min: Option<f32>, max: Option<f32>, step: f32, precision: u32 },
    String { value: String, multiline: bool, max_length: Option<usize> },
    Vec2 { value: [f32; 2] },
    Vec3 { value: [f32; 3] },
    Vec4 { value: [f32; 4] },
    Color { value: [f32; 4], has_alpha: bool },
    Enum { value: usize, options: Vec<String> },
    Asset { value: Option<u64>, asset_type: String },
    Object { value: Option<u64> },
    Curve,
    Gradient,
}

impl InspectorPanel {
    /// Create new inspector
    pub fn new() -> Self {
        Self {
            header: PanelHeader {
                title: "Inspector".to_string(),
                icon: Some("inspector".to_string()),
                collapsible: true,
                collapsed: false,
                closable: true,
                menu_items: vec![
                    DropdownItem {
                        label: "Lock Inspector".to_string(),
                        icon: Some("lock".to_string()),
                        shortcut: None,
                        enabled: true,
                        action_id: "inspector.lock".to_string(),
                    },
                    DropdownItem {
                        label: "Debug Mode".to_string(),
                        icon: None,
                        shortcut: None,
                        enabled: true,
                        action_id: "inspector.debug".to_string(),
                    },
                ],
            },
            sections: Vec::new(),
            show_debug: false,
        }
    }

    /// Show entity inspector
    pub fn show_entity(&mut self, name: &str) {
        self.sections.clear();

        // Transform section
        self.sections.push(InspectorSection {
            title: "Transform".to_string(),
            icon: "move".to_string(),
            expanded: true,
            removable: false,
            enabled: true,
            properties: vec![
                InspectorProperty {
                    name: "Position".to_string(),
                    property_type: PropertyType::Vec3 { value: [0.0, 0.0, 0.0] },
                    tooltip: "World position".to_string(),
                    read_only: false,
                },
                InspectorProperty {
                    name: "Rotation".to_string(),
                    property_type: PropertyType::Vec3 { value: [0.0, 0.0, 0.0] },
                    tooltip: "Euler rotation in degrees".to_string(),
                    read_only: false,
                },
                InspectorProperty {
                    name: "Scale".to_string(),
                    property_type: PropertyType::Vec3 { value: [1.0, 1.0, 1.0] },
                    tooltip: "Scale factor".to_string(),
                    read_only: false,
                },
            ],
        });
    }
}

/// Console panel
pub struct ConsolePanel {
    pub header: PanelHeader,
    pub messages: Vec<ConsoleMessage>,
    pub filter: ConsoleFilter,
    pub search_query: String,
    pub auto_scroll: bool,
    pub show_timestamps: bool,
    pub max_messages: usize,
}

/// Console message
#[derive(Debug, Clone)]
pub struct ConsoleMessage {
    pub level: LogLevel,
    pub message: String,
    pub timestamp: String,
    pub source: Option<String>,
    pub count: u32,
    pub stack_trace: Option<String>,
}

/// Log level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

impl LogLevel {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Trace => "…",
            Self::Debug => "🔍",
            Self::Info => "ℹ",
            Self::Warning => "⚠",
            Self::Error => "✕",
        }
    }

    pub fn color(&self) -> [f32; 4] {
        match self {
            Self::Trace => [0.5, 0.5, 0.5, 1.0],
            Self::Debug => [0.6, 0.6, 0.8, 1.0],
            Self::Info => [0.8, 0.8, 0.8, 1.0],
            Self::Warning => [1.0, 0.8, 0.2, 1.0],
            Self::Error => [1.0, 0.4, 0.4, 1.0],
        }
    }
}

/// Console filter
#[derive(Debug, Clone)]
pub struct ConsoleFilter {
    pub show_trace: bool,
    pub show_debug: bool,
    pub show_info: bool,
    pub show_warning: bool,
    pub show_error: bool,
    pub collapse_similar: bool,
}

impl Default for ConsoleFilter {
    fn default() -> Self {
        Self {
            show_trace: false,
            show_debug: true,
            show_info: true,
            show_warning: true,
            show_error: true,
            collapse_similar: true,
        }
    }
}

impl ConsolePanel {
    /// Create new console
    pub fn new() -> Self {
        Self {
            header: PanelHeader {
                title: "Console".to_string(),
                icon: Some("console".to_string()),
                collapsible: true,
                collapsed: false,
                closable: true,
                menu_items: vec![
                    DropdownItem {
                        label: "Clear".to_string(),
                        icon: Some("x".to_string()),
                        shortcut: Some("Ctrl+L".to_string()),
                        enabled: true,
                        action_id: "console.clear".to_string(),
                    },
                ],
            },
            messages: Vec::new(),
            filter: ConsoleFilter::default(),
            search_query: String::new(),
            auto_scroll: true,
            show_timestamps: true,
            max_messages: 1000,
        }
    }

    /// Log message
    pub fn log(&mut self, level: LogLevel, message: &str) {
        // Check for duplicate
        if self.filter.collapse_similar {
            if let Some(last) = self.messages.last_mut() {
                if last.message == message && last.level == level {
                    last.count += 1;
                    return;
                }
            }
        }

        self.messages.push(ConsoleMessage {
            level,
            message: message.to_string(),
            timestamp: chrono_lite(),
            source: None,
            count: 1,
            stack_trace: None,
        });

        // Limit messages
        if self.messages.len() > self.max_messages {
            self.messages.remove(0);
        }
    }

    /// Clear console
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    /// Get filtered messages
    pub fn filtered_messages(&self) -> Vec<&ConsoleMessage> {
        let query = self.search_query.to_lowercase();
        self.messages.iter()
            .filter(|m| {
                match m.level {
                    LogLevel::Trace => self.filter.show_trace,
                    LogLevel::Debug => self.filter.show_debug,
                    LogLevel::Info => self.filter.show_info,
                    LogLevel::Warning => self.filter.show_warning,
                    LogLevel::Error => self.filter.show_error,
                }
            })
            .filter(|m| {
                if query.is_empty() {
                    true
                } else {
                    m.message.to_lowercase().contains(&query)
                }
            })
            .collect()
    }

    /// Count by level
    pub fn count_by_level(&self, level: LogLevel) -> usize {
        self.messages.iter().filter(|m| m.level == level).count()
    }
}

/// Simple timestamp function
fn chrono_lite() -> String {
    "00:00:00".to_string() // Would use actual time
}

/// Asset browser panel
pub struct AssetBrowserPanel {
    pub header: PanelHeader,
    pub current_path: String,
    pub items: Vec<AssetItem>,
    pub selected: Vec<u64>,
    pub view_mode: AssetViewMode,
    pub thumbnail_size: f32,
    pub search_query: String,
    pub filter_types: Vec<String>,
}

/// Asset item
#[derive(Debug, Clone)]
pub struct AssetItem {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub asset_type: String,
    pub icon: String,
    pub thumbnail: Option<String>,
    pub size_bytes: u64,
    pub modified: String,
    pub is_folder: bool,
}

/// Asset view mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetViewMode {
    Grid,
    List,
    Columns,
}

impl AssetBrowserPanel {
    /// Create new asset browser
    pub fn new() -> Self {
        Self {
            header: PanelHeader {
                title: "Assets".to_string(),
                icon: Some("assets".to_string()),
                collapsible: true,
                collapsed: false,
                closable: true,
                menu_items: vec![
                    DropdownItem {
                        label: "Import Asset...".to_string(),
                        icon: Some("plus".to_string()),
                        shortcut: Some("Ctrl+I".to_string()),
                        enabled: true,
                        action_id: "assets.import".to_string(),
                    },
                    DropdownItem {
                        label: "Refresh".to_string(),
                        icon: None,
                        shortcut: Some("F5".to_string()),
                        enabled: true,
                        action_id: "assets.refresh".to_string(),
                    },
                ],
            },
            current_path: "/".to_string(),
            items: Vec::new(),
            selected: Vec::new(),
            view_mode: AssetViewMode::Grid,
            thumbnail_size: 96.0,
            search_query: String::new(),
            filter_types: Vec::new(),
        }
    }

    /// Navigate to folder
    pub fn navigate(&mut self, path: &str) {
        self.current_path = path.to_string();
        self.selected.clear();
        // Would load folder contents
    }

    /// Go up one level
    pub fn go_up(&mut self) {
        let parent_path = self.current_path.rsplit_once('/').map(|(p, _)| p.to_string());
        if let Some(path) = parent_path {
            self.navigate(&path);
        }
    }

    /// Toggle view mode
    pub fn toggle_view(&mut self) {
        self.view_mode = match self.view_mode {
            AssetViewMode::Grid => AssetViewMode::List,
            AssetViewMode::List => AssetViewMode::Columns,
            AssetViewMode::Columns => AssetViewMode::Grid,
        };
    }
}
