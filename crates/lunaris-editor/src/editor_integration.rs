//! Editor Integration Layer
//!
//! Connects all visual editors with the dock system and retained UI.
//! This is the glue that makes everything work together.

use glam::{Vec2, Vec3};
use std::collections::HashMap;

use super::dock::{DockTree, DockNode, DockArea, TabInfo};
use super::ui_retained::{UiTree, WidgetId, WidgetType, WidgetState, Theme};
use super::visual_graph::{VisualGraphEditor, GraphType};
use super::curve_editor::{CurveEditorWidget, AnimationCurve};
use super::viewport::{ViewportWidget, RenderMode, ViewType};
use super::professional_ui::{HierarchyPanel, InspectorPanel, ConsolePanel, AssetBrowserPanel};
use super::shortcuts::{ShortcutManager, Shortcut};
use super::design_system::DesignTokens;

// ==================== EDITOR CONTEXT ====================

/// Central editor context - the spine of the editor
pub struct EditorContext {
    /// Dock layout
    pub dock: DockTree,
    /// UI tree
    pub ui: UiTree,
    /// Theme
    pub theme: Theme,
    /// Design tokens
    pub tokens: DesignTokens,
    /// Shortcuts
    pub shortcuts: ShortcutManager,
    /// All open panels
    pub panels: PanelManager,
    /// Selection state
    pub selection: SelectionState,
    /// Clipboard
    pub clipboard: ClipboardState,
    /// Undo/Redo
    pub history: EditorHistory,
    /// Asset database
    pub assets: AssetDatabase,
    /// Active tool
    pub active_tool: EditorTool,
    /// Is playing
    pub is_playing: bool,
    /// Is paused
    pub is_paused: bool,
    /// Hot reload state
    pub hot_reload: HotReloadState,
    /// Recent files
    pub recent_files: Vec<String>,
    /// Current project
    pub project: Option<Project>,
    /// Frame stats
    pub frame_stats: FrameStats,
}

impl EditorContext {
    /// Create new editor context
    pub fn new() -> Self {
        let mut ctx = Self {
            dock: DockTree::new(),
            ui: UiTree::new(),
            theme: Theme::dark(),
            tokens: DesignTokens::dark(),
            shortcuts: ShortcutManager::new(),
            panels: PanelManager::new(),
            selection: SelectionState::default(),
            clipboard: ClipboardState::default(),
            history: EditorHistory::new(100),
            assets: AssetDatabase::new(),
            active_tool: EditorTool::Select,
            is_playing: false,
            is_paused: false,
            hot_reload: HotReloadState::default(),
            recent_files: Vec::new(),
            project: None,
            frame_stats: FrameStats::default(),
        };

        // Setup default layout
        ctx.setup_default_layout();

        ctx
    }

    /// Setup default editor layout
    fn setup_default_layout(&mut self) {
        // Create main areas
        let viewport_id = self.panels.create_viewport("Main Viewport");
        let hierarchy_id = self.panels.create_hierarchy();
        let inspector_id = self.panels.create_inspector();
        let console_id = self.panels.create_console();
        let asset_browser_id = self.panels.create_asset_browser();

        // Setup dock tree with default layout
        // Left: Hierarchy
        // Center: Viewport
        // Right: Inspector
        // Bottom: Console + Assets
        self.dock.setup_default_layout(vec![
            ("Hierarchy".to_string(), hierarchy_id),
            ("Viewport".to_string(), viewport_id),
            ("Inspector".to_string(), inspector_id),
            ("Console".to_string(), console_id),
            ("Assets".to_string(), asset_browser_id),
        ]);
    }

    /// Update editor
    pub fn update(&mut self, dt: f32) {
        self.frame_stats.begin_frame();

        // Update hot reload
        self.hot_reload.check_changes();

        // Update all panels
        self.panels.update(dt, &self.selection);

        // Process shortcuts from input
        // (Would be called from input system)

        self.frame_stats.end_frame(dt);
    }

    /// Handle shortcut
    pub fn handle_shortcut(&mut self, shortcut: &Shortcut) -> bool {
        if let Some(action) = self.shortcuts.find_action(shortcut) {
            self.execute_action(&action.action_id.clone());
            true
        } else {
            false
        }
    }

    /// Execute editor action
    pub fn execute_action(&mut self, action_id: &str) {
        match action_id {
            // File actions
            "file.new" => self.new_scene(),
            "file.open" => self.open_scene_dialog(),
            "file.save" => self.save_scene(),
            "file.save_as" => self.save_scene_as_dialog(),

            // Edit actions
            "edit.undo" => self.undo(),
            "edit.redo" => self.redo(),
            "edit.cut" => self.cut(),
            "edit.copy" => self.copy(),
            "edit.paste" => self.paste(),
            "edit.duplicate" => self.duplicate(),
            "edit.delete" => self.delete_selected(),
            "edit.select_all" => self.select_all(),

            // Tool actions
            "tool.select" => self.set_tool(EditorTool::Select),
            "tool.move" => self.set_tool(EditorTool::Move),
            "tool.rotate" => self.set_tool(EditorTool::Rotate),
            "tool.scale" => self.set_tool(EditorTool::Scale),

            // Play actions
            "play.toggle" => self.toggle_play(),
            "play.pause" => self.toggle_pause(),
            "play.stop" => self.stop_play(),

            // View actions
            "view.frame" => self.frame_selected(),
            "view.frame_all" => self.frame_all(),

            // Window actions
            "window.console" => self.toggle_panel(PanelType::Console),
            "window.hierarchy" => self.toggle_panel(PanelType::Hierarchy),
            "window.inspector" => self.toggle_panel(PanelType::Inspector),
            "window.assets" => self.toggle_panel(PanelType::AssetBrowser),

            _ => {
                tracing::warn!("Unknown action: {}", action_id);
            }
        }
    }

    // === File Operations ===

    fn new_scene(&mut self) {
        // Clear scene, reset state
        self.selection.clear();
        self.panels.hierarchy.items.clear();
        tracing::info!("Created new scene");
    }

    fn open_scene_dialog(&mut self) {
        // Would show file dialog
        tracing::info!("Open scene dialog");
    }

    fn save_scene(&mut self) {
        if let Some(ref project) = self.project {
            // Would save to current path
            tracing::info!("Saved scene to {}", project.current_scene_path);
        }
    }

    fn save_scene_as_dialog(&mut self) {
        // Would show save dialog
        tracing::info!("Save scene as dialog");
    }

    // === Edit Operations ===

    fn undo(&mut self) {
        self.history.undo();
    }

    fn redo(&mut self) {
        self.history.redo();
    }

    fn cut(&mut self) {
        self.copy();
        self.delete_selected();
    }

    fn copy(&mut self) {
        self.clipboard.entities = self.selection.entities.clone();
    }

    fn paste(&mut self) {
        // Would duplicate entities from clipboard
        for entity_id in &self.clipboard.entities {
            // Create duplicate at offset position
            tracing::debug!("Paste entity {}", entity_id);
        }
    }

    fn duplicate(&mut self) {
        self.copy();
        self.paste();
    }

    fn delete_selected(&mut self) {
        for entity_id in &self.selection.entities {
            // Would delete from scene
            tracing::debug!("Delete entity {}", entity_id);
        }
        self.selection.clear();
    }

    fn select_all(&mut self) {
        // Would select all entities in hierarchy
        tracing::debug!("Select all");
    }

    // === Tools ===

    fn set_tool(&mut self, tool: EditorTool) {
        self.active_tool = tool;
        // Update viewport gizmo
        for viewport in &mut self.panels.viewports {
            match tool {
                EditorTool::Move => viewport.gizmo.gizmo_type = super::gizmo::GizmoType::Translate,
                EditorTool::Rotate => viewport.gizmo.gizmo_type = super::gizmo::GizmoType::Rotate,
                EditorTool::Scale => viewport.gizmo.gizmo_type = super::gizmo::GizmoType::Scale,
                _ => {}
            }
        }
    }

    // === Play Mode ===

    fn toggle_play(&mut self) {
        if self.is_playing {
            self.stop_play();
        } else {
            self.start_play();
        }
    }

    fn start_play(&mut self) {
        self.is_playing = true;
        self.is_paused = false;
        tracing::info!("Entering play mode");
    }

    fn toggle_pause(&mut self) {
        if self.is_playing {
            self.is_paused = !self.is_paused;
        }
    }

    fn stop_play(&mut self) {
        self.is_playing = false;
        self.is_paused = false;
        tracing::info!("Exiting play mode");
    }

    // === View ===

    fn frame_selected(&mut self) {
        for viewport in &mut self.panels.viewports {
            viewport.frame_selected();
        }
    }

    fn frame_all(&mut self) {
        for viewport in &mut self.panels.viewports {
            viewport.camera.frame_bounds(Vec3::ZERO, 10.0);
        }
    }

    fn toggle_panel(&mut self, panel_type: PanelType) {
        // Toggle visibility in dock
        match panel_type {
            PanelType::Console => {
                self.panels.console.header.collapsed = !self.panels.console.header.collapsed;
            }
            PanelType::Hierarchy => {
                self.panels.hierarchy.header.collapsed = !self.panels.hierarchy.header.collapsed;
            }
            PanelType::Inspector => {
                self.panels.inspector.header.collapsed = !self.panels.inspector.header.collapsed;
            }
            PanelType::AssetBrowser => {
                self.panels.asset_browser.header.collapsed = !self.panels.asset_browser.header.collapsed;
            }
            _ => {}
        }
    }

    /// Open visual graph editor
    pub fn open_graph_editor(&mut self, graph_type: GraphType, name: &str) -> u64 {
        let id = self.panels.create_graph_editor(graph_type, name);
        
        // Add to dock
        self.dock.add_tab("Center".to_string(), TabInfo {
            id,
            title: name.to_string(),
            icon: Some("graph".to_string()),
            closable: true,
            dirty: false,
        });

        id
    }

    /// Open curve editor
    pub fn open_curve_editor(&mut self, name: &str) -> u64 {
        let id = self.panels.create_curve_editor(name);

        // Add to dock (usually bottom area)
        self.dock.add_tab("Bottom".to_string(), TabInfo {
            id,
            title: name.to_string(),
            icon: Some("curve".to_string()),
            closable: true,
            dirty: false,
        });

        id
    }

    /// Open additional viewport
    pub fn open_viewport(&mut self, name: &str) -> u64 {
        let id = self.panels.create_viewport(name);

        self.dock.add_tab("Center".to_string(), TabInfo {
            id,
            title: name.to_string(),
            icon: Some("viewport".to_string()),
            closable: true,
            dirty: false,
        });

        id
    }

    /// Set theme
    pub fn set_theme(&mut self, dark: bool) {
        if dark {
            self.theme = Theme::dark();
            self.tokens = DesignTokens::dark();
        } else {
            self.theme = Theme::light();
            self.tokens = DesignTokens::light();
        }
        self.ui = UiTree::new();
    }
}

// ==================== PANEL MANAGER ====================

/// Manages all editor panels
pub struct PanelManager {
    next_id: u64,

    // Core panels
    pub hierarchy: HierarchyPanel,
    pub inspector: InspectorPanel,
    pub console: ConsolePanel,
    pub asset_browser: AssetBrowserPanel,

    // Dynamic panels
    pub viewports: Vec<ViewportWidget>,
    pub graph_editors: Vec<VisualGraphEditor>,
    pub curve_editors: Vec<CurveEditorWidget>,

    // ID mapping
    panel_ids: HashMap<u64, PanelType>,
}

/// Panel types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelType {
    Hierarchy,
    Inspector,
    Console,
    AssetBrowser,
    Viewport,
    GraphEditor,
    CurveEditor,
}

impl PanelManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            hierarchy: HierarchyPanel::new(),
            inspector: InspectorPanel::new(),
            console: ConsolePanel::new(),
            asset_browser: AssetBrowserPanel::new(),
            viewports: Vec::new(),
            graph_editors: Vec::new(),
            curve_editors: Vec::new(),
            panel_ids: HashMap::new(),
        }
    }

    fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn create_hierarchy(&mut self) -> u64 {
        let id = self.next_id();
        self.panel_ids.insert(id, PanelType::Hierarchy);
        id
    }

    pub fn create_inspector(&mut self) -> u64 {
        let id = self.next_id();
        self.panel_ids.insert(id, PanelType::Inspector);
        id
    }

    pub fn create_console(&mut self) -> u64 {
        let id = self.next_id();
        self.panel_ids.insert(id, PanelType::Console);
        id
    }

    pub fn create_asset_browser(&mut self) -> u64 {
        let id = self.next_id();
        self.panel_ids.insert(id, PanelType::AssetBrowser);
        id
    }

    pub fn create_viewport(&mut self, name: &str) -> u64 {
        let id = self.next_id();
        let viewport = ViewportWidget::new(id, name);
        self.viewports.push(viewport);
        self.panel_ids.insert(id, PanelType::Viewport);
        id
    }

    pub fn create_graph_editor(&mut self, graph_type: GraphType, name: &str) -> u64 {
        let id = self.next_id();
        let editor = VisualGraphEditor::new(graph_type, name);
        self.graph_editors.push(editor);
        self.panel_ids.insert(id, PanelType::GraphEditor);
        id
    }

    pub fn create_curve_editor(&mut self, name: &str) -> u64 {
        let id = self.next_id();
        let mut editor = CurveEditorWidget::new();
        // Add default curve
        editor.add_curve(name);
        self.curve_editors.push(editor);
        self.panel_ids.insert(id, PanelType::CurveEditor);
        id
    }

    pub fn update(&mut self, dt: f32, selection: &SelectionState) {
        // Update inspector based on selection
        if let Some(entity_id) = selection.entities.first() {
            self.inspector.show_entity(&format!("Entity_{}", entity_id));
        }

        // Update viewports
        for viewport in &mut self.viewports {
            // Would update animations, etc.
        }

        // Update curve editors
        for editor in &mut self.curve_editors {
            editor.set_time(editor.current_time + dt);
        }
    }

    pub fn get_panel_type(&self, id: u64) -> Option<PanelType> {
        self.panel_ids.get(&id).copied()
    }

    pub fn get_viewport(&mut self, id: u64) -> Option<&mut ViewportWidget> {
        self.viewports.iter_mut().find(|v| v.id == id)
    }

    pub fn get_graph_editor(&mut self, id: u64) -> Option<&mut VisualGraphEditor> {
        // Would need to store id in VisualGraphEditor
        self.graph_editors.first_mut()
    }

    pub fn close_panel(&mut self, id: u64) {
        if let Some(panel_type) = self.panel_ids.remove(&id) {
            match panel_type {
                PanelType::Viewport => {
                    self.viewports.retain(|v| v.id != id);
                }
                PanelType::GraphEditor => {
                    // Would remove by id
                }
                PanelType::CurveEditor => {
                    // Would remove by id
                }
                _ => {} // Core panels can't be closed
            }
        }
    }
}

// ==================== SELECTION STATE ====================

/// Selection state
#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    /// Selected entity IDs
    pub entities: Vec<u64>,
    /// Primary selected entity
    pub primary: Option<u64>,
    /// Selection mode
    pub mode: SelectionMode,
}

/// Selection mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionMode {
    #[default]
    Replace,
    Add,
    Remove,
    Toggle,
}

impl SelectionState {
    pub fn select(&mut self, entity_id: u64, mode: SelectionMode) {
        match mode {
            SelectionMode::Replace => {
                self.entities.clear();
                self.entities.push(entity_id);
                self.primary = Some(entity_id);
            }
            SelectionMode::Add => {
                if !self.entities.contains(&entity_id) {
                    self.entities.push(entity_id);
                }
                self.primary = Some(entity_id);
            }
            SelectionMode::Remove => {
                self.entities.retain(|&e| e != entity_id);
                if self.primary == Some(entity_id) {
                    self.primary = self.entities.first().copied();
                }
            }
            SelectionMode::Toggle => {
                if self.entities.contains(&entity_id) {
                    self.select(entity_id, SelectionMode::Remove);
                } else {
                    self.select(entity_id, SelectionMode::Add);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.entities.clear();
        self.primary = None;
    }

    pub fn is_selected(&self, entity_id: u64) -> bool {
        self.entities.contains(&entity_id)
    }

    pub fn count(&self) -> usize {
        self.entities.len()
    }
}

// ==================== CLIPBOARD ====================

/// Clipboard state
#[derive(Debug, Clone, Default)]
pub struct ClipboardState {
    pub entities: Vec<u64>,
    pub graph_nodes: Vec<u64>,
    pub curve_keyframes: Vec<u64>,
    pub text: Option<String>,
}

// ==================== EDITOR HISTORY ====================

/// Editor history for undo/redo
pub struct EditorHistory {
    undo_stack: Vec<HistoryAction>,
    redo_stack: Vec<HistoryAction>,
    max_size: usize,
}

/// History action
#[derive(Debug, Clone)]
pub enum HistoryAction {
    CreateEntity(u64),
    DeleteEntity(u64, EntityData),
    MoveEntity(u64, Vec3, Vec3),
    RotateEntity(u64, Vec3, Vec3),
    ScaleEntity(u64, Vec3, Vec3),
    ChangeProperty(u64, String, PropertyValue, PropertyValue),
    Multiple(Vec<HistoryAction>),
}

/// Entity data for undo (simplified)
#[derive(Debug, Clone)]
pub struct EntityData {
    pub id: u64,
    pub name: String,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

/// Property value
#[derive(Debug, Clone)]
pub enum PropertyValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Vec3([f32; 3]),
    Color([f32; 4]),
}

impl EditorHistory {
    pub fn new(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size,
        }
    }

    pub fn push(&mut self, action: HistoryAction) {
        self.undo_stack.push(action);
        self.redo_stack.clear();

        // Limit size
        while self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }

    pub fn undo(&mut self) -> Option<HistoryAction> {
        if let Some(action) = self.undo_stack.pop() {
            self.redo_stack.push(action.clone());
            Some(action)
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<HistoryAction> {
        if let Some(action) = self.redo_stack.pop() {
            self.undo_stack.push(action.clone());
            Some(action)
        } else {
            None
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

// ==================== ASSET DATABASE ====================

/// Asset database
pub struct AssetDatabase {
    assets: HashMap<u64, AssetEntry>,
    paths: HashMap<String, u64>,
    next_id: u64,
}

/// Asset entry
#[derive(Debug, Clone)]
pub struct AssetEntry {
    pub id: u64,
    pub path: String,
    pub asset_type: AssetType,
    pub last_modified: u64,
    pub dependencies: Vec<u64>,
    pub is_dirty: bool,
}

/// Asset type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetType {
    Scene,
    Prefab,
    Material,
    Texture,
    Mesh,
    Animation,
    Audio,
    Script,
    Blueprint,
}

impl AssetDatabase {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            paths: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn import(&mut self, path: &str, asset_type: AssetType) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let entry = AssetEntry {
            id,
            path: path.to_string(),
            asset_type,
            last_modified: 0,
            dependencies: Vec::new(),
            is_dirty: false,
        };

        self.assets.insert(id, entry);
        self.paths.insert(path.to_string(), id);

        id
    }

    pub fn get(&self, id: u64) -> Option<&AssetEntry> {
        self.assets.get(&id)
    }

    pub fn get_by_path(&self, path: &str) -> Option<&AssetEntry> {
        self.paths.get(path).and_then(|id| self.assets.get(id))
    }

    pub fn refresh(&mut self, id: u64) {
        if let Some(entry) = self.assets.get_mut(&id) {
            entry.is_dirty = false;
            // Would reload from disk
        }
    }
}

// ==================== HOT RELOAD ====================

/// Hot reload state
#[derive(Debug, Clone, Default)]
pub struct HotReloadState {
    pub enabled: bool,
    pub auto_apply: bool,
    pub watch_paths: Vec<String>,
    pub pending_changes: Vec<PendingChange>,
    pub last_check: f64,
    pub check_interval: f64,
}

/// Pending change
#[derive(Debug, Clone)]
pub struct PendingChange {
    pub path: String,
    pub change_type: ChangeType,
    pub timestamp: u64,
}

/// Change type
#[derive(Debug, Clone, Copy)]
pub enum ChangeType {
    Created,
    Modified,
    Deleted,
}

impl HotReloadState {
    pub fn check_changes(&mut self) {
        // Would check file system for changes
        // This integrates with lunaris-runtime/hot_reload.rs
    }

    pub fn apply_pending(&mut self) {
        for change in self.pending_changes.drain(..) {
            tracing::info!("Hot reload: {:?} - {}", change.change_type, change.path);
        }
    }
}

// ==================== EDITOR TOOL ====================

/// Editor tool
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditorTool {
    #[default]
    Select,
    Move,
    Rotate,
    Scale,
    Rect,
    Paint,
    Sculpt,
    Terrain,
}

// ==================== PROJECT ====================

/// Project
#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub current_scene_path: String,
    pub settings: ProjectSettings,
}

/// Project settings
#[derive(Debug, Clone, Default)]
pub struct ProjectSettings {
    pub target_fps: u32,
    pub default_resolution: (u32, u32),
    pub company_name: String,
    pub product_name: String,
    pub version: String,
}

// ==================== FRAME STATS ====================

/// Frame statistics
#[derive(Debug, Clone)]
pub struct FrameStats {
    pub fps: f32,
    pub frame_time: f32,
    pub update_time: f32,
    pub render_time: f32,
    pub frame_count: u64,
    frame_start: std::time::Instant,
}

impl Default for FrameStats {
    fn default() -> Self {
        Self {
            fps: 0.0,
            frame_time: 0.0,
            update_time: 0.0,
            render_time: 0.0,
            frame_count: 0,
            frame_start: std::time::Instant::now(),
        }
    }
}

impl FrameStats {
    pub fn begin_frame(&mut self) {
        self.frame_start = std::time::Instant::now();
        self.frame_count += 1;
    }

    pub fn end_frame(&mut self, dt: f32) {
        self.frame_time = self.frame_start.elapsed().as_secs_f32() * 1000.0;
        self.fps = 1.0 / dt;
    }
}

// ==================== DOCK TREE EXTENSION ====================

impl DockTree {
    /// Setup default layout
    pub fn setup_default_layout(&mut self, panels: Vec<(String, u64)>) {
        // Would configure the dock tree with the given panels
        // Left 20%: Hierarchy
        // Center 60%: Viewport
        // Right 20%: Inspector
        // Bottom 25%: Console + Assets
    }

    /// Add tab to area
    pub fn add_tab(&mut self, area: String, tab: TabInfo) {
        // Would add tab to the specified dock area
    }
}
