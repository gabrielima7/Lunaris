//! Cognitive NPC System
//!
//! NPCs with dynamic dialogue using local LLMs.

use std::collections::HashMap;
use glam::Vec3;

/// NPC personality traits
#[derive(Debug, Clone)]
pub struct Personality {
    /// Friendliness (0-1)
    pub friendliness: f32,
    /// Aggression (0-1)
    pub aggression: f32,
    /// Intelligence (0-1)
    pub intelligence: f32,
    /// Humor (0-1)
    pub humor: f32,
    /// Formality (0-1)
    pub formality: f32,
    /// Curiosity (0-1)
    pub curiosity: f32,
    /// Custom traits
    pub custom: HashMap<String, f32>,
}

impl Default for Personality {
    fn default() -> Self {
        Self {
            friendliness: 0.5,
            aggression: 0.2,
            intelligence: 0.5,
            humor: 0.3,
            formality: 0.5,
            curiosity: 0.5,
            custom: HashMap::new(),
        }
    }
}

impl Personality {
    /// Create friendly personality
    #[must_use]
    pub fn friendly() -> Self {
        Self {
            friendliness: 0.9,
            aggression: 0.1,
            humor: 0.6,
            formality: 0.3,
            ..Default::default()
        }
    }

    /// Create hostile personality
    #[must_use]
    pub fn hostile() -> Self {
        Self {
            friendliness: 0.1,
            aggression: 0.9,
            humor: 0.0,
            formality: 0.2,
            ..Default::default()
        }
    }

    /// Create merchant personality
    #[must_use]
    pub fn merchant() -> Self {
        Self {
            friendliness: 0.7,
            aggression: 0.0,
            intelligence: 0.7,
            humor: 0.4,
            formality: 0.6,
            curiosity: 0.3,
            ..Default::default()
        }
    }
}

/// NPC memory entry
#[derive(Debug, Clone)]
pub struct Memory {
    /// Memory content
    pub content: String,
    /// Importance (0-1)
    pub importance: f32,
    /// Timestamp (game time)
    pub timestamp: f64,
    /// Related entity
    pub entity_id: Option<u64>,
    /// Emotional valence (-1 to 1)
    pub valence: f32,
}

/// NPC emotional state
#[derive(Debug, Clone, Copy)]
pub struct Emotion {
    /// Joy (-1 to 1)
    pub joy: f32,
    /// Fear (0-1)
    pub fear: f32,
    /// Anger (0-1)
    pub anger: f32,
    /// Sadness (0-1)
    pub sadness: f32,
    /// Surprise (0-1)
    pub surprise: f32,
    /// Trust (0-1)
    pub trust: f32,
}

impl Default for Emotion {
    fn default() -> Self {
        Self {
            joy: 0.0,
            fear: 0.0,
            anger: 0.0,
            sadness: 0.0,
            surprise: 0.0,
            trust: 0.5,
        }
    }
}

impl Emotion {
    /// Get dominant emotion
    #[must_use]
    pub fn dominant(&self) -> &str {
        let emotions = [
            ("joy", self.joy.abs()),
            ("fear", self.fear),
            ("anger", self.anger),
            ("sadness", self.sadness),
            ("surprise", self.surprise),
        ];
        
        emotions.iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(name, _)| *name)
            .unwrap_or("neutral")
    }

    /// Apply decay over time
    pub fn decay(&mut self, dt: f32) {
        let decay_rate = 0.1 * dt;
        self.joy *= 1.0 - decay_rate;
        self.fear *= 1.0 - decay_rate;
        self.anger *= 1.0 - decay_rate;
        self.sadness *= 1.0 - decay_rate;
        self.surprise *= 1.0 - decay_rate * 2.0; // Surprise fades faster
    }
}

/// Relationship with another entity
#[derive(Debug, Clone)]
pub struct Relationship {
    /// Target entity ID
    pub target: u64,
    /// Affinity (-1 to 1)
    pub affinity: f32,
    /// Trust (0-1)
    pub trust: f32,
    /// Familiarity (0-1)
    pub familiarity: f32,
    /// Interaction count
    pub interactions: u32,
    /// Last interaction time
    pub last_interaction: f64,
}

/// Dialogue context
#[derive(Debug, Clone)]
pub struct DialogueContext {
    /// Current topic
    pub topic: String,
    /// Player intent
    pub player_intent: PlayerIntent,
    /// Recent keywords
    pub keywords: Vec<String>,
    /// Quest-related
    pub quest_id: Option<u64>,
    /// Trade-related
    pub in_trade: bool,
}

/// Player intent
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerIntent {
    Greeting,
    Question,
    Trade,
    Quest,
    Threat,
    Compliment,
    Insult,
    Farewell,
    Unknown,
}

/// Dialogue response
#[derive(Debug, Clone)]
pub struct DialogueResponse {
    /// Response text
    pub text: String,
    /// Emotional tone
    pub tone: String,
    /// Actions to trigger
    pub actions: Vec<NPCAction>,
    /// Available player responses
    pub player_options: Vec<String>,
    /// Should end dialogue
    pub end_dialogue: bool,
}

/// NPC action
#[derive(Debug, Clone)]
pub enum NPCAction {
    /// Play animation
    PlayAnimation(String),
    /// Look at target
    LookAt(Vec3),
    /// Give item
    GiveItem { item_id: u64, count: u32 },
    /// Open trade
    OpenTrade,
    /// Start quest
    StartQuest(u64),
    /// Attack
    Attack,
    /// Flee
    Flee,
    /// Follow player
    Follow,
    /// Custom action
    Custom(String),
}

/// Cognitive NPC
pub struct CognitiveNPC {
    /// Entity ID
    pub entity_id: u64,
    /// NPC name
    pub name: String,
    /// Description/backstory
    pub backstory: String,
    /// Occupation
    pub occupation: String,
    /// Personality
    pub personality: Personality,
    /// Current emotion
    pub emotion: Emotion,
    /// Memories
    memories: Vec<Memory>,
    /// Relationships
    relationships: HashMap<u64, Relationship>,
    /// Knowledge base
    knowledge: HashMap<String, String>,
    /// Conversation history
    conversation_history: Vec<(String, String)>,
    /// LLM system prompt
    system_prompt: String,
    /// Max memory entries
    max_memories: usize,
}

impl CognitiveNPC {
    /// Create new cognitive NPC
    #[must_use]
    pub fn new(entity_id: u64, name: &str, backstory: &str, occupation: &str) -> Self {
        let mut npc = Self {
            entity_id,
            name: name.to_string(),
            backstory: backstory.to_string(),
            occupation: occupation.to_string(),
            personality: Personality::default(),
            emotion: Emotion::default(),
            memories: Vec::new(),
            relationships: HashMap::new(),
            knowledge: HashMap::new(),
            conversation_history: Vec::new(),
            system_prompt: String::new(),
            max_memories: 100,
        };
        npc.update_system_prompt();
        npc
    }

    /// Set personality
    pub fn with_personality(mut self, personality: Personality) -> Self {
        self.personality = personality;
        self.update_system_prompt();
        self
    }

    /// Add knowledge
    pub fn add_knowledge(&mut self, topic: &str, info: &str) {
        self.knowledge.insert(topic.to_string(), info.to_string());
        self.update_system_prompt();
    }

    /// Add memory
    pub fn add_memory(&mut self, content: &str, importance: f32, valence: f32) {
        let memory = Memory {
            content: content.to_string(),
            importance: importance.clamp(0.0, 1.0),
            timestamp: 0.0, // Would use game time
            entity_id: None,
            valence: valence.clamp(-1.0, 1.0),
        };
        
        self.memories.push(memory);
        
        // Prune old unimportant memories
        if self.memories.len() > self.max_memories {
            self.memories.sort_by(|a, b| {
                b.importance.partial_cmp(&a.importance).unwrap_or(std::cmp::Ordering::Equal)
            });
            self.memories.truncate(self.max_memories);
        }
    }

    /// Get or create relationship
    pub fn get_relationship(&mut self, entity_id: u64) -> &mut Relationship {
        self.relationships.entry(entity_id).or_insert(Relationship {
            target: entity_id,
            affinity: 0.0,
            trust: 0.3,
            familiarity: 0.0,
            interactions: 0,
            last_interaction: 0.0,
        })
    }

    /// Modify relationship
    pub fn modify_relationship(&mut self, entity_id: u64, affinity_delta: f32, trust_delta: f32) {
        let rel = self.get_relationship(entity_id);
        rel.affinity = (rel.affinity + affinity_delta).clamp(-1.0, 1.0);
        rel.trust = (rel.trust + trust_delta).clamp(0.0, 1.0);
        rel.familiarity = (rel.familiarity + 0.1).min(1.0);
        rel.interactions += 1;
    }

    /// Generate response to player input
    pub fn respond(&mut self, player_input: &str, context: &DialogueContext) -> DialogueResponse {
        // Store conversation
        self.conversation_history.push(("Player".to_string(), player_input.to_string()));

        // Analyze player intent from input
        let intent = self.analyze_intent(player_input);

        // Generate response based on personality, emotion, and context
        let response = self.generate_response(player_input, &intent, context);

        // Update emotion based on interaction
        self.update_emotion_from_input(player_input, &intent);

        // Store response
        self.conversation_history.push((self.name.clone(), response.text.clone()));

        response
    }

    fn analyze_intent(&self, input: &str) -> PlayerIntent {
        let lower = input.to_lowercase();
        
        if lower.contains("hello") || lower.contains("hi") || lower.contains("hey") {
            PlayerIntent::Greeting
        } else if lower.contains("bye") || lower.contains("goodbye") || lower.contains("farewell") {
            PlayerIntent::Farewell
        } else if lower.contains("?") || lower.contains("what") || lower.contains("where") || lower.contains("how") {
            PlayerIntent::Question
        } else if lower.contains("buy") || lower.contains("sell") || lower.contains("trade") {
            PlayerIntent::Trade
        } else if lower.contains("quest") || lower.contains("mission") || lower.contains("task") {
            PlayerIntent::Quest
        } else if lower.contains("die") || lower.contains("kill") || lower.contains("attack") {
            PlayerIntent::Threat
        } else if lower.contains("nice") || lower.contains("good") || lower.contains("beautiful") {
            PlayerIntent::Compliment
        } else if lower.contains("ugly") || lower.contains("stupid") || lower.contains("hate") {
            PlayerIntent::Insult
        } else {
            PlayerIntent::Unknown
        }
    }

    fn generate_response(&self, _input: &str, intent: &PlayerIntent, context: &DialogueContext) -> DialogueResponse {
        // Would call LLM API here
        // For now, use template responses based on personality and intent

        let tone = self.emotion.dominant().to_string();
        let mut actions = Vec::new();
        let mut player_options = Vec::new();
        let mut end_dialogue = false;

        let text = match intent {
            PlayerIntent::Greeting => {
                if self.personality.friendliness > 0.5 {
                    player_options.push("How are you?".to_string());
                    player_options.push("What do you do here?".to_string());
                    format!("Hello there, traveler! Welcome to our {}. I'm {}, the local {}.", 
                        "village", self.name, self.occupation)
                } else {
                    player_options.push("I'll be going.".to_string());
                    format!("*nods* What do you want?")
                }
            }
            PlayerIntent::Farewell => {
                end_dialogue = true;
                if self.personality.friendliness > 0.5 {
                    "Safe travels, friend! Come back anytime.".to_string()
                } else {
                    "Hmph. Goodbye.".to_string()
                }
            }
            PlayerIntent::Trade => {
                if self.occupation.to_lowercase().contains("merchant") {
                    actions.push(NPCAction::OpenTrade);
                    player_options.push("Show me your wares.".to_string());
                    player_options.push("Maybe later.".to_string());
                    "Ah, looking to do some business? I have fine goods for sale!".to_string()
                } else {
                    "I'm not a merchant, but you might try the shop down the road.".to_string()
                }
            }
            PlayerIntent::Quest => {
                player_options.push("Tell me more.".to_string());
                player_options.push("Not interested.".to_string());
                "Actually, there is something you could help with...".to_string()
            }
            PlayerIntent::Threat => {
                self.emotion.anger * 0.5;
                if self.personality.aggression > 0.5 {
                    actions.push(NPCAction::Attack);
                    "You dare threaten me?! Guards!".to_string()
                } else {
                    actions.push(NPCAction::Flee);
                    "Please, I don't want any trouble!".to_string()
                }
            }
            PlayerIntent::Compliment => {
                player_options.push("You're welcome.".to_string());
                if self.personality.humor > 0.5 {
                    "*blushes* Oh stop it, you're making me embarrassed!".to_string()
                } else {
                    "Thank you, that's very kind.".to_string()
                }
            }
            PlayerIntent::Insult => {
                if self.personality.aggression > 0.7 {
                    actions.push(NPCAction::Attack);
                    "How dare you!".to_string()
                } else {
                    end_dialogue = true;
                    "I don't have to listen to this. Good day.".to_string()
                }
            }
            PlayerIntent::Question => {
                // Check knowledge base
                if let Some(answer) = self.knowledge.get(&context.topic) {
                    player_options.push("Interesting...".to_string());
                    answer.clone()
                } else {
                    "Hmm, I'm not sure about that. You might ask someone else.".to_string()
                }
            }
            PlayerIntent::Unknown => {
                player_options.push("Nevermind.".to_string());
                "I see... Is there something specific you'd like to know?".to_string()
            }
        };

        if player_options.is_empty() && !end_dialogue {
            player_options.push("Continue...".to_string());
            player_options.push("Goodbye.".to_string());
        }

        DialogueResponse {
            text,
            tone,
            actions,
            player_options,
            end_dialogue,
        }
    }

    fn update_emotion_from_input(&mut self, _input: &str, intent: &PlayerIntent) {
        match intent {
            PlayerIntent::Compliment => {
                self.emotion.joy += 0.2;
            }
            PlayerIntent::Insult => {
                if self.personality.aggression > 0.5 {
                    self.emotion.anger += 0.3;
                } else {
                    self.emotion.sadness += 0.2;
                }
            }
            PlayerIntent::Threat => {
                if self.personality.aggression > 0.5 {
                    self.emotion.anger += 0.4;
                } else {
                    self.emotion.fear += 0.4;
                }
            }
            PlayerIntent::Greeting => {
                self.emotion.surprise += 0.1;
            }
            _ => {}
        }
    }

    fn update_system_prompt(&mut self) {
        let traits = format!(
            "Friendliness: {:.0}%, Aggression: {:.0}%, Humor: {:.0}%, Formality: {:.0}%",
            self.personality.friendliness * 100.0,
            self.personality.aggression * 100.0,
            self.personality.humor * 100.0,
            self.personality.formality * 100.0
        );

        let knowledge_str: String = self.knowledge.iter()
            .map(|(k, v)| format!("- {}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\n");

        self.system_prompt = format!(
            r#"You are {}, a {} in a fantasy world.

Backstory: {}

Personality traits: {}

Knowledge:
{}

Respond in character based on your personality and the conversation context.
Keep responses concise (1-3 sentences unless explaining something important)."#,
            self.name,
            self.occupation,
            self.backstory,
            traits,
            knowledge_str
        );
    }

    /// Get conversation history
    #[must_use]
    pub fn conversation_history(&self) -> &[(String, String)] {
        &self.conversation_history
    }

    /// Clear conversation
    pub fn clear_conversation(&mut self) {
        self.conversation_history.clear();
    }

    /// Update (call every frame)
    pub fn update(&mut self, dt: f32) {
        self.emotion.decay(dt);
    }
}

/// NPC Manager for managing multiple cognitive NPCs
pub struct NPCManager {
    /// NPCs
    npcs: HashMap<u64, CognitiveNPC>,
    /// Next ID
    next_id: u64,
}

impl Default for NPCManager {
    fn default() -> Self {
        Self::new()
    }
}

impl NPCManager {
    /// Create new manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            npcs: HashMap::new(),
            next_id: 1,
        }
    }

    /// Create NPC
    pub fn create(&mut self, name: &str, backstory: &str, occupation: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let npc = CognitiveNPC::new(id, name, backstory, occupation);
        self.npcs.insert(id, npc);
        id
    }

    /// Get NPC
    #[must_use]
    pub fn get(&self, id: u64) -> Option<&CognitiveNPC> {
        self.npcs.get(&id)
    }

    /// Get NPC mutably
    pub fn get_mut(&mut self, id: u64) -> Option<&mut CognitiveNPC> {
        self.npcs.get_mut(&id)
    }

    /// Update all NPCs
    pub fn update(&mut self, dt: f32) {
        for npc in self.npcs.values_mut() {
            npc.update(dt);
        }
    }
}
