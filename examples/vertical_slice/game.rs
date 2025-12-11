//! Vertical Slice Demo
//!
//! A complete mini-game demonstrating all Lunaris Engine features.

use glam::{Vec3, Vec2, Quat};
use std::collections::HashMap;

/// Game state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    Loading,
    Playing,
    Paused,
    Cutscene,
    GameOver,
    Victory,
}

/// Demo game configuration
#[derive(Debug, Clone)]
pub struct DemoConfig {
    /// World size
    pub world_size: Vec2,
    /// Player start position
    pub player_start: Vec3,
    /// Enemy count
    pub enemy_count: u32,
    /// Enable day/night cycle
    pub day_night_cycle: bool,
    /// Day length in seconds
    pub day_length: f32,
    /// Enable weather
    pub weather: bool,
    /// Enable AI companions
    pub companions: bool,
    /// Difficulty
    pub difficulty: Difficulty,
}

impl Default for DemoConfig {
    fn default() -> Self {
        Self {
            world_size: Vec2::new(500.0, 500.0),
            player_start: Vec3::new(0.0, 1.0, 0.0),
            enemy_count: 10,
            day_night_cycle: true,
            day_length: 600.0, // 10 minutes
            weather: true,
            companions: false,
            difficulty: Difficulty::Normal,
        }
    }
}

/// Difficulty setting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Nightmare,
}

/// Player data
#[derive(Debug, Clone)]
pub struct Player {
    /// Entity ID
    pub entity_id: u64,
    /// Position
    pub position: Vec3,
    /// Rotation
    pub rotation: Quat,
    /// Velocity
    pub velocity: Vec3,
    /// Health
    pub health: f32,
    /// Max health
    pub max_health: f32,
    /// Stamina
    pub stamina: f32,
    /// Max stamina
    pub max_stamina: f32,
    /// Is grounded
    pub grounded: bool,
    /// Is sprinting
    pub sprinting: bool,
    /// Current weapon
    pub weapon: WeaponType,
    /// Inventory
    pub inventory: Vec<Item>,
    /// Experience
    pub experience: u32,
    /// Level
    pub level: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            entity_id: 1,
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            velocity: Vec3::ZERO,
            health: 100.0,
            max_health: 100.0,
            stamina: 100.0,
            max_stamina: 100.0,
            grounded: true,
            sprinting: false,
            weapon: WeaponType::Sword,
            inventory: Vec::new(),
            experience: 0,
            level: 1,
        }
    }
}

impl Player {
    /// Take damage
    pub fn take_damage(&mut self, amount: f32) {
        self.health = (self.health - amount).max(0.0);
    }

    /// Heal
    pub fn heal(&mut self, amount: f32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    /// Use stamina
    pub fn use_stamina(&mut self, amount: f32) -> bool {
        if self.stamina >= amount {
            self.stamina -= amount;
            true
        } else {
            false
        }
    }

    /// Is dead
    #[must_use]
    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    /// Add experience
    pub fn add_experience(&mut self, xp: u32) {
        self.experience += xp;
        
        // Level up check
        let xp_needed = self.level * 100;
        while self.experience >= xp_needed {
            self.experience -= xp_needed;
            self.level += 1;
            self.max_health += 10.0;
            self.health = self.max_health;
            self.max_stamina += 5.0;
        }
    }
}

/// Weapon type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponType {
    Unarmed,
    Sword,
    Bow,
    Staff,
    Axe,
}

impl WeaponType {
    /// Get damage
    #[must_use]
    pub fn damage(&self) -> f32 {
        match self {
            Self::Unarmed => 5.0,
            Self::Sword => 20.0,
            Self::Bow => 15.0,
            Self::Staff => 25.0,
            Self::Axe => 30.0,
        }
    }

    /// Get attack speed
    #[must_use]
    pub fn speed(&self) -> f32 {
        match self {
            Self::Unarmed => 0.5,
            Self::Sword => 0.8,
            Self::Bow => 1.2,
            Self::Staff => 1.0,
            Self::Axe => 1.5,
        }
    }
}

/// Item
#[derive(Debug, Clone)]
pub struct Item {
    /// Item ID
    pub id: u64,
    /// Name
    pub name: String,
    /// Type
    pub item_type: ItemType,
    /// Stack count
    pub count: u32,
    /// Is equipped
    pub equipped: bool,
}

/// Item type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Weapon,
    Armor,
    Consumable,
    Key,
    Quest,
    Material,
}

/// Enemy data
#[derive(Debug, Clone)]
pub struct Enemy {
    /// Entity ID
    pub entity_id: u64,
    /// Position
    pub position: Vec3,
    /// Rotation
    pub rotation: Quat,
    /// Enemy type
    pub enemy_type: EnemyType,
    /// Health
    pub health: f32,
    /// Max health
    pub max_health: f32,
    /// Is active
    pub active: bool,
    /// AI state
    pub ai_state: EnemyAIState,
    /// Target (player entity ID)
    pub target: Option<u64>,
    /// Patrol points
    pub patrol: Vec<Vec3>,
    /// Current patrol index
    pub patrol_index: usize,
}

/// Enemy type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Goblin,
    Skeleton,
    Orc,
    Dragon,
    Boss,
}

impl EnemyType {
    /// Get stats
    #[must_use]
    pub fn stats(&self) -> (f32, f32, f32) {
        // (health, damage, speed)
        match self {
            Self::Goblin => (30.0, 5.0, 5.0),
            Self::Skeleton => (40.0, 10.0, 3.0),
            Self::Orc => (80.0, 20.0, 4.0),
            Self::Dragon => (200.0, 40.0, 8.0),
            Self::Boss => (500.0, 50.0, 3.0),
        }
    }

    /// Get XP reward
    #[must_use]
    pub fn xp_reward(&self) -> u32 {
        match self {
            Self::Goblin => 10,
            Self::Skeleton => 15,
            Self::Orc => 30,
            Self::Dragon => 100,
            Self::Boss => 500,
        }
    }
}

/// Enemy AI state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyAIState {
    Idle,
    Patrol,
    Chase,
    Attack,
    Flee,
    Dead,
}

/// World environment
#[derive(Debug, Clone)]
pub struct Environment {
    /// Time of day (0-24)
    pub time_of_day: f32,
    /// Sun direction
    pub sun_direction: Vec3,
    /// Sun color
    pub sun_color: Vec3,
    /// Ambient color
    pub ambient_color: Vec3,
    /// Fog density
    pub fog_density: f32,
    /// Weather
    pub weather: Weather,
    /// Wind direction
    pub wind: Vec3,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            time_of_day: 12.0, // Noon
            sun_direction: Vec3::new(-0.5, -1.0, -0.3).normalize(),
            sun_color: Vec3::new(1.0, 0.95, 0.9),
            ambient_color: Vec3::new(0.2, 0.25, 0.3),
            fog_density: 0.01,
            weather: Weather::Clear,
            wind: Vec3::new(1.0, 0.0, 0.5),
        }
    }
}

impl Environment {
    /// Update time of day
    pub fn update(&mut self, dt: f32, day_length: f32) {
        self.time_of_day += (24.0 / day_length) * dt;
        if self.time_of_day >= 24.0 {
            self.time_of_day -= 24.0;
        }

        // Update sun
        let angle = (self.time_of_day / 24.0) * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
        self.sun_direction = Vec3::new(angle.cos(), -angle.sin().abs(), 0.3).normalize();

        // Update colors based on time
        let brightness = if self.time_of_day < 6.0 || self.time_of_day > 20.0 {
            0.1 // Night
        } else if self.time_of_day < 8.0 {
            0.1 + (self.time_of_day - 6.0) / 2.0 * 0.9 // Sunrise
        } else if self.time_of_day > 18.0 {
            1.0 - (self.time_of_day - 18.0) / 2.0 * 0.9 // Sunset
        } else {
            1.0 // Day
        };

        self.sun_color = Vec3::new(1.0, 0.95 * brightness + 0.3, 0.9 * brightness + 0.1);
        self.ambient_color = Vec3::splat(0.1 + 0.15 * brightness);
    }

    /// Is night
    #[must_use]
    pub fn is_night(&self) -> bool {
        self.time_of_day < 6.0 || self.time_of_day > 20.0
    }
}

/// Weather type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weather {
    Clear,
    Cloudy,
    Rain,
    Storm,
    Snow,
    Fog,
}

/// Quest
#[derive(Debug, Clone)]
pub struct Quest {
    /// Quest ID
    pub id: u64,
    /// Name
    pub name: String,
    /// Description
    pub description: String,
    /// Objectives
    pub objectives: Vec<QuestObjective>,
    /// Is completed
    pub completed: bool,
    /// Rewards
    pub rewards: QuestRewards,
}

/// Quest objective
#[derive(Debug, Clone)]
pub struct QuestObjective {
    /// Description
    pub description: String,
    /// Type
    pub objective_type: ObjectiveType,
    /// Current progress
    pub current: u32,
    /// Required count
    pub required: u32,
    /// Is complete
    pub complete: bool,
}

/// Objective type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectiveType {
    Kill,
    Collect,
    Talk,
    Discover,
    Escort,
    Survive,
}

/// Quest rewards
#[derive(Debug, Clone)]
pub struct QuestRewards {
    pub experience: u32,
    pub gold: u32,
    pub items: Vec<u64>,
}

/// The demo game
pub struct VerticalSlice {
    /// Config
    pub config: DemoConfig,
    /// Current state
    pub state: GameState,
    /// Player
    pub player: Player,
    /// Enemies
    pub enemies: Vec<Enemy>,
    /// Environment
    pub environment: Environment,
    /// Quests
    pub quests: Vec<Quest>,
    /// Active quest
    pub active_quest: Option<u64>,
    /// Game time
    pub game_time: f32,
    /// Score
    pub score: u32,
    /// Kills
    pub kills: u32,
    /// Next entity ID
    next_entity_id: u64,
}

impl Default for VerticalSlice {
    fn default() -> Self {
        Self::new(DemoConfig::default())
    }
}

impl VerticalSlice {
    /// Create new demo
    #[must_use]
    pub fn new(config: DemoConfig) -> Self {
        let mut game = Self {
            config: config.clone(),
            state: GameState::MainMenu,
            player: Player {
                position: config.player_start,
                ..Default::default()
            },
            enemies: Vec::new(),
            environment: Environment::default(),
            quests: Vec::new(),
            active_quest: None,
            game_time: 0.0,
            score: 0,
            kills: 0,
            next_entity_id: 100,
        };

        game.spawn_enemies();
        game.create_main_quest();
        game
    }

    fn spawn_enemies(&mut self) {
        for i in 0..self.config.enemy_count {
            let angle = (i as f32 / self.config.enemy_count as f32) * std::f32::consts::TAU;
            let distance = 30.0 + (i as f32 * 5.0);
            
            let position = Vec3::new(
                angle.cos() * distance,
                0.0,
                angle.sin() * distance,
            );

            let enemy_type = match i % 5 {
                0 => EnemyType::Goblin,
                1 => EnemyType::Skeleton,
                2 => EnemyType::Orc,
                3 if i > self.config.enemy_count / 2 => EnemyType::Dragon,
                _ => EnemyType::Goblin,
            };

            let (health, _, _) = enemy_type.stats();

            let enemy = Enemy {
                entity_id: self.next_entity_id,
                position,
                rotation: Quat::IDENTITY,
                enemy_type,
                health,
                max_health: health,
                active: true,
                ai_state: EnemyAIState::Patrol,
                target: None,
                patrol: vec![
                    position,
                    position + Vec3::new(10.0, 0.0, 0.0),
                    position + Vec3::new(10.0, 0.0, 10.0),
                    position + Vec3::new(0.0, 0.0, 10.0),
                ],
                patrol_index: 0,
            };

            self.enemies.push(enemy);
            self.next_entity_id += 1;
        }

        // Add boss
        let boss = Enemy {
            entity_id: self.next_entity_id,
            position: Vec3::new(0.0, 0.0, 100.0),
            rotation: Quat::IDENTITY,
            enemy_type: EnemyType::Boss,
            health: 500.0,
            max_health: 500.0,
            active: true,
            ai_state: EnemyAIState::Idle,
            target: None,
            patrol: vec![],
            patrol_index: 0,
        };
        self.enemies.push(boss);
        self.next_entity_id += 1;
    }

    fn create_main_quest(&mut self) {
        let quest = Quest {
            id: 1,
            name: "The Final Battle".to_string(),
            description: "Defeat the enemies and slay the boss to save the realm.".to_string(),
            objectives: vec![
                QuestObjective {
                    description: "Defeat 5 enemies".to_string(),
                    objective_type: ObjectiveType::Kill,
                    current: 0,
                    required: 5,
                    complete: false,
                },
                QuestObjective {
                    description: "Defeat the Boss".to_string(),
                    objective_type: ObjectiveType::Kill,
                    current: 0,
                    required: 1,
                    complete: false,
                },
            ],
            completed: false,
            rewards: QuestRewards {
                experience: 1000,
                gold: 500,
                items: vec![],
            },
        };

        self.quests.push(quest);
        self.active_quest = Some(1);
    }

    /// Start game
    pub fn start(&mut self) {
        self.state = GameState::Playing;
        self.game_time = 0.0;
    }

    /// Pause game
    pub fn pause(&mut self) {
        if self.state == GameState::Playing {
            self.state = GameState::Paused;
        }
    }

    /// Resume game
    pub fn resume(&mut self) {
        if self.state == GameState::Paused {
            self.state = GameState::Playing;
        }
    }

    /// Update game
    pub fn update(&mut self, dt: f32) {
        if self.state != GameState::Playing {
            return;
        }

        self.game_time += dt;

        // Update environment
        if self.config.day_night_cycle {
            self.environment.update(dt, self.config.day_length);
        }

        // Regenerate stamina
        self.player.stamina = (self.player.stamina + 10.0 * dt).min(self.player.max_stamina);

        // Update enemies
        self.update_enemies(dt);

        // Check game over
        if self.player.is_dead() {
            self.state = GameState::GameOver;
        }

        // Check victory
        if self.quests.iter().all(|q| q.completed) {
            self.state = GameState::Victory;
        }
    }

    fn update_enemies(&mut self, dt: f32) {
        let player_pos = self.player.position;
        
        for enemy in &mut self.enemies {
            if !enemy.active || enemy.health <= 0.0 {
                enemy.ai_state = EnemyAIState::Dead;
                continue;
            }

            let to_player = player_pos - enemy.position;
            let distance = to_player.length();
            let (_, damage, speed) = enemy.enemy_type.stats();

            match enemy.ai_state {
                EnemyAIState::Idle | EnemyAIState::Patrol => {
                    // Check if player is near
                    if distance < 15.0 {
                        enemy.ai_state = EnemyAIState::Chase;
                        enemy.target = Some(self.player.entity_id);
                    } else if !enemy.patrol.is_empty() {
                        // Patrol
                        let target = enemy.patrol[enemy.patrol_index];
                        let to_target = target - enemy.position;
                        if to_target.length() < 1.0 {
                            enemy.patrol_index = (enemy.patrol_index + 1) % enemy.patrol.len();
                        } else {
                            enemy.position += to_target.normalize() * speed * 0.5 * dt;
                        }
                    }
                }
                EnemyAIState::Chase => {
                    if distance < 2.0 {
                        enemy.ai_state = EnemyAIState::Attack;
                    } else if distance > 30.0 {
                        enemy.ai_state = EnemyAIState::Patrol;
                        enemy.target = None;
                    } else {
                        enemy.position += to_player.normalize() * speed * dt;
                    }
                }
                EnemyAIState::Attack => {
                    if distance > 3.0 {
                        enemy.ai_state = EnemyAIState::Chase;
                    }
                    // Damage is applied externally via combat system
                }
                EnemyAIState::Flee => {
                    enemy.position -= to_player.normalize() * speed * dt;
                    if distance > 50.0 {
                        enemy.ai_state = EnemyAIState::Patrol;
                    }
                }
                EnemyAIState::Dead => {}
            }
        }
    }

    /// Player attack
    pub fn player_attack(&mut self) {
        let attack_range = 3.0;
        let damage = self.player.weapon.damage();

        for enemy in &mut self.enemies {
            if enemy.health <= 0.0 {
                continue;
            }

            let distance = (enemy.position - self.player.position).length();
            if distance <= attack_range {
                enemy.health -= damage;
                
                if enemy.health <= 0.0 {
                    enemy.active = false;
                    self.kills += 1;
                    self.score += enemy.enemy_type.xp_reward();
                    self.player.add_experience(enemy.enemy_type.xp_reward());

                    // Update quest
                    self.update_quest_progress(enemy.enemy_type);
                }
            }
        }
    }

    fn update_quest_progress(&mut self, enemy_type: EnemyType) {
        for quest in &mut self.quests {
            for objective in &mut quest.objectives {
                if objective.objective_type == ObjectiveType::Kill && !objective.complete {
                    objective.current += 1;
                    if objective.current >= objective.required {
                        objective.complete = true;
                    }
                }
            }

            // Check if all objectives complete
            if quest.objectives.iter().all(|o| o.complete) {
                quest.completed = true;
                self.player.add_experience(quest.rewards.experience);
            }
        }
    }

    /// Get stats
    #[must_use]
    pub fn stats(&self) -> DemoStats {
        DemoStats {
            game_time: self.game_time,
            player_level: self.player.level,
            player_health: self.player.health,
            kills: self.kills,
            score: self.score,
            enemies_remaining: self.enemies.iter().filter(|e| e.health > 0.0).count(),
            quests_completed: self.quests.iter().filter(|q| q.completed).count(),
        }
    }
}

/// Demo statistics
#[derive(Debug, Clone)]
pub struct DemoStats {
    pub game_time: f32,
    pub player_level: u32,
    pub player_health: f32,
    pub kills: u32,
    pub score: u32,
    pub enemies_remaining: usize,
    pub quests_completed: usize,
}
