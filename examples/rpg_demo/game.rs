//! RPG Demo - Complete Showcase Game
//!
//! Demonstrates all Lunaris Engine features in a polished RPG experience.

use glam::{Vec2, Vec3, Quat};
use std::collections::HashMap;

// ==================== GAME STATE ====================

/// Main RPG game state
pub struct RpgDemo {
    /// Player character
    pub player: Character,
    /// Party members
    pub party: Vec<Character>,
    /// World map
    pub world: World,
    /// Current location
    pub location: Location,
    /// Quest log
    pub quests: QuestLog,
    /// Inventory
    pub inventory: Inventory,
    /// Dialogue system
    pub dialogue: DialogueSystem,
    /// Combat system
    pub combat: Option<CombatSystem>,
    /// Game time
    pub time: GameTime,
    /// Save data
    pub save: SaveData,
    /// UI state
    pub ui_state: UiState,
}

impl RpgDemo {
    /// Create new game
    pub fn new() -> Self {
        let player = Character::new_player("Hero", CharacterClass::Warrior);
        
        Self {
            player,
            party: Vec::new(),
            world: World::new(),
            location: Location::Village("Oakwood".to_string()),
            quests: QuestLog::new(),
            inventory: Inventory::new(50),
            dialogue: DialogueSystem::new(),
            combat: None,
            time: GameTime::new(),
            save: SaveData::default(),
            ui_state: UiState::default(),
        }
    }

    /// Update game
    pub fn update(&mut self, dt: f32) {
        self.time.update(dt);
        
        // Update based on current state
        if let Some(ref mut combat) = self.combat {
            combat.update(dt);
        } else {
            self.update_exploration(dt);
        }
    }

    fn update_exploration(&mut self, dt: f32) {
        // NPC interactions, etc.
    }

    /// Save game
    pub fn save_game(&self, slot: u32) {
        // Would serialize to file
    }

    /// Load game
    pub fn load_game(&mut self, slot: u32) {
        // Would deserialize from file
    }
}

// ==================== CHARACTER SYSTEM ====================

/// Character class
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterClass {
    Warrior,
    Mage,
    Rogue,
    Cleric,
    Ranger,
    Paladin,
}

impl CharacterClass {
    /// Get base stats for class
    pub fn base_stats(&self) -> CharacterStats {
        match self {
            Self::Warrior => CharacterStats {
                max_hp: 120, max_mp: 30,
                strength: 15, defense: 12, magic: 5, speed: 8, luck: 8,
                ..Default::default()
            },
            Self::Mage => CharacterStats {
                max_hp: 70, max_mp: 100,
                strength: 5, defense: 6, magic: 18, speed: 10, luck: 10,
                ..Default::default()
            },
            Self::Rogue => CharacterStats {
                max_hp: 85, max_mp: 40,
                strength: 10, defense: 8, magic: 6, speed: 16, luck: 14,
                ..Default::default()
            },
            Self::Cleric => CharacterStats {
                max_hp: 90, max_mp: 80,
                strength: 8, defense: 10, magic: 14, speed: 8, luck: 10,
                ..Default::default()
            },
            Self::Ranger => CharacterStats {
                max_hp: 95, max_mp: 50,
                strength: 12, defense: 8, magic: 8, speed: 14, luck: 12,
                ..Default::default()
            },
            Self::Paladin => CharacterStats {
                max_hp: 110, max_mp: 60,
                strength: 12, defense: 14, magic: 10, speed: 6, luck: 8,
                ..Default::default()
            },
        }
    }
}

/// Character stats
#[derive(Debug, Clone, Default)]
pub struct CharacterStats {
    pub max_hp: i32,
    pub max_mp: i32,
    pub strength: i32,
    pub defense: i32,
    pub magic: i32,
    pub speed: i32,
    pub luck: i32,
    pub critical_chance: f32,
    pub evasion: f32,
}

/// Character
#[derive(Debug, Clone)]
pub struct Character {
    /// Name
    pub name: String,
    /// Class
    pub class: CharacterClass,
    /// Level
    pub level: u32,
    /// Experience
    pub experience: u32,
    /// Current HP
    pub hp: i32,
    /// Current MP
    pub mp: i32,
    /// Base stats
    pub base_stats: CharacterStats,
    /// Equipment bonuses
    pub equipment_bonus: CharacterStats,
    /// Buff effects
    pub buffs: Vec<Buff>,
    /// Debuff effects
    pub debuffs: Vec<Debuff>,
    /// Equipped items
    pub equipment: Equipment,
    /// Known skills
    pub skills: Vec<Skill>,
    /// Is player controlled
    pub is_player: bool,
    /// Portrait
    pub portrait: String,
}

impl Character {
    /// Create new player character
    pub fn new_player(name: &str, class: CharacterClass) -> Self {
        let stats = class.base_stats();
        Self {
            name: name.to_string(),
            class,
            level: 1,
            experience: 0,
            hp: stats.max_hp,
            mp: stats.max_mp,
            base_stats: stats,
            equipment_bonus: CharacterStats::default(),
            buffs: Vec::new(),
            debuffs: Vec::new(),
            equipment: Equipment::default(),
            skills: Self::starting_skills(class),
            is_player: true,
            portrait: format!("{:?}_portrait", class).to_lowercase(),
        }
    }

    /// Create NPC/Enemy
    pub fn new_npc(name: &str, class: CharacterClass, level: u32) -> Self {
        let mut stats = class.base_stats();
        // Scale stats with level
        let level_mult = 1.0 + (level as f32 - 1.0) * 0.15;
        stats.max_hp = (stats.max_hp as f32 * level_mult) as i32;
        stats.max_mp = (stats.max_mp as f32 * level_mult) as i32;
        stats.strength = (stats.strength as f32 * level_mult) as i32;
        stats.defense = (stats.defense as f32 * level_mult) as i32;
        stats.magic = (stats.magic as f32 * level_mult) as i32;

        Self {
            name: name.to_string(),
            class,
            level,
            experience: 0,
            hp: stats.max_hp,
            mp: stats.max_mp,
            base_stats: stats,
            equipment_bonus: CharacterStats::default(),
            buffs: Vec::new(),
            debuffs: Vec::new(),
            equipment: Equipment::default(),
            skills: Self::starting_skills(class),
            is_player: false,
            portrait: String::new(),
        }
    }

    fn starting_skills(class: CharacterClass) -> Vec<Skill> {
        match class {
            CharacterClass::Warrior => vec![
                Skill::new("Slash", SkillType::Physical, 0, 15),
                Skill::new("Power Strike", SkillType::Physical, 5, 30),
            ],
            CharacterClass::Mage => vec![
                Skill::new("Fireball", SkillType::Fire, 10, 25),
                Skill::new("Ice Shard", SkillType::Ice, 8, 20),
            ],
            CharacterClass::Rogue => vec![
                Skill::new("Backstab", SkillType::Physical, 0, 25),
                Skill::new("Poison Blade", SkillType::Poison, 5, 15),
            ],
            CharacterClass::Cleric => vec![
                Skill::new("Heal", SkillType::Healing, 10, 30),
                Skill::new("Smite", SkillType::Holy, 8, 20),
            ],
            CharacterClass::Ranger => vec![
                Skill::new("Arrow Shot", SkillType::Physical, 0, 18),
                Skill::new("Multishot", SkillType::Physical, 12, 25),
            ],
            CharacterClass::Paladin => vec![
                Skill::new("Holy Strike", SkillType::Holy, 8, 25),
                Skill::new("Lay on Hands", SkillType::Healing, 15, 40),
            ],
        }
    }

    /// Get total stats (base + equipment + buffs)
    pub fn total_stats(&self) -> CharacterStats {
        let mut stats = self.base_stats.clone();
        
        // Add equipment bonuses
        stats.max_hp += self.equipment_bonus.max_hp;
        stats.max_mp += self.equipment_bonus.max_mp;
        stats.strength += self.equipment_bonus.strength;
        stats.defense += self.equipment_bonus.defense;
        stats.magic += self.equipment_bonus.magic;
        stats.speed += self.equipment_bonus.speed;
        stats.luck += self.equipment_bonus.luck;

        // Apply buffs
        for buff in &self.buffs {
            match buff.stat {
                BuffStat::Strength => stats.strength += buff.amount,
                BuffStat::Defense => stats.defense += buff.amount,
                BuffStat::Magic => stats.magic += buff.amount,
                BuffStat::Speed => stats.speed += buff.amount,
                _ => {}
            }
        }

        stats
    }

    /// Add experience and check for level up
    pub fn add_experience(&mut self, exp: u32) -> bool {
        self.experience += exp;
        let exp_needed = self.experience_for_next_level();
        
        if self.experience >= exp_needed {
            self.level_up();
            true
        } else {
            false
        }
    }

    fn experience_for_next_level(&self) -> u32 {
        100 * self.level * self.level
    }

    fn level_up(&mut self) {
        self.level += 1;
        
        // Increase stats based on class
        let growth = match self.class {
            CharacterClass::Warrior => (15, 3, 4, 3, 1, 2),
            CharacterClass::Mage => (8, 12, 1, 2, 5, 3),
            CharacterClass::Rogue => (10, 5, 3, 2, 2, 4),
            CharacterClass::Cleric => (12, 10, 2, 3, 4, 2),
            CharacterClass::Ranger => (11, 6, 3, 2, 2, 4),
            CharacterClass::Paladin => (13, 8, 3, 4, 3, 2),
        };

        self.base_stats.max_hp += growth.0;
        self.base_stats.max_mp += growth.1;
        self.base_stats.strength += growth.2;
        self.base_stats.defense += growth.3;
        self.base_stats.magic += growth.4;
        self.base_stats.speed += growth.5;

        // Restore HP/MP on level up
        self.hp = self.base_stats.max_hp;
        self.mp = self.base_stats.max_mp;
    }

    /// Is alive
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Take damage
    pub fn take_damage(&mut self, amount: i32) {
        self.hp = (self.hp - amount).max(0);
    }

    /// Heal
    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.base_stats.max_hp);
    }
}

/// Equipment slots
#[derive(Debug, Clone, Default)]
pub struct Equipment {
    pub weapon: Option<Item>,
    pub shield: Option<Item>,
    pub head: Option<Item>,
    pub body: Option<Item>,
    pub hands: Option<Item>,
    pub feet: Option<Item>,
    pub accessory1: Option<Item>,
    pub accessory2: Option<Item>,
}

/// Buff effect
#[derive(Debug, Clone)]
pub struct Buff {
    pub name: String,
    pub stat: BuffStat,
    pub amount: i32,
    pub duration: f32,
    pub remaining: f32,
}

/// Debuff effect
#[derive(Debug, Clone)]
pub struct Debuff {
    pub name: String,
    pub effect: DebuffEffect,
    pub duration: f32,
    pub remaining: f32,
}

/// Buff stat type
#[derive(Debug, Clone, Copy)]
pub enum BuffStat {
    Strength,
    Defense,
    Magic,
    Speed,
    CritChance,
    Evasion,
}

/// Debuff effect type
#[derive(Debug, Clone, Copy)]
pub enum DebuffEffect {
    Poison(i32),      // Damage per tick
    Burn(i32),
    Freeze,           // Skip turn
    Stun,
    Blind,            // Reduced accuracy
    Silence,          // Can't use magic
}

// ==================== SKILL SYSTEM ====================

/// Skill
#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub skill_type: SkillType,
    pub mp_cost: i32,
    pub base_power: i32,
    pub target: SkillTarget,
    pub description: String,
}

impl Skill {
    pub fn new(name: &str, skill_type: SkillType, mp_cost: i32, power: i32) -> Self {
        Self {
            name: name.to_string(),
            skill_type,
            mp_cost,
            base_power: power,
            target: SkillTarget::SingleEnemy,
            description: String::new(),
        }
    }
}

/// Skill type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillType {
    Physical,
    Fire,
    Ice,
    Lightning,
    Poison,
    Holy,
    Dark,
    Healing,
    Buff,
    Debuff,
}

/// Skill target
#[derive(Debug, Clone, Copy)]
pub enum SkillTarget {
    Self_,
    SingleAlly,
    AllAllies,
    SingleEnemy,
    AllEnemies,
    All,
}

// ==================== INVENTORY SYSTEM ====================

/// Item
#[derive(Debug, Clone)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub item_type: ItemType,
    pub rarity: Rarity,
    pub description: String,
    pub stats: ItemStats,
    pub value: u32,
    pub stackable: bool,
    pub max_stack: u32,
}

/// Item type
#[derive(Debug, Clone, Copy)]
pub enum ItemType {
    Weapon(WeaponType),
    Armor(ArmorSlot),
    Consumable,
    KeyItem,
    Material,
}

/// Weapon type
#[derive(Debug, Clone, Copy)]
pub enum WeaponType {
    Sword,
    Axe,
    Mace,
    Staff,
    Bow,
    Dagger,
}

/// Armor slot
#[derive(Debug, Clone, Copy)]
pub enum ArmorSlot {
    Head,
    Body,
    Hands,
    Feet,
    Shield,
    Accessory,
}

/// Rarity
#[derive(Debug, Clone, Copy)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl Rarity {
    pub fn color(&self) -> [f32; 4] {
        match self {
            Self::Common => [0.7, 0.7, 0.7, 1.0],
            Self::Uncommon => [0.3, 0.8, 0.3, 1.0],
            Self::Rare => [0.3, 0.5, 1.0, 1.0],
            Self::Epic => [0.7, 0.3, 0.9, 1.0],
            Self::Legendary => [1.0, 0.6, 0.0, 1.0],
        }
    }
}

/// Item stats
#[derive(Debug, Clone, Default)]
pub struct ItemStats {
    pub attack: i32,
    pub defense: i32,
    pub magic: i32,
    pub hp: i32,
    pub mp: i32,
    pub speed: i32,
}

/// Inventory
#[derive(Debug, Clone)]
pub struct Inventory {
    pub items: Vec<InventorySlot>,
    pub capacity: usize,
    pub gold: u32,
}

/// Inventory slot
#[derive(Debug, Clone)]
pub struct InventorySlot {
    pub item: Item,
    pub quantity: u32,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::new(),
            capacity,
            gold: 100,
        }
    }

    pub fn add_item(&mut self, item: Item, quantity: u32) -> bool {
        // Check if item already exists and is stackable
        if item.stackable {
            for slot in &mut self.items {
                if slot.item.id == item.id {
                    slot.quantity = (slot.quantity + quantity).min(item.max_stack);
                    return true;
                }
            }
        }

        // Add new slot
        if self.items.len() < self.capacity {
            self.items.push(InventorySlot { item, quantity });
            true
        } else {
            false // Inventory full
        }
    }

    pub fn remove_item(&mut self, item_id: u64, quantity: u32) -> bool {
        if let Some(idx) = self.items.iter().position(|s| s.item.id == item_id) {
            if self.items[idx].quantity <= quantity {
                self.items.remove(idx);
            } else {
                self.items[idx].quantity -= quantity;
            }
            true
        } else {
            false
        }
    }
}

// ==================== QUEST SYSTEM ====================

/// Quest log
#[derive(Debug, Clone)]
pub struct QuestLog {
    pub active: Vec<Quest>,
    pub completed: Vec<Quest>,
    pub failed: Vec<Quest>,
}

impl QuestLog {
    pub fn new() -> Self {
        Self {
            active: Vec::new(),
            completed: Vec::new(),
            failed: Vec::new(),
        }
    }

    pub fn add_quest(&mut self, quest: Quest) {
        self.active.push(quest);
    }

    pub fn complete_quest(&mut self, quest_id: u64) {
        if let Some(idx) = self.active.iter().position(|q| q.id == quest_id) {
            let mut quest = self.active.remove(idx);
            quest.status = QuestStatus::Completed;
            self.completed.push(quest);
        }
    }
}

/// Quest
#[derive(Debug, Clone)]
pub struct Quest {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub objectives: Vec<QuestObjective>,
    pub rewards: QuestRewards,
    pub status: QuestStatus,
    pub giver: String,
    pub chain_next: Option<u64>,
}

/// Quest objective
#[derive(Debug, Clone)]
pub struct QuestObjective {
    pub description: String,
    pub objective_type: QuestObjectiveType,
    pub current: u32,
    pub required: u32,
    pub complete: bool,
}

/// Quest objective type
#[derive(Debug, Clone)]
pub enum QuestObjectiveType {
    Kill(String),
    Collect(u64),
    Talk(String),
    Visit(String),
    Escort(String),
    Protect(String),
}

/// Quest rewards
#[derive(Debug, Clone)]
pub struct QuestRewards {
    pub experience: u32,
    pub gold: u32,
    pub items: Vec<u64>,
    pub reputation: HashMap<String, i32>,
}

/// Quest status
#[derive(Debug, Clone, Copy)]
pub enum QuestStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
}

// ==================== WORLD SYSTEM ====================

/// World
#[derive(Debug, Clone)]
pub struct World {
    pub regions: HashMap<String, Region>,
    pub current_region: String,
}

impl World {
    pub fn new() -> Self {
        let mut regions = HashMap::new();
        
        // Create starting region
        regions.insert("grasslands".to_string(), Region {
            name: "Emerald Grasslands".to_string(),
            description: "Rolling green hills dotted with wildflowers.".to_string(),
            locations: vec![
                "Oakwood Village".to_string(),
                "Ancient Ruins".to_string(),
                "Forest Edge".to_string(),
            ],
            enemies: vec!["Slime".to_string(), "Goblin".to_string()],
            level_range: (1, 5),
        });

        Self {
            regions,
            current_region: "grasslands".to_string(),
        }
    }
}

/// Region
#[derive(Debug, Clone)]
pub struct Region {
    pub name: String,
    pub description: String,
    pub locations: Vec<String>,
    pub enemies: Vec<String>,
    pub level_range: (u32, u32),
}

/// Location type
#[derive(Debug, Clone)]
pub enum Location {
    Village(String),
    Dungeon(String),
    Field(String),
    Town(String),
}

// ==================== COMBAT SYSTEM ====================

/// Combat system
#[derive(Debug, Clone)]
pub struct CombatSystem {
    /// Player party
    pub party: Vec<Character>,
    /// Enemies
    pub enemies: Vec<Character>,
    /// Turn order
    pub turn_order: Vec<CombatantRef>,
    /// Current turn index
    pub current_turn: usize,
    /// Combat phase
    pub phase: CombatPhase,
    /// Combat log
    pub log: Vec<String>,
    /// Is player's turn
    pub player_turn: bool,
}

/// Combatant reference
#[derive(Debug, Clone)]
pub struct CombatantRef {
    pub is_party: bool,
    pub index: usize,
}

/// Combat phase
#[derive(Debug, Clone, Copy)]
pub enum CombatPhase {
    Starting,
    TurnStart,
    SelectAction,
    ExecuteAction,
    TurnEnd,
    Victory,
    Defeat,
}

impl CombatSystem {
    pub fn new(party: Vec<Character>, enemies: Vec<Character>) -> Self {
        let mut combat = Self {
            party,
            enemies,
            turn_order: Vec::new(),
            current_turn: 0,
            phase: CombatPhase::Starting,
            log: Vec::new(),
            player_turn: true,
        };
        combat.calculate_turn_order();
        combat
    }

    fn calculate_turn_order(&mut self) {
        self.turn_order.clear();
        
        // Add all combatants with their speed
        let mut all: Vec<(CombatantRef, i32)> = Vec::new();
        
        for (i, c) in self.party.iter().enumerate() {
            if c.is_alive() {
                all.push((CombatantRef { is_party: true, index: i }, c.total_stats().speed));
            }
        }
        
        for (i, c) in self.enemies.iter().enumerate() {
            if c.is_alive() {
                all.push((CombatantRef { is_party: false, index: i }, c.total_stats().speed));
            }
        }
        
        // Sort by speed (descending)
        all.sort_by(|a, b| b.1.cmp(&a.1));
        
        self.turn_order = all.into_iter().map(|(r, _)| r).collect();
    }

    pub fn update(&mut self, _dt: f32) {
        match self.phase {
            CombatPhase::Starting => {
                self.log.push("Combat started!".to_string());
                self.phase = CombatPhase::TurnStart;
            }
            CombatPhase::TurnStart => {
                self.phase = CombatPhase::SelectAction;
            }
            CombatPhase::SelectAction => {
                // Wait for input
            }
            CombatPhase::ExecuteAction => {
                self.phase = CombatPhase::TurnEnd;
            }
            CombatPhase::TurnEnd => {
                self.current_turn = (self.current_turn + 1) % self.turn_order.len();
                self.check_end_conditions();
            }
            CombatPhase::Victory | CombatPhase::Defeat => {
                // Combat ended
            }
        }
    }

    pub fn execute_attack(&mut self, attacker_ref: &CombatantRef, target_ref: &CombatantRef) {
        let attacker = if attacker_ref.is_party {
            &self.party[attacker_ref.index]
        } else {
            &self.enemies[attacker_ref.index]
        };

        let stats = attacker.total_stats();
        let base_damage = stats.strength;

        let target = if target_ref.is_party {
            &mut self.party[target_ref.index]
        } else {
            &mut self.enemies[target_ref.index]
        };

        let target_stats = target.total_stats();
        let damage = (base_damage - target_stats.defense / 2).max(1);

        self.log.push(format!("{} attacks {} for {} damage!", 
            attacker.name, target.name, damage));
        
        target.take_damage(damage);

        if !target.is_alive() {
            self.log.push(format!("{} was defeated!", target.name));
        }

        self.phase = CombatPhase::ExecuteAction;
    }

    fn check_end_conditions(&mut self) {
        let party_alive = self.party.iter().any(|c| c.is_alive());
        let enemies_alive = self.enemies.iter().any(|c| c.is_alive());

        if !party_alive {
            self.phase = CombatPhase::Defeat;
            self.log.push("Defeat...".to_string());
        } else if !enemies_alive {
            self.phase = CombatPhase::Victory;
            self.log.push("Victory!".to_string());
        } else {
            self.phase = CombatPhase::TurnStart;
        }
    }

    pub fn is_over(&self) -> bool {
        matches!(self.phase, CombatPhase::Victory | CombatPhase::Defeat)
    }
}

// ==================== DIALOGUE SYSTEM ====================

/// Dialogue system
#[derive(Debug, Clone)]
pub struct DialogueSystem {
    pub current: Option<Dialogue>,
    pub history: Vec<String>,
}

impl DialogueSystem {
    pub fn new() -> Self {
        Self {
            current: None,
            history: Vec::new(),
        }
    }

    pub fn start(&mut self, dialogue: Dialogue) {
        self.current = Some(dialogue);
    }

    pub fn select_option(&mut self, index: usize) {
        if let Some(ref mut dialogue) = self.current {
            if let Some(node) = dialogue.nodes.get(&dialogue.current_node) {
                if let Some(choice) = node.choices.get(index) {
                    dialogue.current_node = choice.next_node;
                }
            }
        }
    }
}

/// Dialogue
#[derive(Debug, Clone)]
pub struct Dialogue {
    pub nodes: HashMap<u64, DialogueNode>,
    pub current_node: u64,
}

/// Dialogue node
#[derive(Debug, Clone)]
pub struct DialogueNode {
    pub speaker: String,
    pub text: String,
    pub choices: Vec<DialogueChoice>,
    pub portrait: Option<String>,
}

/// Dialogue choice
#[derive(Debug, Clone)]
pub struct DialogueChoice {
    pub text: String,
    pub next_node: u64,
    pub condition: Option<String>,
}

// ==================== TIME SYSTEM ====================

/// Game time
#[derive(Debug, Clone)]
pub struct GameTime {
    pub day: u32,
    pub hour: f32,
    pub minute: f32,
    pub time_scale: f32,
}

impl GameTime {
    pub fn new() -> Self {
        Self {
            day: 1,
            hour: 8.0,
            minute: 0.0,
            time_scale: 60.0, // 1 real second = 1 game minute
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.minute += dt * self.time_scale;
        
        while self.minute >= 60.0 {
            self.minute -= 60.0;
            self.hour += 1.0;
        }
        
        while self.hour >= 24.0 {
            self.hour -= 24.0;
            self.day += 1;
        }
    }

    pub fn is_day(&self) -> bool {
        self.hour >= 6.0 && self.hour < 20.0
    }

    pub fn formatted(&self) -> String {
        format!("Day {} - {:02}:{:02}", self.day, self.hour as u32, self.minute as u32)
    }
}

// ==================== SAVE DATA ====================

/// Save data
#[derive(Debug, Clone, Default)]
pub struct SaveData {
    pub slot: u32,
    pub play_time: f32,
    pub timestamp: String,
}

/// UI State
#[derive(Debug, Clone, Default)]
pub struct UiState {
    pub menu_open: bool,
    pub inventory_open: bool,
    pub map_open: bool,
    pub quest_log_open: bool,
}
