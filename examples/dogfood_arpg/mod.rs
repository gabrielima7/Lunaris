//! Dogfooding Game Demo
//!
//! A complete Action-RPG demo to battle-test all Lunaris Engine systems.
//! This game stresses: hot-reload, window management, physics, AI, rendering.

use glam::{Vec2, Vec3, Quat};
use std::collections::HashMap;

// ==================== GAME CONFIG ====================

/// Game configuration
pub struct GameConfig {
    pub title: String,
    pub resolution: (u32, u32),
    pub fullscreen: bool,
    pub vsync: bool,
    pub target_fps: u32,
    pub physics_timestep: f32,
    pub difficulty: Difficulty,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            title: "Lunaris Action RPG - Dogfooding Demo".to_string(),
            resolution: (1920, 1080),
            fullscreen: false,
            vsync: true,
            target_fps: 60,
            physics_timestep: 1.0 / 60.0,
            difficulty: Difficulty::Normal,
        }
    }
}

/// Difficulty level
#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Nightmare,
}

// ==================== GAME STATE ====================

/// Main game state machine
pub struct ActionRpgDemo {
    pub config: GameConfig,
    pub state: GameState,
    pub world: GameWorld,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub projectiles: Vec<Projectile>,
    pub effects: Vec<VisualEffect>,
    pub ui: GameUI,
    pub audio: AudioState,
    pub input: InputState,
    pub camera: GameCamera,
    pub time: GameTime,
    pub stats: GameStats,
    pub save_system: SaveSystem,
}

/// Game state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    Loading,
    Playing,
    Paused,
    Inventory,
    Dialogue,
    Combat,
    GameOver,
    Victory,
}

impl ActionRpgDemo {
    /// Create new game
    pub fn new() -> Self {
        Self {
            config: GameConfig::default(),
            state: GameState::MainMenu,
            world: GameWorld::new(),
            player: Player::new(),
            enemies: Vec::new(),
            projectiles: Vec::new(),
            effects: Vec::new(),
            ui: GameUI::new(),
            audio: AudioState::new(),
            input: InputState::default(),
            camera: GameCamera::new(),
            time: GameTime::new(),
            stats: GameStats::default(),
            save_system: SaveSystem::new(),
        }
    }

    /// Main update loop
    pub fn update(&mut self, dt: f32) {
        self.time.update(dt);
        self.process_input();

        match self.state {
            GameState::MainMenu => self.update_main_menu(),
            GameState::Loading => self.update_loading(),
            GameState::Playing | GameState::Combat => {
                self.update_gameplay(dt);
            }
            GameState::Paused => {}
            GameState::Inventory => self.update_inventory(),
            GameState::Dialogue => self.update_dialogue(),
            GameState::GameOver => self.update_game_over(),
            GameState::Victory => {}
        }

        self.update_effects(dt);
    }

    fn process_input(&mut self) {
        // Input handling would go here
    }

    fn update_main_menu(&mut self) {
        // Menu logic
    }

    fn update_loading(&mut self) {
        // Loading progress
    }

    fn update_gameplay(&mut self, dt: f32) {
        // Player
        self.player.update(dt, &self.input, &self.world);
        
        // Camera follow
        self.camera.follow(self.player.position, dt);

        // Enemies
        for enemy in &mut self.enemies {
            enemy.update(dt, self.player.position, &self.world);
        }

        // Projectiles
        self.update_projectiles(dt);

        // Combat
        self.process_combat();

        // Check triggers
        self.check_world_triggers();

        // Update stats
        self.stats.time_played += dt;
    }

    fn update_projectiles(&mut self, dt: f32) {
        for proj in &mut self.projectiles {
            proj.position += proj.velocity * dt;
            proj.lifetime -= dt;
        }
        // Remove dead projectiles
        self.projectiles.retain(|p| p.lifetime > 0.0);
    }

    fn process_combat(&mut self) {
        // Player attacks
        if self.player.is_attacking {
            let attack_range = self.player.attack_range;
            for enemy in &mut self.enemies {
                let dist = (enemy.position - self.player.position).length();
                if dist < attack_range && !enemy.is_dead() {
                    let damage = self.player.calculate_damage();
                    enemy.take_damage(damage);
                    self.spawn_damage_number(enemy.position, damage);
                    self.stats.damage_dealt += damage as u64;
                }
            }
        }

        // Enemy attacks
        for enemy in &self.enemies {
            if enemy.is_attacking && !enemy.is_dead() {
                let dist = (self.player.position - enemy.position).length();
                if dist < enemy.attack_range {
                    let damage = enemy.calculate_damage();
                    self.player.take_damage(damage);
                    self.stats.damage_taken += damage as u64;
                }
            }
        }

        // Remove dead enemies
        let prev_count = self.enemies.len();
        self.enemies.retain(|e| !e.is_dead());
        self.stats.enemies_killed += (prev_count - self.enemies.len()) as u64;

        // Check player death
        if self.player.is_dead() {
            self.state = GameState::GameOver;
        }
    }

    fn spawn_damage_number(&mut self, position: Vec3, damage: i32) {
        self.effects.push(VisualEffect {
            effect_type: EffectType::DamageNumber(damage),
            position,
            lifetime: 1.0,
            max_lifetime: 1.0,
        });
    }

    fn check_world_triggers(&mut self) {
        for trigger in &self.world.triggers {
            let dist = (self.player.position.truncate() - trigger.position).length();
            if dist < trigger.radius && !trigger.triggered {
                self.handle_trigger(trigger.action.clone());
            }
        }
    }

    fn handle_trigger(&mut self, action: TriggerAction) {
        match action {
            TriggerAction::SpawnEnemies(count, enemy_type) => {
                for _ in 0..count {
                    self.enemies.push(Enemy::new(enemy_type, self.player.position + Vec3::new(10.0, 0.0, 10.0)));
                }
            }
            TriggerAction::StartDialogue(dialogue_id) => {
                self.ui.start_dialogue(dialogue_id);
                self.state = GameState::Dialogue;
            }
            TriggerAction::OpenDoor(door_id) => {
                self.world.open_door(door_id);
            }
            TriggerAction::PlayCutscene(_) => {}
            TriggerAction::Victory => {
                self.state = GameState::Victory;
            }
        }
    }

    fn update_inventory(&mut self) {
        // Inventory management
    }

    fn update_dialogue(&mut self) {
        if self.ui.dialogue_finished() {
            self.state = GameState::Playing;
        }
    }

    fn update_game_over(&mut self) {
        // Game over screen
    }

    fn update_effects(&mut self, dt: f32) {
        for effect in &mut self.effects {
            effect.lifetime -= dt;
        }
        self.effects.retain(|e| e.lifetime > 0.0);
    }

    /// Start new game
    pub fn new_game(&mut self) {
        self.player = Player::new();
        self.enemies.clear();
        self.projectiles.clear();
        self.effects.clear();
        self.world.load_level("level_1");
        self.stats = GameStats::default();
        self.state = GameState::Playing;
    }

    /// Save game
    pub fn save(&self, slot: u32) {
        self.save_system.save(slot, &self.create_save_data());
    }

    /// Load game
    pub fn load(&mut self, slot: u32) {
        if let Some(data) = self.save_system.load(slot) {
            self.apply_save_data(&data);
        }
    }

    fn create_save_data(&self) -> SaveData {
        SaveData {
            player_position: self.player.position,
            player_hp: self.player.hp,
            player_level: self.player.level,
            player_exp: self.player.experience,
            current_level: self.world.current_level.clone(),
            time_played: self.stats.time_played,
            inventory: self.player.inventory.clone(),
        }
    }

    fn apply_save_data(&mut self, data: &SaveData) {
        self.player.position = data.player_position;
        self.player.hp = data.player_hp;
        self.player.level = data.player_level;
        self.player.experience = data.player_exp;
        self.world.load_level(&data.current_level);
        self.stats.time_played = data.time_played;
        self.player.inventory = data.inventory.clone();
    }
}

// ==================== PLAYER ====================

/// Player character
pub struct Player {
    pub position: Vec3,
    pub velocity: Vec3,
    pub rotation: f32,
    pub hp: i32,
    pub max_hp: i32,
    pub mp: i32,
    pub max_mp: i32,
    pub level: u32,
    pub experience: u32,
    pub stats: PlayerStats,
    pub inventory: Inventory,
    pub equipped: EquippedItems,
    pub abilities: Vec<Ability>,
    pub active_buffs: Vec<Buff>,
    pub is_attacking: bool,
    pub attack_timer: f32,
    pub attack_range: f32,
    pub invincible_timer: f32,
    pub combo_count: u32,
    pub dodge_cooldown: f32,
    pub state: PlayerState,
}

/// Player state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Idle,
    Walking,
    Running,
    Attacking,
    Dodging,
    Casting,
    Stunned,
    Dead,
}

/// Player stats
#[derive(Debug, Clone, Default)]
pub struct PlayerStats {
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub vitality: i32,
    pub luck: i32,
    pub attack_speed: f32,
    pub movement_speed: f32,
    pub critical_chance: f32,
    pub critical_damage: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            rotation: 0.0,
            hp: 100,
            max_hp: 100,
            mp: 50,
            max_mp: 50,
            level: 1,
            experience: 0,
            stats: PlayerStats {
                strength: 10,
                dexterity: 10,
                intelligence: 10,
                vitality: 10,
                luck: 5,
                attack_speed: 1.0,
                movement_speed: 5.0,
                critical_chance: 0.05,
                critical_damage: 1.5,
            },
            inventory: Inventory::new(),
            equipped: EquippedItems::default(),
            abilities: vec![
                Ability::new("Slash", AbilityType::Melee, 0, 20, 0.5),
                Ability::new("Fireball", AbilityType::Projectile, 15, 40, 1.0),
                Ability::new("Heal", AbilityType::Heal, 20, 30, 2.0),
            ],
            active_buffs: Vec::new(),
            is_attacking: false,
            attack_timer: 0.0,
            attack_range: 2.0,
            invincible_timer: 0.0,
            combo_count: 0,
            dodge_cooldown: 0.0,
            state: PlayerState::Idle,
        }
    }

    pub fn update(&mut self, dt: f32, input: &InputState, world: &GameWorld) {
        // Timers
        if self.attack_timer > 0.0 {
            self.attack_timer -= dt;
        } else {
            self.is_attacking = false;
        }
        if self.invincible_timer > 0.0 {
            self.invincible_timer -= dt;
        }
        if self.dodge_cooldown > 0.0 {
            self.dodge_cooldown -= dt;
        }

        // Buff duration
        for buff in &mut self.active_buffs {
            buff.duration -= dt;
        }
        self.active_buffs.retain(|b| b.duration > 0.0);

        // Movement
        let move_dir = Vec3::new(input.move_x, 0.0, input.move_y);
        if move_dir.length_squared() > 0.01 {
            let speed = self.stats.movement_speed * if input.sprint { 1.5 } else { 1.0 };
            self.velocity = move_dir.normalize() * speed;
            self.rotation = move_dir.x.atan2(move_dir.z);
            self.state = if input.sprint { PlayerState::Running } else { PlayerState::Walking };
        } else {
            self.velocity = Vec3::ZERO;
            if self.state != PlayerState::Attacking {
                self.state = PlayerState::Idle;
            }
        }

        // Apply velocity
        let new_pos = self.position + self.velocity * dt;
        if !world.is_collision(new_pos) {
            self.position = new_pos;
        }

        // Attack
        if input.attack && self.attack_timer <= 0.0 {
            self.attack();
        }

        // Dodge
        if input.dodge && self.dodge_cooldown <= 0.0 && self.state != PlayerState::Dodging {
            self.dodge(input);
        }

        // Abilities
        for (i, key) in [input.ability1, input.ability2, input.ability3].iter().enumerate() {
            if *key && i < self.abilities.len() {
                self.use_ability(i);
            }
        }
    }

    fn attack(&mut self) {
        self.is_attacking = true;
        self.attack_timer = 0.5 / self.stats.attack_speed;
        self.state = PlayerState::Attacking;
        self.combo_count = (self.combo_count + 1) % 4;
    }

    fn dodge(&mut self, input: &InputState) {
        self.state = PlayerState::Dodging;
        self.invincible_timer = 0.3;
        self.dodge_cooldown = 1.0;
        self.velocity = Vec3::new(input.move_x, 0.0, input.move_y).normalize() * 15.0;
    }

    fn use_ability(&mut self, index: usize) {
        if let Some(ability) = self.abilities.get(index) {
            if self.mp >= ability.mp_cost && ability.cooldown_timer <= 0.0 {
                self.mp -= ability.mp_cost;
                // Trigger ability effect
            }
        }
    }

    pub fn calculate_damage(&self) -> i32 {
        let base = self.stats.strength * 2;
        let weapon_bonus = self.equipped.weapon.as_ref().map(|w| w.damage).unwrap_or(0);
        let combo_bonus = self.combo_count as i32 * 2;
        
        let mut damage = base + weapon_bonus + combo_bonus;
        
        // Critical hit
        let crit_roll = rand_f32();
        if crit_roll < self.stats.critical_chance {
            damage = (damage as f32 * self.stats.critical_damage) as i32;
        }

        damage
    }

    pub fn take_damage(&mut self, damage: i32) {
        if self.invincible_timer > 0.0 {
            return;
        }

        let defense = self.stats.vitality * 2;
        let actual_damage = (damage - defense / 4).max(1);
        
        self.hp -= actual_damage;
        self.invincible_timer = 0.5;

        if self.hp <= 0 {
            self.state = PlayerState::Dead;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.state == PlayerState::Dead
    }

    pub fn add_experience(&mut self, exp: u32) {
        self.experience += exp;
        let needed = self.experience_for_level(self.level + 1);
        if self.experience >= needed {
            self.level_up();
        }
    }

    fn experience_for_level(&self, level: u32) -> u32 {
        100 * level * level
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.max_hp += 10;
        self.max_mp += 5;
        self.hp = self.max_hp;
        self.mp = self.max_mp;
        self.stats.strength += 2;
        self.stats.dexterity += 2;
        self.stats.vitality += 2;
    }
}

fn rand_f32() -> f32 {
    // Simple placeholder
    0.1
}

// ==================== ENEMY ====================

/// Enemy types
#[derive(Debug, Clone, Copy)]
pub enum EnemyType {
    Goblin,
    Skeleton,
    Orc,
    Demon,
    Dragon,
    Boss,
}

/// Enemy
pub struct Enemy {
    pub id: u64,
    pub enemy_type: EnemyType,
    pub position: Vec3,
    pub velocity: Vec3,
    pub rotation: f32,
    pub hp: i32,
    pub max_hp: i32,
    pub damage: i32,
    pub attack_range: f32,
    pub detection_range: f32,
    pub is_attacking: bool,
    pub attack_timer: f32,
    pub state: EnemyState,
    pub ai: EnemyAI,
    pub drop_table: Vec<(u64, f32)>, // (item_id, drop_chance)
}

/// Enemy state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyState {
    Idle,
    Patrol,
    Chase,
    Attack,
    Stunned,
    Dead,
}

/// Enemy AI
#[derive(Debug, Clone)]
pub struct EnemyAI {
    pub behavior: AIBehavior,
    pub patrol_points: Vec<Vec3>,
    pub current_patrol: usize,
    pub aggro_timer: f32,
    pub last_known_player_pos: Option<Vec3>,
}

/// AI behavior type
#[derive(Debug, Clone, Copy)]
pub enum AIBehavior {
    Passive,
    Aggressive,
    Defensive,
    Support,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, position: Vec3) -> Self {
        let (hp, damage, range) = match enemy_type {
            EnemyType::Goblin => (30, 5, 1.5),
            EnemyType::Skeleton => (40, 8, 2.0),
            EnemyType::Orc => (80, 15, 2.5),
            EnemyType::Demon => (120, 25, 3.0),
            EnemyType::Dragon => (500, 50, 5.0),
            EnemyType::Boss => (1000, 40, 4.0),
        };

        Self {
            id: rand_f32() as u64 * 10000,
            enemy_type,
            position,
            velocity: Vec3::ZERO,
            rotation: 0.0,
            hp,
            max_hp: hp,
            damage,
            attack_range: range,
            detection_range: 15.0,
            is_attacking: false,
            attack_timer: 0.0,
            state: EnemyState::Idle,
            ai: EnemyAI {
                behavior: AIBehavior::Aggressive,
                patrol_points: Vec::new(),
                current_patrol: 0,
                aggro_timer: 0.0,
                last_known_player_pos: None,
            },
            drop_table: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32, player_pos: Vec3, world: &GameWorld) {
        if self.is_dead() {
            return;
        }

        // Attack timer
        if self.attack_timer > 0.0 {
            self.attack_timer -= dt;
        } else {
            self.is_attacking = false;
        }

        // AI
        let dist_to_player = (player_pos - self.position).length();

        match self.state {
            EnemyState::Idle | EnemyState::Patrol => {
                if dist_to_player < self.detection_range {
                    self.state = EnemyState::Chase;
                    self.ai.last_known_player_pos = Some(player_pos);
                }
            }
            EnemyState::Chase => {
                if dist_to_player < self.attack_range {
                    self.state = EnemyState::Attack;
                } else if dist_to_player > self.detection_range * 2.0 {
                    self.state = EnemyState::Idle;
                } else {
                    // Move towards player
                    let dir = (player_pos - self.position).normalize();
                    self.velocity = dir * 3.0;
                    self.rotation = dir.x.atan2(dir.z);
                }
            }
            EnemyState::Attack => {
                if dist_to_player > self.attack_range {
                    self.state = EnemyState::Chase;
                } else if self.attack_timer <= 0.0 {
                    self.attack();
                }
            }
            _ => {}
        }

        // Apply velocity
        let new_pos = self.position + self.velocity * dt;
        if !world.is_collision(new_pos) {
            self.position = new_pos;
        }
        self.velocity *= 0.9; // Friction
    }

    fn attack(&mut self) {
        self.is_attacking = true;
        self.attack_timer = 1.0;
    }

    pub fn calculate_damage(&self) -> i32 {
        self.damage
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
        if self.hp <= 0 {
            self.state = EnemyState::Dead;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.state == EnemyState::Dead
    }
}

// ==================== WORLD ====================

/// Game world
pub struct GameWorld {
    pub current_level: String,
    pub tiles: Vec<Tile>,
    pub width: u32,
    pub height: u32,
    pub triggers: Vec<Trigger>,
    pub doors: HashMap<u64, Door>,
    pub spawn_points: Vec<SpawnPoint>,
}

/// Tile
#[derive(Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub walkable: bool,
}

/// Tile types
#[derive(Debug, Clone, Copy)]
pub enum TileType {
    Floor,
    Wall,
    Water,
    Lava,
    Grass,
    Stone,
}

/// Trigger
#[derive(Debug, Clone)]
pub struct Trigger {
    pub id: u64,
    pub position: Vec2,
    pub radius: f32,
    pub action: TriggerAction,
    pub triggered: bool,
    pub repeatable: bool,
}

/// Trigger action
#[derive(Debug, Clone)]
pub enum TriggerAction {
    SpawnEnemies(u32, EnemyType),
    StartDialogue(u64),
    OpenDoor(u64),
    PlayCutscene(String),
    Victory,
}

/// Door
#[derive(Debug, Clone)]
pub struct Door {
    pub id: u64,
    pub position: Vec3,
    pub is_open: bool,
    pub requires_key: Option<u64>,
}

/// Spawn point
#[derive(Debug, Clone)]
pub struct SpawnPoint {
    pub position: Vec3,
    pub spawn_type: SpawnType,
}

/// Spawn type
#[derive(Debug, Clone)]
pub enum SpawnType {
    Player,
    Enemy(EnemyType),
    Item(u64),
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            current_level: String::new(),
            tiles: Vec::new(),
            width: 0,
            height: 0,
            triggers: Vec::new(),
            doors: HashMap::new(),
            spawn_points: Vec::new(),
        }
    }

    pub fn load_level(&mut self, level_name: &str) {
        self.current_level = level_name.to_string();
        // Would load from file
    }

    pub fn is_collision(&self, pos: Vec3) -> bool {
        let x = pos.x as u32;
        let z = pos.z as u32;
        
        if x >= self.width || z >= self.height {
            return true;
        }

        let idx = (z * self.width + x) as usize;
        if idx < self.tiles.len() {
            !self.tiles[idx].walkable
        } else {
            true
        }
    }

    pub fn open_door(&mut self, door_id: u64) {
        if let Some(door) = self.doors.get_mut(&door_id) {
            door.is_open = true;
        }
    }
}

// ==================== SUPPORTING SYSTEMS ====================

/// Projectile
pub struct Projectile {
    pub position: Vec3,
    pub velocity: Vec3,
    pub damage: i32,
    pub owner_is_player: bool,
    pub lifetime: f32,
}

/// Visual effect
pub struct VisualEffect {
    pub effect_type: EffectType,
    pub position: Vec3,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

/// Effect types
pub enum EffectType {
    DamageNumber(i32),
    Explosion,
    Heal,
    LevelUp,
    ItemPickup,
}

/// Game camera
pub struct GameCamera {
    pub position: Vec3,
    pub target: Vec3,
    pub zoom: f32,
    pub shake_intensity: f32,
    pub shake_timer: f32,
}

impl GameCamera {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 15.0, 10.0),
            target: Vec3::ZERO,
            zoom: 1.0,
            shake_intensity: 0.0,
            shake_timer: 0.0,
        }
    }

    pub fn follow(&mut self, target: Vec3, dt: f32) {
        let lerp_speed = 5.0;
        self.target = self.target + (target - self.target) * lerp_speed * dt;
        self.position = self.target + Vec3::new(0.0, 15.0, 10.0) / self.zoom;
    }

    pub fn shake(&mut self, intensity: f32, duration: f32) {
        self.shake_intensity = intensity;
        self.shake_timer = duration;
    }
}

/// Input state
#[derive(Default)]
pub struct InputState {
    pub move_x: f32,
    pub move_y: f32,
    pub sprint: bool,
    pub attack: bool,
    pub dodge: bool,
    pub ability1: bool,
    pub ability2: bool,
    pub ability3: bool,
    pub interact: bool,
    pub pause: bool,
    pub inventory: bool,
}

/// Game time
pub struct GameTime {
    pub total_time: f32,
    pub delta_time: f32,
    pub time_scale: f32,
    pub frame_count: u64,
}

impl GameTime {
    pub fn new() -> Self {
        Self {
            total_time: 0.0,
            delta_time: 0.0,
            time_scale: 1.0,
            frame_count: 0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.delta_time = dt * self.time_scale;
        self.total_time += self.delta_time;
        self.frame_count += 1;
    }
}

/// Game stats
#[derive(Default)]
pub struct GameStats {
    pub time_played: f32,
    pub enemies_killed: u64,
    pub damage_dealt: u64,
    pub damage_taken: u64,
    pub items_collected: u64,
    pub deaths: u64,
}

/// Ability
pub struct Ability {
    pub name: String,
    pub ability_type: AbilityType,
    pub mp_cost: i32,
    pub damage: i32,
    pub cooldown: f32,
    pub cooldown_timer: f32,
}

impl Ability {
    pub fn new(name: &str, ability_type: AbilityType, mp_cost: i32, damage: i32, cooldown: f32) -> Self {
        Self {
            name: name.to_string(),
            ability_type,
            mp_cost,
            damage,
            cooldown,
            cooldown_timer: 0.0,
        }
    }
}

/// Ability type
#[derive(Debug, Clone, Copy)]
pub enum AbilityType {
    Melee,
    Projectile,
    Area,
    Heal,
    Buff,
}

/// Buff
pub struct Buff {
    pub name: String,
    pub stat: String,
    pub value: i32,
    pub duration: f32,
}

/// Inventory
#[derive(Clone, Default)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub capacity: usize,
    pub gold: u32,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            capacity: 30,
            gold: 0,
        }
    }
}

/// Item
#[derive(Clone)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub item_type: ItemType,
    pub damage: i32,
    pub defense: i32,
    pub value: u32,
}

/// Item type
#[derive(Clone, Copy)]
pub enum ItemType {
    Weapon,
    Armor,
    Consumable,
    Key,
    Quest,
}

/// Equipped items
#[derive(Default)]
pub struct EquippedItems {
    pub weapon: Option<Item>,
    pub armor: Option<Item>,
    pub accessory: Option<Item>,
}

/// Game UI
pub struct GameUI {
    pub current_dialogue: Option<u64>,
    pub dialogue_text: String,
    pub dialogue_options: Vec<String>,
}

impl GameUI {
    pub fn new() -> Self {
        Self {
            current_dialogue: None,
            dialogue_text: String::new(),
            dialogue_options: Vec::new(),
        }
    }

    pub fn start_dialogue(&mut self, dialogue_id: u64) {
        self.current_dialogue = Some(dialogue_id);
    }

    pub fn dialogue_finished(&self) -> bool {
        self.current_dialogue.is_none()
    }
}

/// Audio state
pub struct AudioState {
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub current_music: Option<String>,
}

impl AudioState {
    pub fn new() -> Self {
        Self {
            music_volume: 0.7,
            sfx_volume: 1.0,
            current_music: None,
        }
    }
}

/// Save system
pub struct SaveSystem {
    pub slots: [Option<SaveData>; 3],
}

impl SaveSystem {
    pub fn new() -> Self {
        Self {
            slots: [None, None, None],
        }
    }

    pub fn save(&mut self, slot: u32, data: &SaveData) {
        if (slot as usize) < self.slots.len() {
            self.slots[slot as usize] = Some(data.clone());
        }
    }

    pub fn load(&self, slot: u32) -> Option<SaveData> {
        self.slots.get(slot as usize).cloned().flatten()
    }
}

/// Save data
#[derive(Clone)]
pub struct SaveData {
    pub player_position: Vec3,
    pub player_hp: i32,
    pub player_level: u32,
    pub player_exp: u32,
    pub current_level: String,
    pub time_played: f32,
    pub inventory: Inventory,
}
