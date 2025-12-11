//! Window management and event handling
//!
//! Provides a cross-platform window abstraction using winit.

use lunaris_core::{input::Input, math::Vec2, Result};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalSize},
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowAttributes, WindowId},
};

/// Window configuration
#[derive(Debug, Clone)]
pub struct WindowConfig {
    /// Window title
    pub title: String,
    /// Initial width
    pub width: u32,
    /// Initial height
    pub height: u32,
    /// Is resizable
    pub resizable: bool,
    /// Use VSync
    pub vsync: bool,
    /// Fullscreen mode
    pub fullscreen: bool,
    /// Transparent window
    pub transparent: bool,
    /// Window decorations
    pub decorations: bool,
    /// Always on top
    pub always_on_top: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: String::from("Lunaris Engine"),
            width: 1280,
            height: 720,
            resizable: true,
            vsync: true,
            fullscreen: false,
            transparent: false,
            decorations: true,
            always_on_top: false,
        }
    }
}

impl WindowConfig {
    /// Create a new window config with title
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Set window size
    #[must_use]
    pub const fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set fullscreen mode
    #[must_use]
    pub const fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    /// Set resizable
    #[must_use]
    pub const fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
}

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    /// Window is not created yet
    NotCreated,
    /// Window is active and visible
    Active,
    /// Window is minimized
    Minimized,
    /// Window is maximized
    Maximized,
    /// Window is in fullscreen mode
    Fullscreen,
    /// Window is being closed
    Closing,
    /// Window is closed
    Closed,
}

/// Window events
#[derive(Debug, Clone)]
pub enum WindowEventType {
    /// Window was resized
    Resized { width: u32, height: u32 },
    /// Window was moved
    Moved { x: i32, y: i32 },
    /// Window received focus
    Focused,
    /// Window lost focus
    Unfocused,
    /// Close requested
    CloseRequested,
    /// Window scale factor changed (DPI)
    ScaleFactorChanged { scale_factor: f64 },
    /// Redraw requested
    RedrawRequested,
}

/// Window handle wrapper
pub struct Window {
    window: Option<Arc<WinitWindow>>,
    config: WindowConfig,
    state: WindowState,
    size: (u32, u32),
    position: (i32, i32),
    scale_factor: f64,
    focused: bool,
}

impl Window {
    /// Create a new window handle (window is created when event loop runs)
    #[must_use]
    pub fn new(config: WindowConfig) -> Self {
        Self {
            window: None,
            size: (config.width, config.height),
            config,
            state: WindowState::NotCreated,
            position: (0, 0),
            scale_factor: 1.0,
            focused: true,
        }
    }

    /// Get window size
    #[must_use]
    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    /// Get window position
    #[must_use]
    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    /// Get scale factor
    #[must_use]
    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    /// Check if focused
    #[must_use]
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Get window state
    #[must_use]
    pub fn state(&self) -> WindowState {
        self.state
    }

    /// Request close
    pub fn request_close(&mut self) {
        self.state = WindowState::Closing;
    }

    /// Set window title
    pub fn set_title(&self, title: &str) {
        if let Some(window) = &self.window {
            window.set_title(title);
        }
    }

    /// Set window size
    pub fn set_size(&self, width: u32, height: u32) {
        if let Some(window) = &self.window {
            let _ = window.request_inner_size(LogicalSize::new(width, height));
        }
    }

    /// Get the raw winit window
    #[must_use]
    pub fn raw(&self) -> Option<&Arc<WinitWindow>> {
        self.window.as_ref()
    }
}

/// Application trait for game/editor apps
pub trait Application: 'static {
    /// Called once when the application starts
    fn init(&mut self, window: &Window);

    /// Called every frame to update
    fn update(&mut self, input: &Input, delta_time: f32);

    /// Called every frame to render
    fn render(&mut self, window: &Window);

    /// Called when window is resized
    fn on_resize(&mut self, width: u32, height: u32);

    /// Called when close is requested, return true to close
    fn on_close_requested(&mut self) -> bool {
        true
    }

    /// Called when application is shutting down
    fn shutdown(&mut self) {}
}

/// Application runner
pub struct AppRunner<A: Application> {
    app: A,
    window: Window,
    input: Input,
    last_frame: std::time::Instant,
    should_close: bool,
}

impl<A: Application> AppRunner<A> {
    /// Create a new app runner
    pub fn new(app: A, config: WindowConfig) -> Self {
        Self {
            app,
            window: Window::new(config),
            input: Input::new(),
            last_frame: std::time::Instant::now(),
            should_close: false,
        }
    }

    /// Run the application
    pub fn run(mut self) -> Result<()> {
        let event_loop = EventLoop::new()
            .map_err(|e| lunaris_core::Error::Window(e.to_string()))?;

        event_loop.set_control_flow(ControlFlow::Poll);
        
        event_loop.run_app(&mut self)
            .map_err(|e| lunaris_core::Error::Window(e.to_string()))?;

        Ok(())
    }
}

impl<A: Application> ApplicationHandler for AppRunner<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window on resume
        let attrs = WindowAttributes::default()
            .with_title(&self.window.config.title)
            .with_inner_size(LogicalSize::new(
                self.window.config.width,
                self.window.config.height,
            ))
            .with_resizable(self.window.config.resizable)
            .with_decorations(self.window.config.decorations);

        match event_loop.create_window(attrs) {
            Ok(window) => {
                self.window.window = Some(Arc::new(window));
                self.window.state = WindowState::Active;
                self.app.init(&self.window);
                tracing::info!("Window created: {}", self.window.config.title);
            }
            Err(e) => {
                tracing::error!("Failed to create window: {}", e);
                event_loop.exit();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                if self.app.on_close_requested() {
                    self.should_close = true;
                    event_loop.exit();
                }
            }
            WindowEvent::Resized(size) => {
                self.window.size = (size.width, size.height);
                self.app.on_resize(size.width, size.height);
            }
            WindowEvent::Focused(focused) => {
                self.window.focused = focused;
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.window.scale_factor = scale_factor;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                // Convert winit key to lunaris key
                if let Some(key) = convert_key(event.physical_key) {
                    match event.state {
                        ElementState::Pressed => self.input.key_press(key),
                        ElementState::Released => self.input.key_release(key),
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(btn) = convert_mouse_button(button) {
                    match state {
                        ElementState::Pressed => self.input.mouse_press(btn),
                        ElementState::Released => self.input.mouse_release(btn),
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.input.set_mouse_position(position.x as f32, position.y as f32);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => y,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 120.0,
                };
                self.input.set_scroll_delta(scroll);
            }
            WindowEvent::RedrawRequested => {
                // Calculate delta time
                let now = std::time::Instant::now();
                let delta = now.duration_since(self.last_frame).as_secs_f32();
                self.last_frame = now;

                // Update
                self.app.update(&self.input, delta);

                // Render
                self.app.render(&self.window);

                // Clear per-frame input state
                self.input.begin_frame();

                // Request next frame
                if let Some(window) = &self.window.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // Request redraw for continuous rendering
        if let Some(window) = &self.window.window {
            window.request_redraw();
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.app.shutdown();
        self.window.state = WindowState::Closed;
        tracing::info!("Application exiting");
    }
}

/// Convert winit key to lunaris key
fn convert_key(key: winit::keyboard::PhysicalKey) -> Option<lunaris_core::input::Key> {
    use lunaris_core::input::Key;
    use winit::keyboard::{KeyCode, PhysicalKey};

    let PhysicalKey::Code(code) = key else {
        return None;
    };

    Some(match code {
        KeyCode::KeyA => Key::A,
        KeyCode::KeyB => Key::B,
        KeyCode::KeyC => Key::C,
        KeyCode::KeyD => Key::D,
        KeyCode::KeyE => Key::E,
        KeyCode::KeyF => Key::F,
        KeyCode::KeyG => Key::G,
        KeyCode::KeyH => Key::H,
        KeyCode::KeyI => Key::I,
        KeyCode::KeyJ => Key::J,
        KeyCode::KeyK => Key::K,
        KeyCode::KeyL => Key::L,
        KeyCode::KeyM => Key::M,
        KeyCode::KeyN => Key::N,
        KeyCode::KeyO => Key::O,
        KeyCode::KeyP => Key::P,
        KeyCode::KeyQ => Key::Q,
        KeyCode::KeyR => Key::R,
        KeyCode::KeyS => Key::S,
        KeyCode::KeyT => Key::T,
        KeyCode::KeyU => Key::U,
        KeyCode::KeyV => Key::V,
        KeyCode::KeyW => Key::W,
        KeyCode::KeyX => Key::X,
        KeyCode::KeyY => Key::Y,
        KeyCode::KeyZ => Key::Z,
        KeyCode::Digit0 => Key::Num0,
        KeyCode::Digit1 => Key::Num1,
        KeyCode::Digit2 => Key::Num2,
        KeyCode::Digit3 => Key::Num3,
        KeyCode::Digit4 => Key::Num4,
        KeyCode::Digit5 => Key::Num5,
        KeyCode::Digit6 => Key::Num6,
        KeyCode::Digit7 => Key::Num7,
        KeyCode::Digit8 => Key::Num8,
        KeyCode::Digit9 => Key::Num9,
        KeyCode::F1 => Key::F1,
        KeyCode::F2 => Key::F2,
        KeyCode::F3 => Key::F3,
        KeyCode::F4 => Key::F4,
        KeyCode::F5 => Key::F5,
        KeyCode::F6 => Key::F6,
        KeyCode::F7 => Key::F7,
        KeyCode::F8 => Key::F8,
        KeyCode::F9 => Key::F9,
        KeyCode::F10 => Key::F10,
        KeyCode::F11 => Key::F11,
        KeyCode::F12 => Key::F12,
        KeyCode::ArrowUp => Key::Up,
        KeyCode::ArrowDown => Key::Down,
        KeyCode::ArrowLeft => Key::Left,
        KeyCode::ArrowRight => Key::Right,
        KeyCode::Space => Key::Space,
        KeyCode::Enter => Key::Enter,
        KeyCode::Escape => Key::Escape,
        KeyCode::Tab => Key::Tab,
        KeyCode::Backspace => Key::Backspace,
        KeyCode::ShiftLeft => Key::LeftShift,
        KeyCode::ShiftRight => Key::RightShift,
        KeyCode::ControlLeft => Key::LeftCtrl,
        KeyCode::ControlRight => Key::RightCtrl,
        KeyCode::AltLeft => Key::LeftAlt,
        KeyCode::AltRight => Key::RightAlt,
        _ => return None,
    })
}

/// Convert winit mouse button to lunaris
fn convert_mouse_button(button: winit::event::MouseButton) -> Option<lunaris_core::input::MouseButton> {
    use lunaris_core::input::MouseButton;
    Some(match button {
        winit::event::MouseButton::Left => MouseButton::Left,
        winit::event::MouseButton::Right => MouseButton::Right,
        winit::event::MouseButton::Middle => MouseButton::Middle,
        winit::event::MouseButton::Back => MouseButton::Extra1,
        winit::event::MouseButton::Forward => MouseButton::Extra2,
        _ => return None,
    })
}
