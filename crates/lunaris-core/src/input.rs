//! Input handling system

use std::collections::HashSet;

/// Keyboard key codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Key {
    // Letters
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    // Numbers
    Num0, Num1, Num2, Num3, Num4,
    Num5, Num6, Num7, Num8, Num9,

    // Function keys
    F1, F2, F3, F4, F5, F6,
    F7, F8, F9, F10, F11, F12,

    // Arrow keys
    Up, Down, Left, Right,

    // Modifiers
    LeftShift, RightShift,
    LeftCtrl, RightCtrl,
    LeftAlt, RightAlt,

    // Common keys
    Space, Enter, Escape, Tab, Backspace,
    Insert, Delete, Home, End, PageUp, PageDown,
}

/// Mouse button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button (wheel click)
    Middle,
    /// Extra button (side buttons)
    Extra1,
    /// Extra button (side buttons)
    Extra2,
}

/// Input state for the current frame
#[derive(Debug, Default)]
pub struct Input {
    /// Keys currently held down
    keys_down: HashSet<Key>,
    /// Keys pressed this frame
    keys_pressed: HashSet<Key>,
    /// Keys released this frame
    keys_released: HashSet<Key>,

    /// Mouse buttons currently held down
    mouse_down: HashSet<MouseButton>,
    /// Mouse buttons pressed this frame
    mouse_pressed: HashSet<MouseButton>,
    /// Mouse buttons released this frame
    mouse_released: HashSet<MouseButton>,

    /// Mouse position in screen coordinates
    mouse_position: (f32, f32),
    /// Mouse delta since last frame
    mouse_delta: (f32, f32),
    /// Scroll wheel delta
    scroll_delta: f32,
}

impl Input {
    /// Create a new input state
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Called at the start of each frame to clear per-frame state
    pub fn begin_frame(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_pressed.clear();
        self.mouse_released.clear();
        self.mouse_delta = (0.0, 0.0);
        self.scroll_delta = 0.0;
    }

    /// Register a key press
    pub fn key_press(&mut self, key: Key) {
        if !self.keys_down.contains(&key) {
            self.keys_pressed.insert(key);
        }
        self.keys_down.insert(key);
    }

    /// Register a key release
    pub fn key_release(&mut self, key: Key) {
        self.keys_down.remove(&key);
        self.keys_released.insert(key);
    }

    /// Check if a key is currently held down
    #[must_use]
    pub fn is_key_down(&self, key: Key) -> bool {
        self.keys_down.contains(&key)
    }

    /// Check if a key was just pressed this frame
    #[must_use]
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys_pressed.contains(&key)
    }

    /// Check if a key was just released this frame
    #[must_use]
    pub fn is_key_released(&self, key: Key) -> bool {
        self.keys_released.contains(&key)
    }

    /// Register a mouse button press
    pub fn mouse_press(&mut self, button: MouseButton) {
        if !self.mouse_down.contains(&button) {
            self.mouse_pressed.insert(button);
        }
        self.mouse_down.insert(button);
    }

    /// Register a mouse button release
    pub fn mouse_release(&mut self, button: MouseButton) {
        self.mouse_down.remove(&button);
        self.mouse_released.insert(button);
    }

    /// Check if a mouse button is currently held down
    #[must_use]
    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_down.contains(&button)
    }

    /// Check if a mouse button was just pressed this frame
    #[must_use]
    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_pressed.contains(&button)
    }

    /// Check if a mouse button was just released this frame
    #[must_use]
    pub fn is_mouse_released(&self, button: MouseButton) -> bool {
        self.mouse_released.contains(&button)
    }

    /// Set mouse position
    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        let old = self.mouse_position;
        self.mouse_position = (x, y);
        self.mouse_delta = (x - old.0, y - old.1);
    }

    /// Get current mouse position
    #[must_use]
    pub fn mouse_position(&self) -> (f32, f32) {
        self.mouse_position
    }

    /// Get mouse movement delta
    #[must_use]
    pub fn mouse_delta(&self) -> (f32, f32) {
        self.mouse_delta
    }

    /// Set scroll wheel delta
    pub fn set_scroll_delta(&mut self, delta: f32) {
        self.scroll_delta = delta;
    }

    /// Get scroll wheel delta
    #[must_use]
    pub fn scroll_delta(&self) -> f32 {
        self.scroll_delta
    }

    /// Get horizontal axis input (-1 to 1) from arrow keys or WASD
    #[must_use]
    pub fn get_axis_horizontal(&self) -> f32 {
        let mut value = 0.0;
        if self.is_key_down(Key::Left) || self.is_key_down(Key::A) {
            value -= 1.0;
        }
        if self.is_key_down(Key::Right) || self.is_key_down(Key::D) {
            value += 1.0;
        }
        value
    }

    /// Get vertical axis input (-1 to 1) from arrow keys or WASD
    #[must_use]
    pub fn get_axis_vertical(&self) -> f32 {
        let mut value = 0.0;
        if self.is_key_down(Key::Down) || self.is_key_down(Key::S) {
            value -= 1.0;
        }
        if self.is_key_down(Key::Up) || self.is_key_down(Key::W) {
            value += 1.0;
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_press_detection() {
        let mut input = Input::new();
        
        input.key_press(Key::Space);
        assert!(input.is_key_down(Key::Space));
        assert!(input.is_key_pressed(Key::Space));
        
        input.begin_frame();
        assert!(input.is_key_down(Key::Space));
        assert!(!input.is_key_pressed(Key::Space));
        
        input.key_release(Key::Space);
        assert!(!input.is_key_down(Key::Space));
        assert!(input.is_key_released(Key::Space));
    }

    #[test]
    fn axis_input() {
        let mut input = Input::new();
        
        input.key_press(Key::D);
        assert!((input.get_axis_horizontal() - 1.0).abs() < f32::EPSILON);
        
        input.key_press(Key::A);
        assert!((input.get_axis_horizontal()).abs() < f32::EPSILON);
    }
}
