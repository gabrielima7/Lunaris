//! Advanced UI Layout System
//!
//! Docking, tabs, and flexible workspace layouts like Unreal Slate.

use glam::Vec2;
use std::collections::HashMap;

/// Unique identifier for dock nodes
pub type DockNodeId = u64;

/// Unique identifier for tabs
pub type TabId = u64;

/// Dock split direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

/// Dock node type
#[derive(Debug, Clone)]
pub enum DockNode {
    /// Leaf node containing tabs
    Leaf {
        tabs: Vec<TabId>,
        active_tab: usize,
    },
    /// Split node containing two children
    Split {
        direction: SplitDirection,
        ratio: f32,
        first: Box<DockNode>,
        second: Box<DockNode>,
    },
}

impl DockNode {
    /// Create empty leaf
    #[must_use]
    pub fn empty() -> Self {
        Self::Leaf {
            tabs: Vec::new(),
            active_tab: 0,
        }
    }

    /// Create leaf with single tab
    #[must_use]
    pub fn with_tab(tab_id: TabId) -> Self {
        Self::Leaf {
            tabs: vec![tab_id],
            active_tab: 0,
        }
    }

    /// Split this node
    pub fn split(&mut self, direction: SplitDirection, new_tab: TabId) {
        let old_node = std::mem::replace(self, Self::empty());
        *self = Self::Split {
            direction,
            ratio: 0.5,
            first: Box::new(old_node),
            second: Box::new(Self::with_tab(new_tab)),
        };
    }

    /// Add tab to leaf node
    pub fn add_tab(&mut self, tab_id: TabId) {
        if let Self::Leaf { tabs, active_tab } = self {
            tabs.push(tab_id);
            *active_tab = tabs.len() - 1;
        }
    }

    /// Remove tab from leaf node
    pub fn remove_tab(&mut self, tab_id: TabId) -> bool {
        if let Self::Leaf { tabs, active_tab } = self {
            if let Some(idx) = tabs.iter().position(|&t| t == tab_id) {
                tabs.remove(idx);
                if *active_tab >= tabs.len() && !tabs.is_empty() {
                    *active_tab = tabs.len() - 1;
                }
                return true;
            }
        }
        false
    }
}

/// Tab information
#[derive(Debug, Clone)]
pub struct Tab {
    /// Tab ID
    pub id: TabId,
    /// Tab title
    pub title: String,
    /// Tab icon (optional)
    pub icon: Option<String>,
    /// Can close
    pub closable: bool,
    /// Content callback identifier
    pub content_id: String,
}

/// Dock space manages the entire docking layout
pub struct DockSpace {
    /// Root dock node
    pub root: DockNode,
    /// All registered tabs
    pub tabs: HashMap<TabId, Tab>,
    /// Next tab ID
    next_tab_id: TabId,
    /// Dragging state
    dragging: Option<DragState>,
    /// Drop preview
    drop_preview: Option<DropPreview>,
    /// Bounds
    pub bounds: Rect,
}

/// Drag state
#[derive(Debug, Clone)]
struct DragState {
    tab_id: TabId,
    offset: Vec2,
}

/// Drop preview
#[derive(Debug, Clone)]
struct DropPreview {
    bounds: Rect,
    direction: Option<SplitDirection>,
}

/// Rectangle
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    #[must_use]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    #[must_use]
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }

    #[must_use]
    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    #[must_use]
    pub fn split_horizontal(&self, ratio: f32) -> (Rect, Rect) {
        let first_width = self.width * ratio;
        (
            Rect::new(self.x, self.y, first_width, self.height),
            Rect::new(self.x + first_width, self.y, self.width - first_width, self.height),
        )
    }

    #[must_use]
    pub fn split_vertical(&self, ratio: f32) -> (Rect, Rect) {
        let first_height = self.height * ratio;
        (
            Rect::new(self.x, self.y, self.width, first_height),
            Rect::new(self.x, self.y + first_height, self.width, self.height - first_height),
        )
    }
}

impl Default for DockSpace {
    fn default() -> Self {
        Self::new()
    }
}

impl DockSpace {
    /// Create new dock space
    #[must_use]
    pub fn new() -> Self {
        Self {
            root: DockNode::empty(),
            tabs: HashMap::new(),
            next_tab_id: 1,
            dragging: None,
            drop_preview: None,
            bounds: Rect::new(0.0, 0.0, 1920.0, 1080.0),
        }
    }

    /// Create tab and return ID
    pub fn create_tab(&mut self, title: &str, content_id: &str, closable: bool) -> TabId {
        let id = self.next_tab_id;
        self.next_tab_id += 1;

        let tab = Tab {
            id,
            title: title.to_string(),
            icon: None,
            closable,
            content_id: content_id.to_string(),
        };

        self.tabs.insert(id, tab);
        id
    }

    /// Add tab to root (or first available leaf)
    pub fn dock_tab(&mut self, tab_id: TabId) {
        self.root.add_tab(tab_id);
    }

    /// Split root and add tab
    pub fn dock_tab_split(&mut self, tab_id: TabId, direction: SplitDirection) {
        self.root.split(direction, tab_id);
    }

    /// Get tab info
    #[must_use]
    pub fn get_tab(&self, id: TabId) -> Option<&Tab> {
        self.tabs.get(&id)
    }

    /// Close tab
    pub fn close_tab(&mut self, id: TabId) {
        self.root.remove_tab(id);
        self.tabs.remove(&id);
    }

    /// Start dragging a tab
    pub fn start_drag(&mut self, tab_id: TabId, offset: Vec2) {
        self.dragging = Some(DragState { tab_id, offset });
    }

    /// Update drag
    pub fn update_drag(&mut self, mouse_pos: Vec2) {
        if let Some(ref drag) = self.dragging {
            // Calculate drop preview
            let center = self.bounds.center();
            let dx = mouse_pos.x - center.x;
            let dy = mouse_pos.y - center.y;

            let direction = if dx.abs() > dy.abs() {
                if dx > 0.0 {
                    Some(SplitDirection::Horizontal)
                } else {
                    Some(SplitDirection::Horizontal)
                }
            } else if dy > 0.0 {
                Some(SplitDirection::Vertical)
            } else {
                Some(SplitDirection::Vertical)
            };

            self.drop_preview = Some(DropPreview {
                bounds: self.bounds,
                direction,
            });
        }
    }

    /// End drag
    pub fn end_drag(&mut self, mouse_pos: Vec2) {
        if let Some(drag) = self.dragging.take() {
            if let Some(preview) = self.drop_preview.take() {
                if let Some(direction) = preview.direction {
                    // Remove tab from current location
                    self.root.remove_tab(drag.tab_id);
                    // Add to new location with split
                    self.root.split(direction, drag.tab_id);
                }
            }
        }
    }

    /// Calculate layout rectangles
    pub fn calculate_layout(&self, node: &DockNode, bounds: Rect) -> Vec<(Rect, Vec<TabId>)> {
        let mut result = Vec::new();

        match node {
            DockNode::Leaf { tabs, .. } => {
                result.push((bounds, tabs.clone()));
            }
            DockNode::Split { direction, ratio, first, second } => {
                let (first_bounds, second_bounds) = match direction {
                    SplitDirection::Horizontal => bounds.split_horizontal(*ratio),
                    SplitDirection::Vertical => bounds.split_vertical(*ratio),
                };

                result.extend(self.calculate_layout(first, first_bounds));
                result.extend(self.calculate_layout(second, second_bounds));
            }
        }

        result
    }
}

/// Preset workspace layouts
pub struct WorkspacePreset {
    /// Preset name
    pub name: String,
    /// Layout configuration
    pub layout: DockNode,
}

impl WorkspacePreset {
    /// Default editor layout
    #[must_use]
    pub fn default_editor() -> Self {
        // Left: Hierarchy | Center: Viewport | Right: Inspector | Bottom: Console/Assets
        Self {
            name: "Default".to_string(),
            layout: DockNode::Split {
                direction: SplitDirection::Vertical,
                ratio: 0.7,
                first: Box::new(DockNode::Split {
                    direction: SplitDirection::Horizontal,
                    ratio: 0.2,
                    first: Box::new(DockNode::with_tab(1)), // Hierarchy
                    second: Box::new(DockNode::Split {
                        direction: SplitDirection::Horizontal,
                        ratio: 0.75,
                        first: Box::new(DockNode::with_tab(2)), // Viewport
                        second: Box::new(DockNode::with_tab(3)), // Inspector
                    }),
                }),
                second: Box::new(DockNode::Leaf {
                    tabs: vec![4, 5], // Console, Assets
                    active_tab: 0,
                }),
            },
        }
    }

    /// Scripting focused layout
    #[must_use]
    pub fn scripting() -> Self {
        Self {
            name: "Scripting".to_string(),
            layout: DockNode::Split {
                direction: SplitDirection::Horizontal,
                ratio: 0.6,
                first: Box::new(DockNode::with_tab(10)), // Code Editor
                second: Box::new(DockNode::Split {
                    direction: SplitDirection::Vertical,
                    ratio: 0.5,
                    first: Box::new(DockNode::with_tab(11)), // Variables
                    second: Box::new(DockNode::with_tab(12)), // Console
                }),
            },
        }
    }

    /// Animation focused layout
    #[must_use]
    pub fn animation() -> Self {
        Self {
            name: "Animation".to_string(),
            layout: DockNode::Split {
                direction: SplitDirection::Vertical,
                ratio: 0.6,
                first: Box::new(DockNode::Split {
                    direction: SplitDirection::Horizontal,
                    ratio: 0.3,
                    first: Box::new(DockNode::with_tab(20)), // Skeleton
                    second: Box::new(DockNode::with_tab(21)), // Viewport
                }),
                second: Box::new(DockNode::Leaf {
                    tabs: vec![22, 23], // Timeline, Curves
                    active_tab: 0,
                }),
            },
        }
    }
}

/// Workspace manager
pub struct WorkspaceManager {
    /// Current dock space
    pub dock_space: DockSpace,
    /// Saved presets
    pub presets: Vec<WorkspacePreset>,
    /// Current preset index
    pub current_preset: usize,
}

impl Default for WorkspaceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceManager {
    /// Create new workspace manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            dock_space: DockSpace::new(),
            presets: vec![
                WorkspacePreset::default_editor(),
                WorkspacePreset::scripting(),
                WorkspacePreset::animation(),
            ],
            current_preset: 0,
        }
    }

    /// Apply preset
    pub fn apply_preset(&mut self, index: usize) {
        if let Some(preset) = self.presets.get(index) {
            self.dock_space.root = preset.layout.clone();
            self.current_preset = index;
        }
    }

    /// Save current as preset
    pub fn save_preset(&mut self, name: &str) {
        self.presets.push(WorkspacePreset {
            name: name.to_string(),
            layout: self.dock_space.root.clone(),
        });
    }
}
