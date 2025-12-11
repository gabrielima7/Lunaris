//! Example: Simple Pong game in Rust
//!
//! Demonstrates how to create a game using the Lunaris Engine with Rust.

use lunaris_core::{
    game::{Game, GameConfig},
    input::{Input, Key},
    math::{Color, Rect, Vec2},
    time::Time,
    Result,
};

/// Paddle entity
struct Paddle {
    position: Vec2,
    size: Vec2,
    speed: f32,
}

impl Paddle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            size: Vec2::new(20.0, 100.0),
            speed: 400.0,
        }
    }

    fn rect(&self) -> Rect {
        Rect::new(
            self.position.x - self.size.x / 2.0,
            self.position.y - self.size.y / 2.0,
            self.size.x,
            self.size.y,
        )
    }
}

/// Ball entity
struct Ball {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
}

impl Ball {
    fn new() -> Self {
        Self {
            position: Vec2::new(640.0, 360.0),
            velocity: Vec2::new(300.0, 200.0),
            radius: 10.0,
        }
    }

    fn reset(&mut self) {
        self.position = Vec2::new(640.0, 360.0);
        self.velocity = Vec2::new(
            if self.velocity.x > 0.0 { -300.0 } else { 300.0 },
            200.0,
        );
    }
}

/// The main Pong game
pub struct PongGame {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    left_score: u32,
    right_score: u32,
}

impl Game for PongGame {
    fn new() -> Result<Self> {
        Ok(Self {
            left_paddle: Paddle::new(50.0, 360.0),
            right_paddle: Paddle::new(1230.0, 360.0),
            ball: Ball::new(),
            left_score: 0,
            right_score: 0,
        })
    }

    fn config(&self) -> GameConfig {
        GameConfig::new("Pong - Lunaris Engine")
            .with_size(1280, 720)
            .with_fps(60)
    }

    fn update(&mut self, time: &Time, input: &Input) {
        let dt = time.delta_seconds();

        // Left paddle (W/S keys)
        if input.is_key_down(Key::W) {
            self.left_paddle.position.y -= self.left_paddle.speed * dt;
        }
        if input.is_key_down(Key::S) {
            self.left_paddle.position.y += self.left_paddle.speed * dt;
        }

        // Right paddle (Up/Down arrows)
        if input.is_key_down(Key::Up) {
            self.right_paddle.position.y -= self.right_paddle.speed * dt;
        }
        if input.is_key_down(Key::Down) {
            self.right_paddle.position.y += self.right_paddle.speed * dt;
        }

        // Clamp paddles to screen
        let half_height = self.left_paddle.size.y / 2.0;
        self.left_paddle.position.y = self.left_paddle.position.y.clamp(half_height, 720.0 - half_height);
        self.right_paddle.position.y = self.right_paddle.position.y.clamp(half_height, 720.0 - half_height);

        // Move ball
        self.ball.position += self.ball.velocity * dt;

        // Ball collision with top/bottom
        if self.ball.position.y <= self.ball.radius || self.ball.position.y >= 720.0 - self.ball.radius {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        // Ball collision with paddles
        let ball_rect = Rect::new(
            self.ball.position.x - self.ball.radius,
            self.ball.position.y - self.ball.radius,
            self.ball.radius * 2.0,
            self.ball.radius * 2.0,
        );

        if ball_rect.overlaps(self.left_paddle.rect()) && self.ball.velocity.x < 0.0 {
            self.ball.velocity.x = -self.ball.velocity.x * 1.1;
            self.ball.position.x = self.left_paddle.position.x + self.left_paddle.size.x / 2.0 + self.ball.radius;
        }

        if ball_rect.overlaps(self.right_paddle.rect()) && self.ball.velocity.x > 0.0 {
            self.ball.velocity.x = -self.ball.velocity.x * 1.1;
            self.ball.position.x = self.right_paddle.position.x - self.right_paddle.size.x / 2.0 - self.ball.radius;
        }

        // Scoring
        if self.ball.position.x <= 0.0 {
            self.right_score += 1;
            self.ball.reset();
            tracing::info!("Score: {} - {}", self.left_score, self.right_score);
        }
        if self.ball.position.x >= 1280.0 {
            self.left_score += 1;
            self.ball.reset();
            tracing::info!("Score: {} - {}", self.left_score, self.right_score);
        }
    }

    fn render(&mut self) {
        // In a real implementation, this would use the renderer
        // For now, just log the game state periodically
    }
}

// Entry point macro usage example:
// lunaris_main!(PongGame);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pong_game_creates() {
        let game = PongGame::new().unwrap();
        assert_eq!(game.left_score, 0);
        assert_eq!(game.right_score, 0);
    }

    #[test]
    fn ball_bounces() {
        let mut game = PongGame::new().unwrap();
        let time = Time::new();
        let input = Input::new();

        // Simulate a few frames
        for _ in 0..10 {
            game.update(&time, &input);
        }
    }
}
