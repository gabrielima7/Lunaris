# Tutorial: Creating a 3D First Person Game

Build a complete first-person game with Lunaris Engine!

## 🎯 What You'll Build

A 3D first-person game with:
- ✅ Mouse look camera
- ✅ WASD movement
- ✅ Physics-based character
- ✅ Shooting mechanics
- ✅ AI enemies
- ✅ Health and ammo

## 📁 Project Setup

```toml
[dependencies]
lunaris-runtime = { path = "../Lunaris/crates/lunaris-runtime" }
lunaris-renderer = { path = "../Lunaris/crates/lunaris-renderer" }
lunaris-physics = { path = "../Lunaris/crates/lunaris-physics" }
lunaris-core = { path = "../Lunaris/crates/lunaris-core" }
lunaris-audio = { path = "../Lunaris/crates/lunaris-audio" }
glam = "0.24"
```

## 🎮 Step 1: Game Structure

```rust
use glam::{Vec3, Quat};
use lunaris_runtime::Application;
use lunaris_renderer::{Camera3D, Render3D};
use lunaris_physics::PhysicsWorld;

struct FPSGame {
    player: Player,
    camera: Camera3D,
    physics: PhysicsWorld,
    enemies: Vec<Enemy>,
    projectiles: Vec<Projectile>,
    level: Level,
}

struct Player {
    position: Vec3,
    velocity: Vec3,
    yaw: f32,      // Horizontal rotation
    pitch: f32,    // Vertical rotation
    grounded: bool,
    health: f32,
    ammo: u32,
    weapon: Weapon,
}

struct Enemy {
    position: Vec3,
    health: f32,
    state: EnemyState,
    target: Option<Vec3>,
}

enum EnemyState {
    Idle,
    Patrol,
    Chase,
    Attack,
    Dead,
}

struct Projectile {
    position: Vec3,
    velocity: Vec3,
    lifetime: f32,
}

struct Weapon {
    damage: f32,
    fire_rate: f32,
    last_shot: f32,
}
```

## 🖱️ Step 2: Mouse Look Camera

```rust
impl FPSGame {
    const MOUSE_SENSITIVITY: f32 = 0.002;
    const PITCH_LIMIT: f32 = 1.5; // ~85 degrees

    fn update_camera(&mut self, input: &Input) {
        let mouse_delta = input.mouse_delta();

        // Update yaw (horizontal)
        self.player.yaw -= mouse_delta.x * Self::MOUSE_SENSITIVITY;

        // Update pitch (vertical) with clamping
        self.player.pitch -= mouse_delta.y * Self::MOUSE_SENSITIVITY;
        self.player.pitch = self.player.pitch.clamp(-Self::PITCH_LIMIT, Self::PITCH_LIMIT);

        // Calculate camera direction
        let direction = Vec3::new(
            self.player.yaw.cos() * self.player.pitch.cos(),
            self.player.pitch.sin(),
            self.player.yaw.sin() * self.player.pitch.cos(),
        ).normalize();

        // Update camera
        self.camera.position = self.player.position + Vec3::new(0.0, 1.7, 0.0); // Eye height
        self.camera.look_at(self.camera.position + direction);
    }

    fn get_forward(&self) -> Vec3 {
        Vec3::new(
            self.player.yaw.cos(),
            0.0,
            self.player.yaw.sin(),
        ).normalize()
    }

    fn get_right(&self) -> Vec3 {
        Vec3::new(
            (self.player.yaw - std::f32::consts::FRAC_PI_2).cos(),
            0.0,
            (self.player.yaw - std::f32::consts::FRAC_PI_2).sin(),
        ).normalize()
    }
}
```

## 🏃 Step 3: WASD Movement

```rust
impl FPSGame {
    const MOVE_SPEED: f32 = 5.0;
    const SPRINT_MULTIPLIER: f32 = 1.5;
    const JUMP_FORCE: f32 = 8.0;
    const GRAVITY: f32 = 20.0;

    fn update_movement(&mut self, dt: f32, input: &Input) {
        let forward = self.get_forward();
        let right = self.get_right();

        // Input
        let mut move_dir = Vec3::ZERO;
        
        if input.is_key_down(Key::W) { move_dir += forward; }
        if input.is_key_down(Key::S) { move_dir -= forward; }
        if input.is_key_down(Key::A) { move_dir -= right; }
        if input.is_key_down(Key::D) { move_dir += right; }

        // Normalize and apply speed
        if move_dir.length_squared() > 0.0 {
            move_dir = move_dir.normalize();
            
            let speed = if input.is_key_down(Key::Shift) {
                Self::MOVE_SPEED * Self::SPRINT_MULTIPLIER
            } else {
                Self::MOVE_SPEED
            };

            self.player.velocity.x = move_dir.x * speed;
            self.player.velocity.z = move_dir.z * speed;
        } else {
            // Friction
            self.player.velocity.x *= 0.9;
            self.player.velocity.z *= 0.9;
        }

        // Jumping
        if self.player.grounded && input.is_key_pressed(Key::Space) {
            self.player.velocity.y = Self::JUMP_FORCE;
            self.player.grounded = false;
        }

        // Gravity
        if !self.player.grounded {
            self.player.velocity.y -= Self::GRAVITY * dt;
        }

        // Apply velocity
        self.player.position += self.player.velocity * dt;

        // Ground check (simplified)
        if self.player.position.y <= 0.0 {
            self.player.position.y = 0.0;
            self.player.velocity.y = 0.0;
            self.player.grounded = true;
        }
    }
}
```

## 🔫 Step 4: Shooting

```rust
impl FPSGame {
    fn update_shooting(&mut self, game_time: f32, input: &Input) {
        // Check fire input
        if !input.is_mouse_down(MouseButton::Left) {
            return;
        }

        // Check fire rate
        let time_since_shot = game_time - self.player.weapon.last_shot;
        if time_since_shot < 1.0 / self.player.weapon.fire_rate {
            return;
        }

        // Check ammo
        if self.player.ammo == 0 {
            // Play empty click sound
            return;
        }

        // Fire!
        self.player.ammo -= 1;
        self.player.weapon.last_shot = game_time;

        // Create projectile
        let direction = self.camera.forward();
        let projectile = Projectile {
            position: self.camera.position + direction * 0.5,
            velocity: direction * 50.0, // 50 m/s
            lifetime: 3.0,
        };
        self.projectiles.push(projectile);

        // Play sound
        // lunaris.audio.play("shoot.wav");

        // Muzzle flash effect
        // self.effects.spawn_muzzle_flash(self.camera.position + direction * 0.3);
    }

    fn update_projectiles(&mut self, dt: f32) {
        let mut hits = Vec::new();

        for (i, projectile) in self.projectiles.iter_mut().enumerate() {
            // Move projectile
            projectile.position += projectile.velocity * dt;
            projectile.lifetime -= dt;

            // Check enemy hits
            for (j, enemy) in self.enemies.iter().enumerate() {
                if matches!(enemy.state, EnemyState::Dead) {
                    continue;
                }

                let distance = (enemy.position - projectile.position).length();
                if distance < 1.0 { // Hit radius
                    hits.push((i, j));
                }
            }
        }

        // Apply hits (in reverse to not invalidate indices)
        for (proj_idx, enemy_idx) in hits.into_iter().rev() {
            self.projectiles.remove(proj_idx);
            self.enemies[enemy_idx].health -= self.player.weapon.damage;
            
            if self.enemies[enemy_idx].health <= 0.0 {
                self.enemies[enemy_idx].state = EnemyState::Dead;
            }
        }

        // Remove expired projectiles
        self.projectiles.retain(|p| p.lifetime > 0.0);
    }
}
```

## 🤖 Step 5: Enemy AI

```rust
impl FPSGame {
    const ENEMY_SPEED: f32 = 3.0;
    const DETECTION_RANGE: f32 = 15.0;
    const ATTACK_RANGE: f32 = 2.0;

    fn update_enemies(&mut self, dt: f32) {
        let player_pos = self.player.position;

        for enemy in &mut self.enemies {
            if matches!(enemy.state, EnemyState::Dead) {
                continue;
            }

            let to_player = player_pos - enemy.position;
            let distance = to_player.length();

            match enemy.state {
                EnemyState::Idle | EnemyState::Patrol => {
                    // Check if player is in range
                    if distance < Self::DETECTION_RANGE {
                        enemy.state = EnemyState::Chase;
                        enemy.target = Some(player_pos);
                    }
                }

                EnemyState::Chase => {
                    // Move towards player
                    if distance > Self::ATTACK_RANGE {
                        let direction = to_player.normalize();
                        enemy.position += direction * Self::ENEMY_SPEED * dt;
                        enemy.target = Some(player_pos);
                    } else {
                        enemy.state = EnemyState::Attack;
                    }

                    // Lost player
                    if distance > Self::DETECTION_RANGE * 1.5 {
                        enemy.state = EnemyState::Patrol;
                        enemy.target = None;
                    }
                }

                EnemyState::Attack => {
                    // Attack player
                    // self.player.health -= ENEMY_DAMAGE * dt;

                    // Back to chase if player moved away
                    if distance > Self::ATTACK_RANGE {
                        enemy.state = EnemyState::Chase;
                    }
                }

                EnemyState::Dead => {}
            }
        }
    }
}
```

## 🖼️ Step 6: Rendering

```rust
impl Application for FPSGame {
    fn render(&mut self, renderer: &mut Render3D) {
        renderer.begin_frame(&self.camera);

        // Draw level
        for mesh in &self.level.meshes {
            renderer.draw_mesh(mesh, &mesh.transform, &mesh.material);
        }

        // Draw enemies
        for enemy in &self.enemies {
            if matches!(enemy.state, EnemyState::Dead) {
                continue;
            }

            let transform = Transform::from_position(enemy.position);
            renderer.draw_mesh(&self.enemy_mesh, &transform, &self.enemy_material);
        }

        // Draw projectiles
        for projectile in &self.projectiles {
            let transform = Transform::from_position(projectile.position)
                .with_scale(Vec3::splat(0.1));
            renderer.draw_mesh(&self.bullet_mesh, &transform, &self.bullet_material);
        }

        renderer.end_frame();

        // Draw HUD (2D overlay)
        self.render_hud(renderer);
    }

    fn render_hud(&self, renderer: &mut Render2D) {
        // Crosshair
        let center = Vec2::new(400.0, 300.0);
        renderer.draw_line(center - Vec2::new(10.0, 0.0), center + Vec2::new(10.0, 0.0), Color::WHITE);
        renderer.draw_line(center - Vec2::new(0.0, 10.0), center + Vec2::new(0.0, 10.0), Color::WHITE);

        // Health bar
        renderer.draw_rect(Vec2::new(20.0, 550.0), Vec2::new(200.0, 20.0), Color::RED);
        renderer.draw_rect(
            Vec2::new(20.0, 550.0), 
            Vec2::new(200.0 * (self.player.health / 100.0), 20.0), 
            Color::GREEN,
        );

        // Ammo
        renderer.draw_text(
            &format!("Ammo: {}", self.player.ammo),
            Vec2::new(700.0, 550.0),
            24.0,
            Color::WHITE,
        );
    }
}
```

## 🎬 Step 7: Main Loop

```rust
fn main() {
    let config = WindowConfig {
        title: "FPS Game".to_string(),
        width: 800,
        height: 600,
        cursor_locked: true, // Lock cursor for mouse look
        ..Default::default()
    };

    let mut game = FPSGame::new();
    run_game!(game, config);
}
```

## 🎯 Challenges

1. **Weapon Switching**: Multiple weapons (pistol, shotgun, rifle)
2. **Reloading**: Ammo clips with reload animation
3. **Headshots**: Different damage for body parts
4. **Cover System**: AI uses cover
5. **Multiplayer**: Network opponents

---

**Next Tutorial**: [RPG Systems →](rpg.md)
