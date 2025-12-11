# Lunaris Engine Tutorials

## 📺 Tutorial Video Scripts

Este diretório contém scripts para tutoriais em vídeo do YouTube.

### Série: Começando com Lunaris

#### Vídeo 1: Instalação e Primeiro Projeto (15 min)

**Intro (0:00 - 0:30)**
- Apresentação do Lunaris Engine
- O que vamos construir: Hello World

**Instalação (0:30 - 3:00)**
```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clonar Lunaris
git clone https://github.com/gabrielima7/Lunaris.git
cd Lunaris

# Compilar
cargo build --release
```

**Criando Projeto (3:00 - 6:00)**
```bash
cargo new meu_jogo
cd meu_jogo
```

Adicionar ao `Cargo.toml`:
```toml
[dependencies]
lunaris-runtime = { path = "../Lunaris/crates/lunaris-runtime" }
lunaris-core = { path = "../Lunaris/crates/lunaris-core" }
```

**Hello World (6:00 - 10:00)**
```rust
use lunaris_runtime::Application;

struct MeuJogo;

impl Application for MeuJogo {
    fn update(&mut self, dt: f32) {
        println!("Frame: {:.2}ms", dt * 1000.0);
    }
    
    fn render(&mut self) {
        // Renderização virá depois
    }
}

fn main() {
    MeuJogo.run();
}
```

**Explicação do Código (10:00 - 13:00)**
- Trait Application
- Game loop
- Delta time

**Próximos Passos (13:00 - 15:00)**
- Preview do próximo vídeo: Sprites e Input

---

#### Vídeo 2: Sprites e Movimentação (20 min)

**Recap (0:00 - 1:00)**

**Carregando Sprites (1:00 - 5:00)**
```rust
use lunaris_assets::AssetManager;
use lunaris_renderer::Sprite;

struct MeuJogo {
    assets: AssetManager,
    player_sprite: Sprite,
}

impl MeuJogo {
    fn new() -> Self {
        let mut assets = AssetManager::new();
        let texture = assets.load_texture("player.png");
        
        Self {
            assets,
            player_sprite: Sprite::new(texture),
        }
    }
}
```

**Sistema de Input (5:00 - 10:00)**
```rust
use lunaris_core::input::{Input, Key};

fn update(&mut self, dt: f32) {
    let speed = 200.0;
    
    if self.input.is_key_down(Key::W) {
        self.player_y -= speed * dt;
    }
    if self.input.is_key_down(Key::S) {
        self.player_y += speed * dt;
    }
    if self.input.is_key_down(Key::A) {
        self.player_x -= speed * dt;
    }
    if self.input.is_key_down(Key::D) {
        self.player_x += speed * dt;
    }
}
```

**Renderizando (10:00 - 15:00)**
```rust
fn render(&mut self) {
    self.renderer.clear([0.1, 0.1, 0.15, 1.0]);
    
    self.renderer.draw_sprite(
        &self.player_sprite,
        self.player_x,
        self.player_y,
    );
}
```

**Animação de Sprite Sheet (15:00 - 18:00)**
```rust
struct Animation {
    frames: Vec<Rect>,
    current_frame: usize,
    frame_time: f32,
    elapsed: f32,
}

impl Animation {
    fn update(&mut self, dt: f32) {
        self.elapsed += dt;
        if self.elapsed >= self.frame_time {
            self.elapsed = 0.0;
            self.current_frame = (self.current_frame + 1) % self.frames.len();
        }
    }
}
```

**Conclusão (18:00 - 20:00)**

---

#### Vídeo 3: Física e Colisões (20 min)

**Recap (0:00 - 1:00)**

**Adicionando Física (1:00 - 8:00)**
```rust
use lunaris_physics::{RigidBody, Collider, PhysicsWorld};

struct MeuJogo {
    physics: PhysicsWorld,
    player_body: RigidBody,
}

fn setup(&mut self) {
    self.player_body = self.physics.create_body()
        .position(100.0, 100.0)
        .collider(Collider::box2d(32.0, 32.0))
        .build();
}

fn update(&mut self, dt: f32) {
    self.physics.step(dt);
    
    let pos = self.physics.get_position(self.player_body);
    self.player_x = pos.x;
    self.player_y = pos.y;
}
```

**Detectando Colisões (8:00 - 14:00)**
```rust
fn update(&mut self, dt: f32) {
    for collision in self.physics.get_collisions(self.player_body) {
        match collision.other.tag {
            "coin" => {
                self.score += 1;
                self.physics.destroy(collision.other.id);
            }
            "enemy" => {
                self.take_damage(10);
            }
            _ => {}
        }
    }
}
```

**Raycasting (14:00 - 18:00)**
```rust
fn shoot(&mut self) {
    let ray = Ray::new(self.player_pos, self.aim_direction);
    
    if let Some(hit) = self.physics.raycast(ray, 500.0) {
        self.spawn_impact_effect(hit.point);
        
        if let Some(enemy) = self.get_enemy(hit.entity) {
            enemy.take_damage(25);
        }
    }
}
```

**Conclusão (18:00 - 20:00)**

---

#### Vídeo 4: Audio e Efeitos (15 min)

**Música de Fundo (0:00 - 4:00)**
```rust
use lunaris_audio::{AudioEngine, Sound};

fn setup(&mut self) {
    self.audio.play_music("bgm.ogg", 0.5, true);
}
```

**Efeitos Sonoros (4:00 - 8:00)**
```rust
fn on_jump(&mut self) {
    self.audio.play_sfx("jump.wav", 0.8);
}

fn on_coin_collect(&mut self) {
    self.audio.play_sfx("coin.wav", 1.0)
        .with_pitch_variation(0.1);
}
```

**Audio 3D/Espacial (8:00 - 12:00)**
```rust
fn update(&mut self, dt: f32) {
    // Atualizar listener
    self.audio.set_listener_position(self.player_pos);
    
    // Som posicional
    for enemy in &self.enemies {
        self.audio.play_3d(
            "growl.wav",
            enemy.position,
            10.0, // range
        );
    }
}
```

**Conclusão (12:00 - 15:00)**

---

### Série: Projetos Completos

#### Projeto 1: Platformer 2D Completo (1 hora)
- Player Controller
- Níveis com Tilemaps
- Inimigos com IA
- Sistema de Vidas
- Menu e HUD
- Save/Load

#### Projeto 2: Top-Down Shooter (1 hora)
- Movimento 8-direcional
- Sistema de Armas
- Waves de Inimigos
- Power-ups
- Leaderboard

#### Projeto 3: RPG Simples (2 horas)
- Sistema de Stats
- Combate por Turnos
- Inventário
- NPCs e Diálogos
- Quest System

---

## 📋 Checklist de Produção de Vídeo

- [ ] Script revisado
- [ ] Assets preparados (sprites, sons)
- [ ] Código testado
- [ ] Gravação de tela configurada
- [ ] Microfone testado
- [ ] Iluminação OK (se facecam)
- [ ] Edição concluída
- [ ] Thumbnail criada
- [ ] Título e descrição SEO
- [ ] Tags relevantes
- [ ] Playlist atualizada
