# Tutorial: Creating a 2D Platformer

Learn to create a complete 2D platformer game with Lunaris Engine!

## 🎯 What You'll Build

A classic 2D platformer with:
- ✅ Player movement and jumping
- ✅ Gravity and physics
- ✅ Platforms and obstacles
- ✅ Collectibles (coins)
- ✅ Enemies
- ✅ Score system

## 📁 Project Setup

### 1. Create Project

```bash
cargo new my_platformer
cd my_platformer
```

### 2. Add Dependencies

```toml
# Cargo.toml
[dependencies]
lunaris-runtime = { path = "../Lunaris/crates/lunaris-runtime" }
lunaris-renderer = { path = "../Lunaris/crates/lunaris-renderer" }
lunaris-physics = { path = "../Lunaris/crates/lunaris-physics" }
lunaris-core = { path = "../Lunaris/crates/lunaris-core" }
```

## 🎮 Step 1: Game Structure

```rust
// src/main.rs
use lunaris_runtime::{Application, WindowConfig, run_game};
use lunaris_core::math::Vec2;

struct Platformer {
    player: Player,
    platforms: Vec<Platform>,
    coins: Vec<Coin>,
    enemies: Vec<Enemy>,
    score: u32,
    camera_offset: Vec2,
}

struct Player {
    position: Vec2,
    velocity: Vec2,
    grounded: bool,
    facing_right: bool,
}

struct Platform {
    position: Vec2,
    size: Vec2,
}

struct Coin {
    position: Vec2,
    collected: bool,
}

struct Enemy {
    position: Vec2,
    velocity: Vec2,
    alive: bool,
}

impl Platformer {
    fn new() -> Self {
        Self {
            player: Player {
                position: Vec2::new(100.0, 300.0),
                velocity: Vec2::ZERO,
                grounded: false,
                facing_right: true,
            },
            platforms: vec![
                // Ground
                Platform { position: Vec2::new(400.0, 550.0), size: Vec2::new(800.0, 50.0) },
                // Floating platforms
                Platform { position: Vec2::new(200.0, 400.0), size: Vec2::new(150.0, 20.0) },
                Platform { position: Vec2::new(450.0, 300.0), size: Vec2::new(150.0, 20.0) },
                Platform { position: Vec2::new(700.0, 200.0), size: Vec2::new(150.0, 20.0) },
            ],
            coins: vec![
                Coin { position: Vec2::new(200.0, 360.0), collected: false },
                Coin { position: Vec2::new(450.0, 260.0), collected: false },
                Coin { position: Vec2::new(700.0, 160.0), collected: false },
            ],
            enemies: vec![
                Enemy { position: Vec2::new(300.0, 520.0), velocity: Vec2::new(50.0, 0.0), alive: true },
            ],
            score: 0,
            camera_offset: Vec2::ZERO,
        }
    }
}
```

## 🏃 Step 2: Player Movement

```rust
impl Platformer {
    const GRAVITY: f32 = 980.0;
    const MOVE_SPEED: f32 = 300.0;
    const JUMP_FORCE: f32 = 500.0;
    const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 48.0);

    fn update_player(&mut self, dt: f32, input: &Input) {
        // Horizontal movement
        let mut move_x = 0.0;
        
        if input.is_key_down(Key::A) || input.is_key_down(Key::Left) {
            move_x -= 1.0;
            self.player.facing_right = false;
        }
        if input.is_key_down(Key::D) || input.is_key_down(Key::Right) {
            move_x += 1.0;
            self.player.facing_right = true;
        }
        
        self.player.velocity.x = move_x * Self::MOVE_SPEED;

        // Jumping
        if self.player.grounded && input.is_key_pressed(Key::Space) {
            self.player.velocity.y = -Self::JUMP_FORCE;
            self.player.grounded = false;
        }

        // Apply gravity
        if !self.player.grounded {
            self.player.velocity.y += Self::GRAVITY * dt;
        }

        // Update position
        self.player.position += self.player.velocity * dt;

        // Collision detection
        self.check_platform_collisions();
    }

    fn check_platform_collisions(&mut self) {
        self.player.grounded = false;

        for platform in &self.platforms {
            if self.aabb_collision(
                self.player.position, Self::PLAYER_SIZE,
                platform.position, platform.size
            ) {
                // Landing on top
                if self.player.velocity.y > 0.0 {
                    self.player.position.y = platform.position.y - platform.size.y / 2.0 - Self::PLAYER_SIZE.y / 2.0;
                    self.player.velocity.y = 0.0;
                    self.player.grounded = true;
                }
            }
        }
    }

    fn aabb_collision(&self, pos1: Vec2, size1: Vec2, pos2: Vec2, size2: Vec2) -> bool {
        let half1 = size1 / 2.0;
        let half2 = size2 / 2.0;

        (pos1.x - half1.x) < (pos2.x + half2.x) &&
        (pos1.x + half1.x) > (pos2.x - half2.x) &&
        (pos1.y - half1.y) < (pos2.y + half2.y) &&
        (pos1.y + half1.y) > (pos2.y - half2.y)
    }
}
```

## 🪙 Step 3: Collectibles

```rust
impl Platformer {
    const COIN_SIZE: f32 = 24.0;

    fn update_coins(&mut self) {
        for coin in &mut self.coins {
            if coin.collected {
                continue;
            }

            if self.aabb_collision(
                self.player.position, Self::PLAYER_SIZE,
                coin.position, Vec2::splat(Self::COIN_SIZE)
            ) {
                coin.collected = true;
                self.score += 100;
                // Play sound: lunaris.audio.play("coin.wav");
            }
        }
    }
}
```

## 👾 Step 4: Enemies

```rust
impl Platformer {
    const ENEMY_SIZE: Vec2 = Vec2::new(32.0, 32.0);

    fn update_enemies(&mut self, dt: f32) {
        for enemy in &mut self.enemies {
            if !enemy.alive {
                continue;
            }

            // Move enemy
            enemy.position.x += enemy.velocity.x * dt;

            // Reverse at edges (simple patrol)
            if enemy.position.x < 100.0 || enemy.position.x > 700.0 {
                enemy.velocity.x = -enemy.velocity.x;
            }

            // Check collision with player
            if self.aabb_collision(
                self.player.position, Self::PLAYER_SIZE,
                enemy.position, Self::ENEMY_SIZE
            ) {
                // Player jumping on enemy
                if self.player.velocity.y > 0.0 && 
                   self.player.position.y < enemy.position.y {
                    enemy.alive = false;
                    self.player.velocity.y = -300.0; // Bounce
                    self.score += 50;
                } else {
                    // Player hit by enemy
                    self.player.position = Vec2::new(100.0, 300.0);
                    self.player.velocity = Vec2::ZERO;
                }
            }
        }
    }
}
```

## 🖼️ Step 5: Rendering

```rust
impl Application for Platformer {
    fn render(&mut self, renderer: &mut Render2D) {
        // Clear background
        renderer.clear(Color::rgb(0.4, 0.6, 0.9)); // Sky blue

        // Draw platforms
        for platform in &self.platforms {
            renderer.draw_rect(
                platform.position,
                platform.size,
                Color::rgb(0.4, 0.3, 0.2), // Brown
            );
        }

        // Draw coins
        for coin in &self.coins {
            if !coin.collected {
                renderer.draw_circle(
                    coin.position,
                    Self::COIN_SIZE / 2.0,
                    Color::rgb(1.0, 0.8, 0.0), // Gold
                );
            }
        }

        // Draw enemies
        for enemy in &self.enemies {
            if enemy.alive {
                renderer.draw_rect(
                    enemy.position,
                    Self::ENEMY_SIZE,
                    Color::rgb(0.8, 0.2, 0.2), // Red
                );
            }
        }

        // Draw player
        let player_color = if self.player.facing_right {
            Color::rgb(0.2, 0.6, 1.0)
        } else {
            Color::rgb(0.2, 0.5, 0.9)
        };
        renderer.draw_rect(
            self.player.position,
            Self::PLAYER_SIZE,
            player_color,
        );

        // Draw UI
        renderer.draw_text(
            &format!("Score: {}", self.score),
            Vec2::new(20.0, 30.0),
            24.0,
            Color::WHITE,
        );
    }
}
```

## 🎬 Step 6: Main Loop

```rust
impl Application for Platformer {
    fn update(&mut self, dt: f32, input: &Input) {
        self.update_player(dt, input);
        self.update_coins();
        self.update_enemies(dt);
        
        // Camera follows player
        self.camera_offset = Vec2::new(
            -self.player.position.x + 400.0,
            0.0,
        );
    }
}

fn main() {
    let config = WindowConfig {
        title: "My Platformer".to_string(),
        width: 800,
        height: 600,
        ..Default::default()
    };

    let game = Platformer::new();
    run_game!(game, config);
}
```

## 🎯 Challenges

Try adding these features:
1. **Double Jump**: Allow player to jump once in mid-air
2. **Moving Platforms**: Platforms that move up/down or left/right
3. **Power-ups**: Speed boost, invincibility
4. **Levels**: Load different level layouts
5. **Animations**: Sprite animations for player

## 📦 Complete Code

See `examples/platformer/` for the full working example.

---

**Next Tutorial**: [3D First Person →](fps.md)
