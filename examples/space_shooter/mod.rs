//! Complete Game Example: Space Shooter
//!
//! A full mini-game demonstrating Lunaris features.

use glam::{Vec2, Vec3, Quat};
use std::collections::HashMap;

// ==================== COMPONENTS ====================

/// Player component
pub struct Player {
    pub speed: f32,
    pub fire_rate: f32,
    pub fire_cooldown: f32,
    pub lives: u32,
    pub shield: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self { speed: 10.0, fire_rate: 0.2, fire_cooldown: 0.0, lives: 3, shield: false }
    }
}

/// Enemy component
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub health: f32,
    pub speed: f32,
    pub points: u32,
}

/// Enemy type
pub enum EnemyType { Basic, Fast, Tank, Boss }

/// Bullet
pub struct Bullet {
    pub velocity: Vec3,
    pub damage: f32,
    pub from_player: bool,
    pub lifetime: f32,
}

/// Powerup
pub struct Powerup {
    pub powerup_type: PowerupType,
    pub lifetime: f32,
}

/// Powerup type
pub enum PowerupType { Shield, SpeedBoost, TripleShot, ExtraLife }

/// Transform
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self { position: Vec3::ZERO, rotation: Quat::IDENTITY, scale: Vec3::ONE }
    }
}

// ==================== GAME STATE ====================

/// Game state
pub struct SpaceShooter {
    pub player: Player,
    pub player_pos: Vec3,
    pub enemies: Vec<(Enemy, Transform)>,
    pub bullets: Vec<(Bullet, Transform)>,
    pub powerups: Vec<(Powerup, Transform)>,
    pub score: u32,
    pub high_score: u32,
    pub wave: u32,
    pub state: GameState,
    pub spawn_timer: f32,
}

/// Game state
pub enum GameState { Menu, Playing, Paused, GameOver }

impl SpaceShooter {
    pub fn new() -> Self {
        Self {
            player: Player::default(),
            player_pos: Vec3::new(0.0, 0.0, -8.0),
            enemies: Vec::new(),
            bullets: Vec::new(),
            powerups: Vec::new(),
            score: 0,
            high_score: 0,
            wave: 1,
            state: GameState::Menu,
            spawn_timer: 0.0,
        }
    }

    pub fn start(&mut self) {
        self.player = Player::default();
        self.player_pos = Vec3::new(0.0, 0.0, -8.0);
        self.enemies.clear();
        self.bullets.clear();
        self.powerups.clear();
        self.score = 0;
        self.wave = 1;
        self.state = GameState::Playing;
    }

    pub fn update(&mut self, dt: f32, input: &Input) {
        match self.state {
            GameState::Menu => {
                if input.start_pressed { self.start(); }
            }
            GameState::Playing => {
                self.update_player(dt, input);
                self.update_enemies(dt);
                self.update_bullets(dt);
                self.update_powerups(dt);
                self.check_collisions();
                self.spawn_enemies(dt);
            }
            GameState::Paused => {
                if input.pause_pressed { self.state = GameState::Playing; }
            }
            GameState::GameOver => {
                if input.start_pressed { self.start(); }
            }
        }
    }

    fn update_player(&mut self, dt: f32, input: &Input) {
        // Movement
        let move_dir = Vec3::new(input.move_x, 0.0, input.move_y);
        self.player_pos += move_dir * self.player.speed * dt;
        self.player_pos.x = self.player_pos.x.clamp(-10.0, 10.0);
        self.player_pos.z = self.player_pos.z.clamp(-10.0, 0.0);

        // Shooting
        self.player.fire_cooldown -= dt;
        if input.fire && self.player.fire_cooldown <= 0.0 {
            self.fire_bullet();
            self.player.fire_cooldown = self.player.fire_rate;
        }

        // Pause
        if input.pause_pressed { self.state = GameState::Paused; }
    }

    fn fire_bullet(&mut self) {
        self.bullets.push((
            Bullet { velocity: Vec3::new(0.0, 0.0, 20.0), damage: 1.0, from_player: true, lifetime: 3.0 },
            Transform { position: self.player_pos + Vec3::new(0.0, 0.0, 1.0), ..Default::default() },
        ));
    }

    fn update_enemies(&mut self, dt: f32) {
        for (enemy, transform) in &mut self.enemies {
            transform.position.z -= enemy.speed * dt;
            
            // Shoot occasionally
            if rand() < 0.01 {
                self.bullets.push((
                    Bullet { velocity: Vec3::new(0.0, 0.0, -15.0), damage: 1.0, from_player: false, lifetime: 3.0 },
                    Transform { position: transform.position, ..Default::default() },
                ));
            }
        }
        
        // Remove off-screen enemies
        self.enemies.retain(|(_, t)| t.position.z > -15.0);
    }

    fn update_bullets(&mut self, dt: f32) {
        for (bullet, transform) in &mut self.bullets {
            transform.position += bullet.velocity * dt;
            bullet.lifetime -= dt;
        }
        self.bullets.retain(|(b, _)| b.lifetime > 0.0);
    }

    fn update_powerups(&mut self, dt: f32) {
        for (powerup, transform) in &mut self.powerups {
            transform.position.z -= 3.0 * dt;
            powerup.lifetime -= dt;
        }
        self.powerups.retain(|(p, t)| p.lifetime > 0.0 && t.position.z > -15.0);
    }

    fn check_collisions(&mut self) {
        // Bullets vs Enemies
        let mut enemies_to_remove = Vec::new();
        let mut bullets_to_remove = Vec::new();

        for (bi, (bullet, bt)) in self.bullets.iter().enumerate() {
            if !bullet.from_player { continue; }
            for (ei, (enemy, et)) in self.enemies.iter().enumerate() {
                if (bt.position - et.position).length() < 1.0 {
                    bullets_to_remove.push(bi);
                    enemies_to_remove.push(ei);
                    self.score += enemy.points;
                    
                    // Chance to spawn powerup
                    if rand() < 0.1 {
                        self.spawn_powerup(et.position);
                    }
                }
            }
        }

        // Remove in reverse order
        bullets_to_remove.sort();
        enemies_to_remove.sort();
        for i in bullets_to_remove.into_iter().rev() { if i < self.bullets.len() { self.bullets.remove(i); } }
        for i in enemies_to_remove.into_iter().rev() { if i < self.enemies.len() { self.enemies.remove(i); } }

        // Bullets vs Player
        for (bullet, bt) in &self.bullets {
            if bullet.from_player { continue; }
            if (bt.position - self.player_pos).length() < 1.0 {
                if self.player.shield {
                    self.player.shield = false;
                } else {
                    self.player.lives -= 1;
                    if self.player.lives == 0 {
                        self.game_over();
                    }
                }
            }
        }

        // Powerups vs Player
        let mut powerups_to_remove = Vec::new();
        for (pi, (powerup, pt)) in self.powerups.iter().enumerate() {
            if (pt.position - self.player_pos).length() < 1.5 {
                self.apply_powerup(&powerup.powerup_type);
                powerups_to_remove.push(pi);
            }
        }
        for i in powerups_to_remove.into_iter().rev() { self.powerups.remove(i); }
    }

    fn spawn_enemies(&mut self, dt: f32) {
        self.spawn_timer -= dt;
        if self.spawn_timer <= 0.0 {
            let spawn_rate = 2.0 - (self.wave as f32 * 0.1).min(1.5);
            self.spawn_timer = spawn_rate;

            let x = (rand() - 0.5) * 18.0;
            let enemy_type = if rand() < 0.1 { EnemyType::Tank } else if rand() < 0.3 { EnemyType::Fast } else { EnemyType::Basic };
            let (health, speed, points) = match enemy_type {
                EnemyType::Basic => (1.0, 3.0, 100),
                EnemyType::Fast => (1.0, 6.0, 150),
                EnemyType::Tank => (3.0, 2.0, 300),
                EnemyType::Boss => (10.0, 1.5, 1000),
            };

            self.enemies.push((
                Enemy { enemy_type, health, speed, points },
                Transform { position: Vec3::new(x, 0.0, 15.0), ..Default::default() },
            ));

            // Wave progression
            if self.score > self.wave as u32 * 1000 {
                self.wave += 1;
            }
        }
    }

    fn spawn_powerup(&mut self, position: Vec3) {
        let powerup_type = match (rand() * 4.0) as u32 {
            0 => PowerupType::Shield,
            1 => PowerupType::SpeedBoost,
            2 => PowerupType::TripleShot,
            _ => PowerupType::ExtraLife,
        };
        self.powerups.push((
            Powerup { powerup_type, lifetime: 10.0 },
            Transform { position, ..Default::default() },
        ));
    }

    fn apply_powerup(&mut self, powerup: &PowerupType) {
        match powerup {
            PowerupType::Shield => self.player.shield = true,
            PowerupType::SpeedBoost => self.player.speed = 15.0,
            PowerupType::TripleShot => self.player.fire_rate = 0.1,
            PowerupType::ExtraLife => self.player.lives += 1,
        }
    }

    fn game_over(&mut self) {
        self.state = GameState::GameOver;
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }
}

/// Input state
pub struct Input {
    pub move_x: f32,
    pub move_y: f32,
    pub fire: bool,
    pub pause_pressed: bool,
    pub start_pressed: bool,
}

fn rand() -> f32 { 0.5 } // Placeholder

// ==================== USAGE ====================

/// Example of how to use the game
pub fn run_game() {
    let mut game = SpaceShooter::new();
    let dt = 1.0 / 60.0;
    
    // Simulate game loop
    for _ in 0..100 {
        let input = Input { move_x: 0.0, move_y: 0.0, fire: true, pause_pressed: false, start_pressed: true };
        game.update(dt, &input);
    }
    
    println!("Score: {}", game.score);
    println!("Wave: {}", game.wave);
    println!("Enemies: {}", game.enemies.len());
}
