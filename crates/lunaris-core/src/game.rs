//! Game trait for creating games in Rust
//!
//! This module provides the core `Game` trait that Rust games must implement.

use crate::input::Input;
use crate::math::Color;
use crate::time::Time;
use crate::Result;

/// Configuration for creating a game window
#[derive(Debug, Clone)]
pub struct GameConfig {
    /// Window title
    pub title: String,
    /// Window width
    pub width: u32,
    /// Window height
    pub height: u32,
    /// Target frames per second (0 = unlimited)
    pub target_fps: u32,
    /// Whether to enable VSync
    pub vsync: bool,
    /// Whether the window is resizable
    pub resizable: bool,
    /// Clear color for the screen
    pub clear_color: Color,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            title: String::from("Lunaris Game"),
            width: 1280,
            height: 720,
            target_fps: 60,
            vsync: true,
            resizable: true,
            clear_color: Color::BLACK,
        }
    }
}

impl GameConfig {
    /// Create a new game config with a title
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Set the resolution
    #[must_use]
    pub const fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set target FPS
    #[must_use]
    pub const fn with_fps(mut self, fps: u32) -> Self {
        self.target_fps = fps;
        self
    }
}

/// The main trait that all Rust games must implement
pub trait Game: Sized {
    /// Create a new instance of the game
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails
    fn new() -> Result<Self>;

    /// Get the game configuration
    fn config(&self) -> GameConfig {
        GameConfig::default()
    }

    /// Called once at the start
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    /// Called every frame to update game logic
    fn update(&mut self, time: &Time, input: &Input);

    /// Called every frame to render the game
    fn render(&mut self);

    /// Called when the game is shutting down
    fn shutdown(&mut self) {}

    /// Run the game loop (default implementation)
    ///
    /// # Errors
    ///
    /// Returns an error if the game loop fails
    fn run(mut self) -> Result<()> {
        self.init()?;

        let mut time = Time::new();
        let mut input = Input::new();

        // Simulated game loop (actual windowing handled by runtime)
        tracing::info!("Game initialized: {}", self.config().title);

        // This would be replaced by actual event loop
        loop {
            input.begin_frame();
            time.update();

            self.update(&time, &input);
            self.render();

            // For now, just run a few frames for testing
            if time.frame_count() > 10 {
                break;
            }
        }

        self.shutdown();
        Ok(())
    }
}

/// Macro to define the game entry point
#[macro_export]
macro_rules! lunaris_main {
    ($game:ty) => {
        fn main() {
            if let Err(e) = <$game as $crate::game::Game>::new().and_then(|game| game.run()) {
                eprintln!("Game error: {e}");
                std::process::exit(1);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestGame {
        frame_count: u32,
    }

    impl Game for TestGame {
        fn new() -> Result<Self> {
            Ok(Self { frame_count: 0 })
        }

        fn config(&self) -> GameConfig {
            GameConfig::new("Test Game")
        }

        fn update(&mut self, _time: &Time, _input: &Input) {
            self.frame_count += 1;
        }

        fn render(&mut self) {}
    }

    #[test]
    fn game_creation() {
        let game = TestGame::new().unwrap();
        assert_eq!(game.frame_count, 0);
        assert_eq!(game.config().title, "Test Game");
    }
}
