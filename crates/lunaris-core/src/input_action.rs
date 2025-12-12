//! Input Handling System
//!
//! Advanced input management with action mapping and rebinding.

use crate::input::{Key, MouseButton};
use std::collections::HashMap;

/// Input action
#[derive(Debug, Clone)]
pub struct InputAction {
    /// Action name
    pub name: String,
    /// Primary binding
    pub primary: InputBinding,
    /// Secondary binding
    pub secondary: Option<InputBinding>,
    /// Is pressed
    pressed: bool,
    /// Just pressed this frame
    just_pressed: bool,
    /// Just released this frame
    just_released: bool,
    /// Axis value (-1 to 1)
    value: f32,
}

/// Input binding
#[derive(Debug, Clone)]
pub enum InputBinding {
    /// Keyboard key
    Key(Key),
    /// Mouse button
    Mouse(MouseButton),
    /// Gamepad button
    GamepadButton(u32),
    /// Gamepad axis
    GamepadAxis(u32, bool), // axis index, positive direction
    /// Mouse axis
    MouseAxis(MouseAxis),
    /// Composite (two keys for axis)
    Composite(Key, Key), // negative, positive
}

/// Mouse axis
#[derive(Debug, Clone, Copy)]
pub enum MouseAxis {
    /// X movement
    X,
    /// Y movement
    Y,
    /// Scroll wheel
    Scroll,
}

impl InputAction {
    /// Create a new action
    #[must_use]
    pub fn new(name: impl Into<String>, primary: InputBinding) -> Self {
        Self {
            name: name.into(),
            primary,
            secondary: None,
            pressed: false,
            just_pressed: false,
            just_released: false,
            value: 0.0,
        }
    }

    /// With secondary binding
    #[must_use]
    pub fn with_secondary(mut self, binding: InputBinding) -> Self {
        self.secondary = Some(binding);
        self
    }

    /// Is action pressed
    #[must_use]
    pub fn is_pressed(&self) -> bool {
        self.pressed
    }

    /// Was action just pressed
    #[must_use]
    pub fn is_just_pressed(&self) -> bool {
        self.just_pressed
    }

    /// Was action just released
    #[must_use]
    pub fn is_just_released(&self) -> bool {
        self.just_released
    }

    /// Get axis value
    #[must_use]
    pub fn value(&self) -> f32 {
        self.value
    }
}

/// Input map (action mappings)
pub struct InputMap {
    /// Actions by name
    actions: HashMap<String, InputAction>,
    /// Key to actions mapping
    key_map: HashMap<Key, Vec<String>>,
    /// Mouse button to actions
    mouse_map: HashMap<MouseButton, Vec<String>>,
}

impl Default for InputMap {
    fn default() -> Self {
        Self::new()
    }
}

impl InputMap {
    /// Create a new input map
    #[must_use]
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
            key_map: HashMap::new(),
            mouse_map: HashMap::new(),
        }
    }

    /// Add an action
    pub fn add_action(&mut self, action: InputAction) {
        let name = action.name.clone();

        // Map primary binding
        match &action.primary {
            InputBinding::Key(key) => {
                self.key_map.entry(*key).or_default().push(name.clone());
            }
            InputBinding::Mouse(button) => {
                self.mouse_map.entry(*button).or_default().push(name.clone());
            }
            InputBinding::Composite(neg, pos) => {
                self.key_map.entry(*neg).or_default().push(name.clone());
                self.key_map.entry(*pos).or_default().push(name.clone());
            }
            _ => {}
        }

        // Map secondary binding
        if let Some(ref secondary) = action.secondary {
            match secondary {
                InputBinding::Key(key) => {
                    self.key_map.entry(*key).or_default().push(name.clone());
                }
                InputBinding::Mouse(button) => {
                    self.mouse_map.entry(*button).or_default().push(name.clone());
                }
                _ => {}
            }
        }

        self.actions.insert(name, action);
    }

    /// Get action
    #[must_use]
    pub fn action(&self, name: &str) -> Option<&InputAction> {
        self.actions.get(name)
    }

    /// Check if action is pressed
    #[must_use]
    pub fn is_pressed(&self, name: &str) -> bool {
        self.actions.get(name).map_or(false, |a| a.is_pressed())
    }

    /// Check if action was just pressed
    #[must_use]
    pub fn is_just_pressed(&self, name: &str) -> bool {
        self.actions.get(name).map_or(false, |a| a.is_just_pressed())
    }

    /// Get action value
    #[must_use]
    pub fn value(&self, name: &str) -> f32 {
        self.actions.get(name).map_or(0.0, |a| a.value())
    }

    /// Begin frame (reset just_pressed/released)
    pub fn begin_frame(&mut self) {
        for action in self.actions.values_mut() {
            action.just_pressed = false;
            action.just_released = false;
        }
    }

    /// Process key press
    pub fn key_pressed(&mut self, key: Key) {
        if let Some(action_names) = self.key_map.get(&key) {
            for name in action_names.clone() {
                if let Some(action) = self.actions.get_mut(&name) {
                    if !action.pressed {
                        action.just_pressed = true;
                    }
                    action.pressed = true;
                    action.value = 1.0;
                }
            }
        }
    }

    /// Process key release
    pub fn key_released(&mut self, key: Key) {
        if let Some(action_names) = self.key_map.get(&key) {
            for name in action_names.clone() {
                if let Some(action) = self.actions.get_mut(&name) {
                    if action.pressed {
                        action.just_released = true;
                    }
                    action.pressed = false;
                    action.value = 0.0;
                }
            }
        }
    }

    /// Process mouse button press
    pub fn mouse_pressed(&mut self, button: MouseButton) {
        if let Some(action_names) = self.mouse_map.get(&button) {
            for name in action_names.clone() {
                if let Some(action) = self.actions.get_mut(&name) {
                    if !action.pressed {
                        action.just_pressed = true;
                    }
                    action.pressed = true;
                    action.value = 1.0;
                }
            }
        }
    }

    /// Process mouse button release
    pub fn mouse_released(&mut self, button: MouseButton) {
        if let Some(action_names) = self.mouse_map.get(&button) {
            for name in action_names.clone() {
                if let Some(action) = self.actions.get_mut(&name) {
                    if action.pressed {
                        action.just_released = true;
                    }
                    action.pressed = false;
                    action.value = 0.0;
                }
            }
        }
    }

    /// Create default FPS controls
    #[must_use]
    pub fn default_fps() -> Self {
        let mut map = Self::new();

        map.add_action(InputAction::new("move_forward", InputBinding::Key(Key::W))
            .with_secondary(InputBinding::Key(Key::Up)));
        map.add_action(InputAction::new("move_back", InputBinding::Key(Key::S))
            .with_secondary(InputBinding::Key(Key::Down)));
        map.add_action(InputAction::new("move_left", InputBinding::Key(Key::A))
            .with_secondary(InputBinding::Key(Key::Left)));
        map.add_action(InputAction::new("move_right", InputBinding::Key(Key::D))
            .with_secondary(InputBinding::Key(Key::Right)));
        map.add_action(InputAction::new("jump", InputBinding::Key(Key::Space)));
        map.add_action(InputAction::new("crouch", InputBinding::Key(Key::LeftCtrl)));
        map.add_action(InputAction::new("sprint", InputBinding::Key(Key::LeftShift)));
        map.add_action(InputAction::new("fire", InputBinding::Mouse(MouseButton::Left)));
        map.add_action(InputAction::new("aim", InputBinding::Mouse(MouseButton::Right)));
        map.add_action(InputAction::new("reload", InputBinding::Key(Key::R)));
        map.add_action(InputAction::new("interact", InputBinding::Key(Key::E)));

        map
    }

    /// Create default platformer controls
    #[must_use]
    pub fn default_platformer() -> Self {
        let mut map = Self::new();

        map.add_action(InputAction::new("move_left", InputBinding::Key(Key::A))
            .with_secondary(InputBinding::Key(Key::Left)));
        map.add_action(InputAction::new("move_right", InputBinding::Key(Key::D))
            .with_secondary(InputBinding::Key(Key::Right)));
        map.add_action(InputAction::new("jump", InputBinding::Key(Key::Space))
            .with_secondary(InputBinding::Key(Key::W)));
        map.add_action(InputAction::new("crouch", InputBinding::Key(Key::S))
            .with_secondary(InputBinding::Key(Key::Down)));
        map.add_action(InputAction::new("attack", InputBinding::Key(Key::J))
            .with_secondary(InputBinding::Mouse(MouseButton::Left)));
        map.add_action(InputAction::new("special", InputBinding::Key(Key::K)));
        map.add_action(InputAction::new("dash", InputBinding::Key(Key::LeftShift)));

        map
    }
}

/// Input rebinding
pub struct InputRebinder {
    /// Currently rebinding action
    rebinding: Option<String>,
    /// Is waiting for input
    waiting: bool,
}

impl Default for InputRebinder {
    fn default() -> Self {
        Self::new()
    }
}

impl InputRebinder {
    /// Create a new rebinder
    #[must_use]
    pub fn new() -> Self {
        Self {
            rebinding: None,
            waiting: false,
        }
    }

    /// Start rebinding an action
    pub fn start_rebind(&mut self, action_name: &str) {
        self.rebinding = Some(action_name.to_string());
        self.waiting = true;
    }

    /// Cancel rebinding
    pub fn cancel(&mut self) {
        self.rebinding = None;
        self.waiting = false;
    }

    /// Is waiting for input
    #[must_use]
    pub fn is_waiting(&self) -> bool {
        self.waiting
    }

    /// Get action being rebound
    #[must_use]
    pub fn rebinding_action(&self) -> Option<&str> {
        self.rebinding.as_deref()
    }
}
