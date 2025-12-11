//! Complete example application demonstrating the integrated engine

use lunaris_core::{
    input::{Input, Key, MouseButton},
    math::{Color, Vec2},
    time::Time,
};

/// Example game state
pub struct ExampleGame {
    /// Player position
    player_pos: Vec2,
    /// Player velocity
    player_vel: Vec2,
    /// Player speed
    speed: f32,
    /// Camera position
    camera_pos: Vec2,
    /// Background color
    clear_color: Color,
    /// Score
    score: u32,
    /// Entities
    entities: Vec<GameEntity>,
    /// Frame count
    frame: u64,
}

/// Simple game entity
#[derive(Debug, Clone)]
pub struct GameEntity {
    /// Position
    pub position: Vec2,
    /// Size
    pub size: Vec2,
    /// Color
    pub color: Color,
    /// Velocity
    pub velocity: Vec2,
    /// Is active
    pub active: bool,
    /// Tag
    pub tag: String,
}

impl Default for ExampleGame {
    fn default() -> Self {
        Self::new()
    }
}

impl ExampleGame {
    /// Create a new example game
    #[must_use]
    pub fn new() -> Self {
        let mut entities = Vec::new();

        // Create some collectibles
        for i in 0..10 {
            entities.push(GameEntity {
                position: Vec2::new(
                    200.0 + (i as f32 * 100.0) % 800.0,
                    150.0 + (i as f32 * 70.0) % 400.0,
                ),
                size: Vec2::new(30.0, 30.0),
                color: Color::new(1.0, 0.8, 0.0, 1.0),
                velocity: Vec2::ZERO,
                active: true,
                tag: "collectible".to_string(),
            });
        }

        // Create some enemies
        for i in 0..5 {
            entities.push(GameEntity {
                position: Vec2::new(
                    300.0 + (i as f32 * 150.0),
                    200.0 + (i as f32 * 100.0) % 300.0,
                ),
                size: Vec2::new(40.0, 40.0),
                color: Color::new(1.0, 0.2, 0.2, 1.0),
                velocity: Vec2::new(
                    50.0 * if i % 2 == 0 { 1.0 } else { -1.0 },
                    30.0 * if i % 3 == 0 { 1.0 } else { -1.0 },
                ),
                active: true,
                tag: "enemy".to_string(),
            });
        }

        Self {
            player_pos: Vec2::new(640.0, 360.0),
            player_vel: Vec2::ZERO,
            speed: 300.0,
            camera_pos: Vec2::ZERO,
            clear_color: Color::new(0.1, 0.1, 0.15, 1.0),
            score: 0,
            entities,
            frame: 0,
        }
    }

    /// Update game logic
    pub fn update(&mut self, input: &Input, dt: f32) {
        self.frame += 1;

        // Player movement
        let mut move_dir = Vec2::ZERO;
        if input.is_key_down(Key::W) || input.is_key_down(Key::Up) {
            move_dir.y -= 1.0;
        }
        if input.is_key_down(Key::S) || input.is_key_down(Key::Down) {
            move_dir.y += 1.0;
        }
        if input.is_key_down(Key::A) || input.is_key_down(Key::Left) {
            move_dir.x -= 1.0;
        }
        if input.is_key_down(Key::D) || input.is_key_down(Key::Right) {
            move_dir.x += 1.0;
        }

        // Sprint
        let current_speed = if input.is_key_down(Key::LeftShift) {
            self.speed * 2.0
        } else {
            self.speed
        };

        // Apply movement
        if move_dir.length_squared() > 0.0 {
            move_dir = move_dir.normalize();
            self.player_vel = move_dir * current_speed;
        } else {
            // Friction
            self.player_vel = self.player_vel * 0.9;
        }

        self.player_pos = self.player_pos + self.player_vel * dt;

        // Clamp player to screen
        self.player_pos.x = self.player_pos.x.clamp(20.0, 1260.0);
        self.player_pos.y = self.player_pos.y.clamp(20.0, 700.0);

        // Update camera (smooth follow)
        let target_camera = Vec2::new(
            self.player_pos.x - 640.0,
            self.player_pos.y - 360.0,
        );
        self.camera_pos = self.camera_pos.lerp(target_camera, 5.0 * dt);

        // Update enemies
        for entity in &mut self.entities {
            if !entity.active {
                continue;
            }

            if entity.tag == "enemy" {
                entity.position = entity.position + entity.velocity * dt;

                // Bounce off walls
                if entity.position.x < 0.0 || entity.position.x > 1280.0 {
                    entity.velocity.x = -entity.velocity.x;
                }
                if entity.position.y < 0.0 || entity.position.y > 720.0 {
                    entity.velocity.y = -entity.velocity.y;
                }
            }
        }

        // Check collisions
        let player_rect = (
            self.player_pos.x - 20.0,
            self.player_pos.y - 20.0,
            40.0,
            40.0,
        );

        for entity in &mut self.entities {
            if !entity.active {
                continue;
            }

            let entity_rect = (
                entity.position.x - entity.size.x / 2.0,
                entity.position.y - entity.size.y / 2.0,
                entity.size.x,
                entity.size.y,
            );

            if Self::rects_overlap(player_rect, entity_rect) {
                if entity.tag == "collectible" {
                    entity.active = false;
                    self.score += 10;
                    tracing::info!("Collected! Score: {}", self.score);
                } else if entity.tag == "enemy" {
                    // Reset player
                    self.player_pos = Vec2::new(640.0, 360.0);
                    self.player_vel = Vec2::ZERO;
                    tracing::warn!("Hit enemy! Respawning...");
                }
            }
        }

        // Respawn collected items
        if self.frame % 300 == 0 {
            for entity in &mut self.entities {
                if entity.tag == "collectible" && !entity.active {
                    entity.active = true;
                    entity.position = Vec2::new(
                        100.0 + (self.frame as f32 * 0.1) % 1000.0,
                        100.0 + (self.frame as f32 * 0.07) % 500.0,
                    );
                }
            }
        }
    }

    fn rects_overlap(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> bool {
        a.0 < b.0 + b.2 && a.0 + a.2 > b.0 && a.1 < b.1 + b.3 && a.1 + a.3 > b.1
    }

    /// Get render data for the game
    #[must_use]
    pub fn render_data(&self) -> GameRenderData {
        let mut sprites = Vec::new();

        // Player
        sprites.push(RenderSprite {
            position: self.player_pos,
            size: Vec2::new(40.0, 40.0),
            color: Color::new(0.2, 0.8, 0.2, 1.0),
            z_order: 10,
        });

        // Entities
        for entity in &self.entities {
            if entity.active {
                sprites.push(RenderSprite {
                    position: entity.position,
                    size: entity.size,
                    color: entity.color,
                    z_order: 5,
                });
            }
        }

        GameRenderData {
            clear_color: self.clear_color,
            sprites,
            ui_text: vec![
                format!("Score: {}", self.score),
                format!("FPS: {:.0}", 60.0),
                "WASD to move, Shift to sprint".to_string(),
            ],
        }
    }
}

/// Render data for the game
#[derive(Debug, Clone)]
pub struct GameRenderData {
    /// Clear color
    pub clear_color: Color,
    /// Sprites to render
    pub sprites: Vec<RenderSprite>,
    /// UI text
    pub ui_text: Vec<String>,
}

/// Simple sprite for rendering
#[derive(Debug, Clone)]
pub struct RenderSprite {
    /// Position
    pub position: Vec2,
    /// Size
    pub size: Vec2,
    /// Color
    pub color: Color,
    /// Z-order
    pub z_order: i32,
}
