//! Keyboard Shortcuts System
//!
//! Complete keyboard shortcut management with customization.

use std::collections::HashMap;

/// Key code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    // Letters
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    // Numbers
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    // Navigation
    Up, Down, Left, Right, Home, End, PageUp, PageDown,
    // Editing
    Backspace, Delete, Insert, Enter, Tab, Space,
    // Modifiers (shouldn't be used alone)
    Ctrl, Shift, Alt, Super,
    // Other
    Escape, Tilde, Minus, Equal, BracketLeft, BracketRight,
    Semicolon, Quote, Comma, Period, Slash, Backslash,
}

/// Key modifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub super_: bool,
}

impl Modifiers {
    pub const NONE: Self = Self { ctrl: false, shift: false, alt: false, super_: false };
    pub const CTRL: Self = Self { ctrl: true, shift: false, alt: false, super_: false };
    pub const SHIFT: Self = Self { ctrl: false, shift: true, alt: false, super_: false };
    pub const ALT: Self = Self { ctrl: false, shift: false, alt: true, super_: false };
    pub const CTRL_SHIFT: Self = Self { ctrl: true, shift: true, alt: false, super_: false };
    pub const CTRL_ALT: Self = Self { ctrl: true, shift: false, alt: true, super_: false };
}

/// Keyboard shortcut
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shortcut {
    pub key: KeyCode,
    pub modifiers: Modifiers,
}

impl Shortcut {
    pub fn new(key: KeyCode, modifiers: Modifiers) -> Self {
        Self { key, modifiers }
    }

    /// Format as string (e.g., "Ctrl+S")
    pub fn display(&self) -> String {
        let mut parts = Vec::new();

        if self.modifiers.ctrl { parts.push("Ctrl"); }
        if self.modifiers.alt { parts.push("Alt"); }
        if self.modifiers.shift { parts.push("Shift"); }
        if self.modifiers.super_ { parts.push("Super"); }

        let key_str = format!("{:?}", self.key);
        parts.push(&key_str);

        parts.join("+")
    }
}

/// Shortcut action
#[derive(Debug, Clone)]
pub struct ShortcutAction {
    /// Action identifier
    pub action_id: String,
    /// Display name
    pub name: String,
    /// Category
    pub category: String,
    /// Description
    pub description: String,
    /// Default shortcut
    pub default_shortcut: Option<Shortcut>,
    /// Current shortcut
    pub shortcut: Option<Shortcut>,
    /// Is global (works when any window focused)
    pub global: bool,
    /// Requires selected entity
    pub requires_selection: bool,
}

/// Shortcut manager
pub struct ShortcutManager {
    /// All actions
    actions: HashMap<String, ShortcutAction>,
    /// Shortcut to action lookup
    shortcut_map: HashMap<Shortcut, String>,
    /// Context stack (for context-sensitive shortcuts)
    context_stack: Vec<String>,
}

impl ShortcutManager {
    /// Create new shortcut manager with defaults
    pub fn new() -> Self {
        let mut manager = Self {
            actions: HashMap::new(),
            shortcut_map: HashMap::new(),
            context_stack: vec!["global".to_string()],
        };
        manager.register_defaults();
        manager.rebuild_map();
        manager
    }

    fn register_defaults(&mut self) {
        // File operations
        self.register(ShortcutAction {
            action_id: "file.new".to_string(),
            name: "New Scene".to_string(),
            category: "File".to_string(),
            description: "Create a new empty scene".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::N, Modifiers::CTRL)),
            shortcut: Some(Shortcut::new(KeyCode::N, Modifiers::CTRL)),
            global: true,
            requires_selection: false,
        });

        self.register(ShortcutAction {
            action_id: "file.open".to_string(),
            name: "Open Scene".to_string(),
            category: "File".to_string(),
            description: "Open an existing scene".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::O, Modifiers::CTRL)),
            shortcut: Some(Shortcut::new(KeyCode::O, Modifiers::CTRL)),
            global: true,
            requires_selection: false,
        });

        self.register(ShortcutAction {
            action_id: "file.save".to_string(),
            name: "Save Scene".to_string(),
            category: "File".to_string(),
            description: "Save the current scene".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::S, Modifiers::CTRL)),
            shortcut: Some(Shortcut::new(KeyCode::S, Modifiers::CTRL)),
            global: true,
            requires_selection: false,
        });

        self.register(ShortcutAction {
            action_id: "file.save_as".to_string(),
            name: "Save Scene As...".to_string(),
            category: "File".to_string(),
            description: "Save the current scene with a new name".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::S, Modifiers::CTRL_SHIFT)),
            shortcut: Some(Shortcut::new(KeyCode::S, Modifiers::CTRL_SHIFT)),
            global: true,
            requires_selection: false,
        });

        // Edit operations
        self.register(ShortcutAction {
            action_id: "edit.undo".to_string(),
            name: "Undo".to_string(),
            category: "Edit".to_string(),
            description: "Undo the last action".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::Z, Modifiers::CTRL)),
            shortcut: Some(Shortcut::new(KeyCode::Z, Modifiers::CTRL)),
            global: true,
            requires_selection: false,
        });

        self.register(ShortcutAction {
            action_id: "edit.redo".to_string(),
            name: "Redo".to_string(),
            category: "Edit".to_string(),
            description: "Redo the last undone action".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::Y, Modifiers::CTRL)),
            shortcut: Some(Shortcut::new(KeyCode::Y, Modifiers::CTRL)),
            global: true,
            requires_selection: false,
        });

        self.register(ShortcutAction {
            action_id: "edit.cut".to_string(),
            name: "Cut".to_string(),
            category: "Edit".to_string(),
            description: "Cut selection".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::X, Modifiers::CTRL)),
            shortcut: Some(Shortcut::new(KeyCode::X, Modifiers::CTRL)),
            global: true,
            requires_selection: true,
        });

        self.register(ShortcutAction {
            action_id: "edit.copy".to_string(),
            name: "Copy".to_string(),
            category: "Edit".to_string(),
            description: "Copy selection".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::C, Modifiers::CTRL)),
            shortcut: Some(Shortcut::new(KeyCode::C, Modifiers::CTRL)),
            global: true,
            requires_selection: true,
        });

        self.register(ShortcutAction {
            action_id: "edit.paste".to_string(),
            name: "Paste".to_string(),
            category: "Edit".to_string(),
            description: "Paste from clipboard".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::V, Modifiers::CTRL)),
            shortcut: Some(Shortcut::new(KeyCode::V, Modifiers::CTRL)),
            global: true,
            requires_selection: false,
        });

        self.register(ShortcutAction {
            action_id: "edit.duplicate".to_string(),
            name: "Duplicate".to_string(),
            category: "Edit".to_string(),
            description: "Duplicate selection".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::D, Modifiers::CTRL)),
            shortcut: Some(Shortcut::new(KeyCode::D, Modifiers::CTRL)),
            global: true,
            requires_selection: true,
        });

        self.register(ShortcutAction {
            action_id: "edit.delete".to_string(),
            name: "Delete".to_string(),
            category: "Edit".to_string(),
            description: "Delete selection".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::Delete, Modifiers::NONE)),
            shortcut: Some(Shortcut::new(KeyCode::Delete, Modifiers::NONE)),
            global: true,
            requires_selection: true,
        });

        self.register(ShortcutAction {
            action_id: "edit.select_all".to_string(),
            name: "Select All".to_string(),
            category: "Edit".to_string(),
            description: "Select all objects".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::A, Modifiers::CTRL)),
            shortcut: Some(Shortcut::new(KeyCode::A, Modifiers::CTRL)),
            global: true,
            requires_selection: false,
        });

        // Transform tools
        self.register(ShortcutAction {
            action_id: "tool.move".to_string(),
            name: "Move Tool".to_string(),
            category: "Tools".to_string(),
            description: "Activate move tool".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::W, Modifiers::NONE)),
            shortcut: Some(Shortcut::new(KeyCode::W, Modifiers::NONE)),
            global: false,
            requires_selection: false,
        });

        self.register(ShortcutAction {
            action_id: "tool.rotate".to_string(),
            name: "Rotate Tool".to_string(),
            category: "Tools".to_string(),
            description: "Activate rotate tool".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::E, Modifiers::NONE)),
            shortcut: Some(Shortcut::new(KeyCode::E, Modifiers::NONE)),
            global: false,
            requires_selection: false,
        });

        self.register(ShortcutAction {
            action_id: "tool.scale".to_string(),
            name: "Scale Tool".to_string(),
            category: "Tools".to_string(),
            description: "Activate scale tool".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::R, Modifiers::NONE)),
            shortcut: Some(Shortcut::new(KeyCode::R, Modifiers::NONE)),
            global: false,
            requires_selection: false,
        });

        // View operations
        self.register(ShortcutAction {
            action_id: "view.frame".to_string(),
            name: "Frame Selected".to_string(),
            category: "View".to_string(),
            description: "Focus camera on selection".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::F, Modifiers::NONE)),
            shortcut: Some(Shortcut::new(KeyCode::F, Modifiers::NONE)),
            global: false,
            requires_selection: true,
        });

        self.register(ShortcutAction {
            action_id: "view.frame_all".to_string(),
            name: "Frame All".to_string(),
            category: "View".to_string(),
            description: "Focus camera on all objects".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::Home, Modifiers::NONE)),
            shortcut: Some(Shortcut::new(KeyCode::Home, Modifiers::NONE)),
            global: false,
            requires_selection: false,
        });

        self.register(ShortcutAction {
            action_id: "view.toggle_grid".to_string(),
            name: "Toggle Grid".to_string(),
            category: "View".to_string(),
            description: "Show/hide grid".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::G, Modifiers::NONE)),
            shortcut: Some(Shortcut::new(KeyCode::G, Modifiers::NONE)),
            global: false,
            requires_selection: false,
        });

        // Play controls
        self.register(ShortcutAction {
            action_id: "play.toggle".to_string(),
            name: "Play/Pause".to_string(),
            category: "Play".to_string(),
            description: "Toggle play mode".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::Space, Modifiers::NONE)),
            shortcut: Some(Shortcut::new(KeyCode::Space, Modifiers::NONE)),
            global: true,
            requires_selection: false,
        });

        self.register(ShortcutAction {
            action_id: "play.stop".to_string(),
            name: "Stop".to_string(),
            category: "Play".to_string(),
            description: "Stop play mode".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::Escape, Modifiers::NONE)),
            shortcut: Some(Shortcut::new(KeyCode::Escape, Modifiers::NONE)),
            global: true,
            requires_selection: false,
        });

        // Window operations
        self.register(ShortcutAction {
            action_id: "window.console".to_string(),
            name: "Toggle Console".to_string(),
            category: "Window".to_string(),
            description: "Show/hide console".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::Tilde, Modifiers::CTRL)),
            shortcut: Some(Shortcut::new(KeyCode::Tilde, Modifiers::CTRL)),
            global: true,
            requires_selection: false,
        });

        self.register(ShortcutAction {
            action_id: "window.fullscreen".to_string(),
            name: "Toggle Fullscreen".to_string(),
            category: "Window".to_string(),
            description: "Toggle fullscreen mode".to_string(),
            default_shortcut: Some(Shortcut::new(KeyCode::F11, Modifiers::NONE)),
            shortcut: Some(Shortcut::new(KeyCode::F11, Modifiers::NONE)),
            global: true,
            requires_selection: false,
        });
    }

    /// Register action
    pub fn register(&mut self, action: ShortcutAction) {
        self.actions.insert(action.action_id.clone(), action);
    }

    /// Rebuild shortcut lookup map
    pub fn rebuild_map(&mut self) {
        self.shortcut_map.clear();
        for (id, action) in &self.actions {
            if let Some(shortcut) = &action.shortcut {
                self.shortcut_map.insert(shortcut.clone(), id.clone());
            }
        }
    }

    /// Find action for shortcut
    pub fn find_action(&self, shortcut: &Shortcut) -> Option<&ShortcutAction> {
        self.shortcut_map.get(shortcut)
            .and_then(|id| self.actions.get(id))
    }

    /// Get all actions
    pub fn all_actions(&self) -> Vec<&ShortcutAction> {
        self.actions.values().collect()
    }

    /// Get actions by category
    pub fn by_category(&self, category: &str) -> Vec<&ShortcutAction> {
        self.actions.values()
            .filter(|a| a.category == category)
            .collect()
    }

    /// Get categories
    pub fn categories(&self) -> Vec<String> {
        let mut cats: Vec<_> = self.actions.values()
            .map(|a| a.category.clone())
            .collect();
        cats.sort();
        cats.dedup();
        cats
    }

    /// Set shortcut for action
    pub fn set_shortcut(&mut self, action_id: &str, shortcut: Option<Shortcut>) {
        if let Some(action) = self.actions.get_mut(action_id) {
            action.shortcut = shortcut;
        }
        self.rebuild_map();
    }

    /// Reset to defaults
    pub fn reset_to_defaults(&mut self) {
        for action in self.actions.values_mut() {
            action.shortcut = action.default_shortcut.clone();
        }
        self.rebuild_map();
    }
}

impl Default for ShortcutManager {
    fn default() -> Self {
        Self::new()
    }
}
