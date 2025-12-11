//! Lunaris Engine Demo: Survival Horror
//!
//! A complete demo game showcasing all engine features.

use glam::{Vec2, Vec3, Vec4, Quat, Mat4};
use std::collections::HashMap;

// ==================== GAME CONFIG ====================

/// Game configuration
pub struct GameConfig {
    pub difficulty: Difficulty,
    pub graphics: GraphicsSettings,
    pub audio: AudioSettings,
    pub controls: ControlSettings,
}

/// Difficulty
pub enum Difficulty { Easy, Normal, Hard, Nightmare }

/// Graphics settings
pub struct GraphicsSettings {
    pub resolution: (u32, u32),
    pub fullscreen: bool,
    pub vsync: bool,
    pub quality: QualityPreset,
    pub ray_tracing: bool,
    pub dlss: bool,
}

/// Quality preset
pub enum QualityPreset { Low, Medium, High, Ultra, Custom }

/// Audio settings
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub voice_volume: f32,
    pub subtitles: bool,
}

/// Control settings
pub struct ControlSettings {
    pub mouse_sensitivity: f32,
    pub invert_y: bool,
    pub controller_enabled: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            difficulty: Difficulty::Normal,
            graphics: GraphicsSettings { resolution: (1920, 1080), fullscreen: true, vsync: true, quality: QualityPreset::High, ray_tracing: false, dlss: false },
            audio: AudioSettings { master_volume: 1.0, music_volume: 0.8, sfx_volume: 1.0, voice_volume: 1.0, subtitles: true },
            controls: ControlSettings { mouse_sensitivity: 1.0, invert_y: false, controller_enabled: true },
        }
    }
}

// ==================== PLAYER ====================

/// Player
pub struct Player {
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    pub health: f32,
    pub max_health: f32,
    pub stamina: f32,
    pub max_stamina: f32,
    pub sanity: f32,
    pub inventory: Inventory,
    pub equipped_item: Option<usize>,
    pub state: PlayerState,
    pub flashlight: Flashlight,
}

/// Player state
pub enum PlayerState { Idle, Walking, Running, Crouching, Hiding, Dead }

/// Flashlight
pub struct Flashlight {
    pub on: bool,
    pub battery: f32,
    pub max_battery: f32,
    pub drain_rate: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO, rotation: Quat::IDENTITY, velocity: Vec3::ZERO,
            health: 100.0, max_health: 100.0, stamina: 100.0, max_stamina: 100.0, sanity: 100.0,
            inventory: Inventory::new(20), equipped_item: None, state: PlayerState::Idle,
            flashlight: Flashlight { on: false, battery: 100.0, max_battery: 100.0, drain_rate: 1.0 },
        }
    }
}

impl Player {
    pub fn update(&mut self, dt: f32, input: &PlayerInput) {
        // Movement
        let move_speed = match self.state {
            PlayerState::Running => 8.0,
            PlayerState::Crouching => 2.0,
            _ => 5.0,
        };

        let forward = self.rotation * Vec3::NEG_Z;
        let right = self.rotation * Vec3::X;
        let movement = (forward * input.move_forward + right * input.move_right).normalize_or_zero();
        self.velocity = movement * move_speed;
        self.position += self.velocity * dt;

        // Stamina
        if matches!(self.state, PlayerState::Running) && movement.length() > 0.1 {
            self.stamina = (self.stamina - 20.0 * dt).max(0.0);
            if self.stamina <= 0.0 { self.state = PlayerState::Walking; }
        } else {
            self.stamina = (self.stamina + 10.0 * dt).min(self.max_stamina);
        }

        // Flashlight
        if self.flashlight.on {
            self.flashlight.battery = (self.flashlight.battery - self.flashlight.drain_rate * dt).max(0.0);
            if self.flashlight.battery <= 0.0 { self.flashlight.on = false; }
        }

        // Sanity decay in darkness
        if !self.flashlight.on {
            self.sanity = (self.sanity - 0.5 * dt).max(0.0);
        } else {
            self.sanity = (self.sanity + 0.2 * dt).min(100.0);
        }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.health = (self.health - amount).max(0.0);
        if self.health <= 0.0 { self.state = PlayerState::Dead; }
    }

    pub fn heal(&mut self, amount: f32) {
        self.health = (self.health + amount).min(self.max_health);
    }
}

/// Player input
pub struct PlayerInput {
    pub move_forward: f32,
    pub move_right: f32,
    pub look_delta: Vec2,
    pub run: bool,
    pub crouch: bool,
    pub interact: bool,
    pub flashlight: bool,
    pub use_item: bool,
}

// ==================== INVENTORY ====================

/// Inventory
pub struct Inventory {
    pub items: Vec<Option<Item>>,
    pub capacity: usize,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self { items: vec![None; capacity], capacity }
    }

    pub fn add(&mut self, item: Item) -> bool {
        for slot in &mut self.items {
            if slot.is_none() { *slot = Some(item); return true; }
        }
        false
    }

    pub fn remove(&mut self, index: usize) -> Option<Item> {
        if index < self.items.len() { self.items[index].take() } else { None }
    }

    pub fn count(&self) -> usize {
        self.items.iter().filter(|i| i.is_some()).count()
    }
}

/// Item
#[derive(Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub quantity: u32,
    pub max_stack: u32,
}

/// Item type
#[derive(Clone)]
pub enum ItemType {
    Key { key_id: String },
    Consumable { heal: f32, stamina: f32, sanity: f32 },
    Battery,
    Note { text: String },
    Weapon { damage: f32, range: f32 },
    Quest { quest_id: String },
}

// ==================== ENEMIES ====================

/// Enemy
pub struct Enemy {
    pub id: u64,
    pub enemy_type: EnemyType,
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    pub health: f32,
    pub state: EnemyState,
    pub ai: EnemyAI,
    pub patrol_points: Vec<Vec3>,
    pub current_patrol: usize,
    pub detection: Detection,
}

/// Enemy type
pub enum EnemyType { Shadow, Crawler, Watcher, Stalker, Boss }

/// Enemy state
pub enum EnemyState { Idle, Patrolling, Investigating, Chasing, Attacking, Stunned, Dead }

/// Enemy AI
pub struct EnemyAI {
    pub aggression: f32,
    pub speed: f32,
    pub attack_range: f32,
    pub attack_damage: f32,
    pub attack_cooldown: f32,
    pub last_attack: f32,
}

/// Detection
pub struct Detection {
    pub sight_range: f32,
    pub sight_angle: f32,
    pub hearing_range: f32,
    pub awareness: f32,
    pub last_known_position: Option<Vec3>,
}

impl Enemy {
    pub fn update(&mut self, dt: f32, player_pos: Vec3, player_visible: bool) {
        match self.state {
            EnemyState::Patrolling => self.patrol(dt),
            EnemyState::Investigating => self.investigate(dt),
            EnemyState::Chasing => self.chase(dt, player_pos),
            EnemyState::Attacking => self.attack(dt),
            _ => {}
        }

        // Detection
        if player_visible {
            let dist = (player_pos - self.position).length();
            if dist < self.detection.sight_range {
                self.detection.awareness += dt * 50.0;
                if self.detection.awareness >= 100.0 {
                    self.state = EnemyState::Chasing;
                    self.detection.last_known_position = Some(player_pos);
                }
            }
        } else {
            self.detection.awareness = (self.detection.awareness - dt * 20.0).max(0.0);
        }
    }

    fn patrol(&mut self, dt: f32) {
        if self.patrol_points.is_empty() { return; }
        let target = self.patrol_points[self.current_patrol];
        let dir = (target - self.position).normalize_or_zero();
        self.position += dir * self.ai.speed * 0.5 * dt;

        if (self.position - target).length() < 0.5 {
            self.current_patrol = (self.current_patrol + 1) % self.patrol_points.len();
        }
    }

    fn investigate(&mut self, dt: f32) {
        if let Some(pos) = self.detection.last_known_position {
            let dir = (pos - self.position).normalize_or_zero();
            self.position += dir * self.ai.speed * 0.7 * dt;
            if (self.position - pos).length() < 1.0 {
                self.state = EnemyState::Patrolling;
                self.detection.last_known_position = None;
            }
        }
    }

    fn chase(&mut self, dt: f32, player_pos: Vec3) {
        let dir = (player_pos - self.position).normalize_or_zero();
        self.position += dir * self.ai.speed * dt;
        self.detection.last_known_position = Some(player_pos);

        if (self.position - player_pos).length() < self.ai.attack_range {
            self.state = EnemyState::Attacking;
        }
    }

    fn attack(&mut self, dt: f32) {
        self.ai.last_attack += dt;
        if self.ai.last_attack >= self.ai.attack_cooldown {
            self.ai.last_attack = 0.0;
            // Deal damage to player
        }
    }
}

// ==================== WORLD ====================

/// Game world
pub struct GameWorld {
    pub rooms: Vec<Room>,
    pub current_room: usize,
    pub doors: Vec<Door>,
    pub interactables: Vec<Interactable>,
    pub triggers: Vec<Trigger>,
    pub lighting: LightingState,
}

/// Room
pub struct Room {
    pub id: String,
    pub name: String,
    pub bounds: (Vec3, Vec3),
    pub ambient_light: Vec3,
    pub fog_density: f32,
    pub music: Option<String>,
}

/// Door
pub struct Door {
    pub id: String,
    pub position: Vec3,
    pub rotation: Quat,
    pub locked: bool,
    pub required_key: Option<String>,
    pub connects: (String, String),
    pub open: bool,
}

/// Interactable
pub struct Interactable {
    pub id: String,
    pub position: Vec3,
    pub interaction_type: InteractionType,
    pub used: bool,
}

/// Interaction type
pub enum InteractionType {
    Pickup(Item),
    Examine(String),
    Switch { target: String, on: bool },
    Save,
    Puzzle { puzzle_id: String },
}

/// Trigger
pub struct Trigger {
    pub id: String,
    pub bounds: (Vec3, Vec3),
    pub trigger_type: TriggerType,
    pub triggered: bool,
    pub repeatable: bool,
}

/// Trigger type
pub enum TriggerType {
    Cutscene(String),
    SpawnEnemy(EnemyType, Vec3),
    PlaySound(String),
    SetFlag(String, bool),
    Checkpoint,
}

/// Lighting state
pub struct LightingState {
    pub global_intensity: f32,
    pub flicker: bool,
    pub power_out: bool,
}

// ==================== GAME STATE ====================

/// Main game state
pub struct SurvivalHorrorGame {
    pub config: GameConfig,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub world: GameWorld,
    pub objectives: Vec<Objective>,
    pub notes_collected: Vec<String>,
    pub game_time: f32,
    pub state: GameState,
    pub save_data: Option<SaveData>,
}

/// Game state
pub enum GameState { MainMenu, Playing, Paused, Inventory, Cutscene, GameOver, Victory }

/// Objective
pub struct Objective {
    pub id: String,
    pub description: String,
    pub completed: bool,
    pub hidden: bool,
}

/// Save data
pub struct SaveData {
    pub player_position: Vec3,
    pub player_health: f32,
    pub inventory: Vec<String>,
    pub objectives_completed: Vec<String>,
    pub notes_collected: Vec<String>,
    pub game_time: f32,
    pub flags: HashMap<String, bool>,
}

impl SurvivalHorrorGame {
    pub fn new() -> Self {
        Self {
            config: GameConfig::default(),
            player: Player::default(),
            enemies: Vec::new(),
            world: GameWorld {
                rooms: Vec::new(), current_room: 0, doors: Vec::new(),
                interactables: Vec::new(), triggers: Vec::new(),
                lighting: LightingState { global_intensity: 0.3, flicker: false, power_out: false },
            },
            objectives: vec![
                Objective { id: "find_key".into(), description: "Find the basement key".into(), completed: false, hidden: false },
                Objective { id: "escape".into(), description: "Escape the mansion".into(), completed: false, hidden: true },
            ],
            notes_collected: Vec::new(),
            game_time: 0.0,
            state: GameState::MainMenu,
            save_data: None,
        }
    }

    pub fn start_new_game(&mut self) {
        self.player = Player::default();
        self.player.position = Vec3::new(0.0, 1.0, 0.0);
        self.state = GameState::Playing;
        self.game_time = 0.0;
        self.spawn_enemies();
    }

    fn spawn_enemies(&mut self) {
        self.enemies.push(Enemy {
            id: 0, enemy_type: EnemyType::Shadow, position: Vec3::new(10.0, 0.0, 10.0),
            rotation: Quat::IDENTITY, velocity: Vec3::ZERO, health: 50.0,
            state: EnemyState::Patrolling,
            ai: EnemyAI { aggression: 0.5, speed: 4.0, attack_range: 2.0, attack_damage: 20.0, attack_cooldown: 1.5, last_attack: 0.0 },
            patrol_points: vec![Vec3::new(10.0, 0.0, 10.0), Vec3::new(10.0, 0.0, -10.0), Vec3::new(-10.0, 0.0, -10.0)],
            current_patrol: 0,
            detection: Detection { sight_range: 15.0, sight_angle: 90.0, hearing_range: 10.0, awareness: 0.0, last_known_position: None },
        });
    }

    pub fn update(&mut self, dt: f32, input: &PlayerInput) {
        if !matches!(self.state, GameState::Playing) { return; }

        self.game_time += dt;
        self.player.update(dt, input);

        let player_pos = self.player.position;
        for enemy in &mut self.enemies {
            let visible = self.is_player_visible(enemy);
            enemy.update(dt, player_pos, visible);
        }

        self.check_triggers();
        self.check_objectives();
    }

    fn is_player_visible(&self, enemy: &Enemy) -> bool {
        let to_player = self.player.position - enemy.position;
        let dist = to_player.length();
        if dist > enemy.detection.sight_range { return false; }
        let forward = enemy.rotation * Vec3::NEG_Z;
        let angle = forward.dot(to_player.normalize()).acos().to_degrees();
        angle < enemy.detection.sight_angle / 2.0
    }

    fn check_triggers(&mut self) {
        for trigger in &mut self.world.triggers {
            if trigger.triggered && !trigger.repeatable { continue; }
            let in_bounds = self.player.position.x >= trigger.bounds.0.x && self.player.position.x <= trigger.bounds.1.x
                && self.player.position.y >= trigger.bounds.0.y && self.player.position.y <= trigger.bounds.1.y
                && self.player.position.z >= trigger.bounds.0.z && self.player.position.z <= trigger.bounds.1.z;
            if in_bounds { trigger.triggered = true; }
        }
    }

    fn check_objectives(&mut self) {
        // Check if all objectives complete
        if self.objectives.iter().all(|o| o.completed || o.hidden) {
            // Unhide escape objective
            if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == "escape") {
                obj.hidden = false;
            }
        }
    }

    pub fn save(&self) -> SaveData {
        SaveData {
            player_position: self.player.position,
            player_health: self.player.health,
            inventory: self.player.inventory.items.iter().filter_map(|i| i.as_ref().map(|item| item.id.clone())).collect(),
            objectives_completed: self.objectives.iter().filter(|o| o.completed).map(|o| o.id.clone()).collect(),
            notes_collected: self.notes_collected.clone(),
            game_time: self.game_time,
            flags: HashMap::new(),
        }
    }

    pub fn load(&mut self, save: SaveData) {
        self.player.position = save.player_position;
        self.player.health = save.player_health;
        self.game_time = save.game_time;
        self.notes_collected = save.notes_collected;
        self.state = GameState::Playing;
    }
}

// ==================== ENTRY POINT ====================

/// Run the demo game
pub fn run_demo() {
    println!("🎮 Lunaris Engine Demo: Survival Horror");
    println!("=========================================");
    
    let mut game = SurvivalHorrorGame::new();
    game.start_new_game();
    
    // Simulate game loop
    let dt = 1.0 / 60.0;
    let input = PlayerInput {
        move_forward: 1.0, move_right: 0.0, look_delta: Vec2::ZERO,
        run: false, crouch: false, interact: false, flashlight: true, use_item: false,
    };
    
    for frame in 0..600 {
        game.update(dt, &input);
        
        if frame % 60 == 0 {
            println!("Time: {:.1}s | Health: {:.0} | Stamina: {:.0} | Sanity: {:.0}",
                game.game_time, game.player.health, game.player.stamina, game.player.sanity);
        }
    }
    
    println!("\n✅ Demo completed!");
    println!("Player Position: {:?}", game.player.position);
    println!("Enemies: {}", game.enemies.len());
}
