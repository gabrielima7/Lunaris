//! Pong Example
//!
//! Classic Pong game demonstrating 2D rendering and physics.
//!
//! Run with: cargo run --example pong

use glam::Vec2;

/// Paddle constants
const PADDLE_WIDTH: f32 = 15.0;
const PADDLE_HEIGHT: f32 = 80.0;
const PADDLE_SPEED: f32 = 400.0;
const PADDLE_MARGIN: f32 = 30.0;

/// Ball constants
const BALL_SIZE: f32 = 15.0;
const BALL_SPEED: f32 = 350.0;

/// Screen dimensions
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

/// Paddle
#[derive(Debug, Clone)]
pub struct Paddle {
    pub position: Vec2,
    pub score: u32,
}

impl Paddle {
    pub fn new(x: f32) -> Self {
        Self {
            position: Vec2::new(x, SCREEN_HEIGHT / 2.0),
            score: 0,
        }
    }

    pub fn top(&self) -> f32 {
        self.position.y - PADDLE_HEIGHT / 2.0
    }

    pub fn bottom(&self) -> f32 {
        self.position.y + PADDLE_HEIGHT / 2.0
    }

    pub fn left(&self) -> f32 {
        self.position.x - PADDLE_WIDTH / 2.0
    }

    pub fn right(&self) -> f32 {
        self.position.x + PADDLE_WIDTH / 2.0
    }
}

/// Ball
#[derive(Debug, Clone)]
pub struct Ball {
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            velocity: Vec2::new(BALL_SPEED, BALL_SPEED * 0.5),
        }
    }

    pub fn reset(&mut self, direction: f32) {
        self.position = Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
        self.velocity = Vec2::new(BALL_SPEED * direction, BALL_SPEED * 0.5);
    }
}

/// Pong game
pub struct Pong {
    pub player1: Paddle,
    pub player2: Paddle,
    pub ball: Ball,
    pub game_over: bool,
    pub winner: Option<u8>,
}

impl Default for Pong {
    fn default() -> Self {
        Self::new()
    }
}

impl Pong {
    const WINNING_SCORE: u32 = 11;

    pub fn new() -> Self {
        Self {
            player1: Paddle::new(PADDLE_MARGIN),
            player2: Paddle::new(SCREEN_WIDTH - PADDLE_MARGIN),
            ball: Ball::new(),
            game_over: false,
            winner: None,
        }
    }

    /// Update game state
    pub fn update(&mut self, dt: f32, p1_input: f32, p2_input: f32) {
        if self.game_over {
            return;
        }

        // Move paddles
        self.player1.position.y += p1_input * PADDLE_SPEED * dt;
        self.player2.position.y += p2_input * PADDLE_SPEED * dt;

        // Clamp paddle positions
        let half_height = PADDLE_HEIGHT / 2.0;
        self.player1.position.y = self.player1.position.y.clamp(half_height, SCREEN_HEIGHT - half_height);
        self.player2.position.y = self.player2.position.y.clamp(half_height, SCREEN_HEIGHT - half_height);

        // Move ball
        self.ball.position += self.ball.velocity * dt;

        // Ball collision with top/bottom walls
        if self.ball.position.y <= BALL_SIZE / 2.0 {
            self.ball.position.y = BALL_SIZE / 2.0;
            self.ball.velocity.y = -self.ball.velocity.y;
        }
        if self.ball.position.y >= SCREEN_HEIGHT - BALL_SIZE / 2.0 {
            self.ball.position.y = SCREEN_HEIGHT - BALL_SIZE / 2.0;
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        // Ball collision with paddles
        self.check_paddle_collision(&mut self.player1.clone(), 1.0);
        self.check_paddle_collision(&mut self.player2.clone(), -1.0);

        // Scoring
        if self.ball.position.x <= 0.0 {
            self.player2.score += 1;
            self.ball.reset(-1.0);
            self.check_winner();
        }
        if self.ball.position.x >= SCREEN_WIDTH {
            self.player1.score += 1;
            self.ball.reset(1.0);
            self.check_winner();
        }
    }

    fn check_paddle_collision(&mut self, paddle: &Paddle, direction: f32) {
        let ball_left = self.ball.position.x - BALL_SIZE / 2.0;
        let ball_right = self.ball.position.x + BALL_SIZE / 2.0;
        let ball_top = self.ball.position.y - BALL_SIZE / 2.0;
        let ball_bottom = self.ball.position.y + BALL_SIZE / 2.0;

        if ball_right >= paddle.left() && ball_left <= paddle.right() &&
           ball_bottom >= paddle.top() && ball_top <= paddle.bottom() {
            // Reverse horizontal velocity
            self.ball.velocity.x = -self.ball.velocity.x;
            
            // Add spin based on where it hit the paddle
            let hit_pos = (self.ball.position.y - paddle.position.y) / (PADDLE_HEIGHT / 2.0);
            self.ball.velocity.y = hit_pos * BALL_SPEED;
            
            // Speed up slightly
            self.ball.velocity *= 1.05;
            
            // Push ball out of paddle
            if direction > 0.0 {
                self.ball.position.x = paddle.right() + BALL_SIZE / 2.0;
            } else {
                self.ball.position.x = paddle.left() - BALL_SIZE / 2.0;
            }
        }
    }

    fn check_winner(&mut self) {
        if self.player1.score >= Self::WINNING_SCORE {
            self.game_over = true;
            self.winner = Some(1);
        } else if self.player2.score >= Self::WINNING_SCORE {
            self.game_over = true;
            self.winner = Some(2);
        }
    }

    /// Reset game
    pub fn reset(&mut self) {
        self.player1 = Paddle::new(PADDLE_MARGIN);
        self.player2 = Paddle::new(SCREEN_WIDTH - PADDLE_MARGIN);
        self.ball = Ball::new();
        self.game_over = false;
        self.winner = None;
    }
}

fn main() {
    println!("====================");
    println!("  🏓 PONG - Lunaris");
    println!("====================");
    println!();

    let mut game = Pong::new();
    
    // Simulate some frames
    for frame in 0..300 {
        let dt = 1.0 / 60.0;
        
        // Simple AI: paddles follow ball
        let p1_input = (game.ball.position.y - game.player1.position.y).signum();
        let p2_input = (game.ball.position.y - game.player2.position.y).signum();
        
        game.update(dt, p1_input, p2_input);
        
        if frame % 60 == 0 {
            println!("Frame {}: P1={} vs P2={} | Ball: ({:.0}, {:.0})",
                frame,
                game.player1.score,
                game.player2.score,
                game.ball.position.x,
                game.ball.position.y
            );
        }
        
        if game.game_over {
            println!();
            println!("🎉 Game Over! Player {} wins!", game.winner.unwrap());
            break;
        }
    }
    
    println!();
    println!("Final Score: Player 1: {} - Player 2: {}", 
        game.player1.score, game.player2.score);
}
