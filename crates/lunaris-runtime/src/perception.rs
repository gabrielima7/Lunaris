//! AI Perception System
//!
//! Sight, hearing, and other senses for AI agents.

use glam::Vec3;
use std::collections::HashMap;

/// Perception stimulus type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StimulusType {
    /// Visual detection
    Sight,
    /// Audio detection
    Hearing,
    /// Damage received
    Damage,
    /// Touch/collision
    Touch,
    /// Custom sense
    Custom(u32),
}

/// Stimulus source
#[derive(Debug, Clone)]
pub struct Stimulus {
    /// Unique ID
    pub id: u64,
    /// Type
    pub stimulus_type: StimulusType,
    /// World position
    pub position: Vec3,
    /// Source entity
    pub source_entity: u64,
    /// Strength (0-1)
    pub strength: f32,
    /// Radius
    pub radius: f32,
    /// Age (seconds)
    pub age: f32,
    /// Max age before expiry
    pub max_age: f32,
    /// Tags
    pub tags: Vec<String>,
}

impl Stimulus {
    /// Create a new stimulus
    #[must_use]
    pub fn new(stimulus_type: StimulusType, position: Vec3, source: u64) -> Self {
        Self {
            id: 0,
            stimulus_type,
            position,
            source_entity: source,
            strength: 1.0,
            radius: 10.0,
            age: 0.0,
            max_age: 5.0,
            tags: Vec::new(),
        }
    }

    /// Is expired
    #[must_use]
    pub fn expired(&self) -> bool {
        self.age >= self.max_age
    }

    /// Get strength at distance
    #[must_use]
    pub fn strength_at(&self, distance: f32) -> f32 {
        if distance >= self.radius {
            0.0
        } else {
            self.strength * (1.0 - distance / self.radius)
        }
    }
}

/// Sight sense configuration
#[derive(Debug, Clone)]
pub struct SightConfig {
    /// View distance
    pub range: f32,
    /// Field of view (degrees)
    pub fov: f32,
    /// Peripheral vision range
    pub peripheral_range: f32,
    /// Peripheral FOV
    pub peripheral_fov: f32,
    /// Height for ray origin
    pub eye_height: f32,
    /// Lose sight time (seconds to forget)
    pub lose_sight_time: f32,
}

impl Default for SightConfig {
    fn default() -> Self {
        Self {
            range: 30.0,
            fov: 120.0,
            peripheral_range: 10.0,
            peripheral_fov: 180.0,
            eye_height: 1.7,
            lose_sight_time: 3.0,
        }
    }
}

/// Hearing sense configuration
#[derive(Debug, Clone)]
pub struct HearingConfig {
    /// Hearing range
    pub range: f32,
    /// Threshold (minimum strength to perceive)
    pub threshold: f32,
    /// Directional hearing accuracy
    pub direction_accuracy: f32,
}

impl Default for HearingConfig {
    fn default() -> Self {
        Self {
            range: 50.0,
            threshold: 0.1,
            direction_accuracy: 0.8,
        }
    }
}

/// Perceived target info
#[derive(Debug, Clone)]
pub struct PerceivedTarget {
    /// Entity ID
    pub entity_id: u64,
    /// Last known position
    pub last_position: Vec3,
    /// Last seen time
    pub last_seen: f32,
    /// Is currently visible
    pub visible: bool,
    /// Strength of perception (0-1)
    pub strength: f32,
    /// Detection source
    pub source: StimulusType,
    /// Threat level (0-1)
    pub threat_level: f32,
}

/// AI perception component
pub struct AIPerception {
    /// Owner entity
    pub owner: u64,
    /// Sight configuration
    pub sight: SightConfig,
    /// Hearing configuration
    pub hearing: HearingConfig,
    /// Currently perceived targets
    targets: HashMap<u64, PerceivedTarget>,
    /// Team/faction for friend/foe
    pub team: u32,
    /// Enabled senses
    pub enabled_senses: Vec<StimulusType>,
    /// Forward direction
    pub forward: Vec3,
    /// Eye position
    pub eye_position: Vec3,
}

impl AIPerception {
    /// Create new perception
    #[must_use]
    pub fn new(owner: u64) -> Self {
        Self {
            owner,
            sight: SightConfig::default(),
            hearing: HearingConfig::default(),
            targets: HashMap::new(),
            team: 0,
            enabled_senses: vec![StimulusType::Sight, StimulusType::Hearing],
            forward: Vec3::Z,
            eye_position: Vec3::ZERO,
        }
    }

    /// Update perception
    pub fn update(&mut self, delta_time: f32) {
        // Age all targets
        for target in self.targets.values_mut() {
            if !target.visible {
                target.last_seen += delta_time;
            }
        }

        // Remove forgotten targets
        let lose_time = self.sight.lose_sight_time;
        self.targets.retain(|_, t| t.visible || t.last_seen < lose_time);
    }

    /// Check if a point is in sight
    #[must_use]
    pub fn can_see(&self, target_pos: Vec3) -> bool {
        let to_target = target_pos - self.eye_position;
        let distance = to_target.length();
        
        if distance > self.sight.range {
            return false;
        }

        let direction = to_target.normalize();
        let angle = self.forward.dot(direction).acos().to_degrees();
        
        if angle <= self.sight.fov / 2.0 {
            return true;
        }

        // Check peripheral
        if distance <= self.sight.peripheral_range && angle <= self.sight.peripheral_fov / 2.0 {
            return true;
        }

        false
    }

    /// Process a stimulus
    pub fn process_stimulus(&mut self, stimulus: &Stimulus) {
        if !self.enabled_senses.contains(&stimulus.stimulus_type) {
            return;
        }

        let distance = (stimulus.position - self.eye_position).length();
        let strength = stimulus.strength_at(distance);

        if strength <= 0.0 {
            return;
        }

        match stimulus.stimulus_type {
            StimulusType::Sight => {
                if self.can_see(stimulus.position) {
                    self.add_or_update_target(
                        stimulus.source_entity,
                        stimulus.position,
                        true,
                        strength,
                        StimulusType::Sight,
                    );
                }
            }
            StimulusType::Hearing => {
                if distance <= self.hearing.range && strength >= self.hearing.threshold {
                    self.add_or_update_target(
                        stimulus.source_entity,
                        stimulus.position,
                        false,
                        strength,
                        StimulusType::Hearing,
                    );
                }
            }
            StimulusType::Damage | StimulusType::Touch => {
                self.add_or_update_target(
                    stimulus.source_entity,
                    stimulus.position,
                    true,
                    1.0,
                    stimulus.stimulus_type,
                );
            }
            _ => {}
        }
    }

    fn add_or_update_target(
        &mut self,
        entity_id: u64,
        position: Vec3,
        visible: bool,
        strength: f32,
        source: StimulusType,
    ) {
        let target = self.targets.entry(entity_id).or_insert(PerceivedTarget {
            entity_id,
            last_position: position,
            last_seen: 0.0,
            visible: false,
            strength: 0.0,
            source,
            threat_level: 0.0,
        });

        target.last_position = position;
        target.visible = visible;
        target.strength = strength;
        target.source = source;
        
        if visible {
            target.last_seen = 0.0;
        }
    }

    /// Get all perceived targets
    #[must_use]
    pub fn get_targets(&self) -> Vec<&PerceivedTarget> {
        self.targets.values().collect()
    }

    /// Get visible targets
    #[must_use]
    pub fn get_visible_targets(&self) -> Vec<&PerceivedTarget> {
        self.targets.values().filter(|t| t.visible).collect()
    }

    /// Get highest threat
    #[must_use]
    pub fn get_highest_threat(&self) -> Option<&PerceivedTarget> {
        self.targets.values()
            .filter(|t| t.visible)
            .max_by(|a, b| a.threat_level.partial_cmp(&b.threat_level).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Get nearest target
    #[must_use]
    pub fn get_nearest(&self) -> Option<&PerceivedTarget> {
        self.targets.values()
            .filter(|t| t.visible)
            .min_by(|a, b| {
                let da = (a.last_position - self.eye_position).length_squared();
                let db = (b.last_position - self.eye_position).length_squared();
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Check if knows about entity
    #[must_use]
    pub fn knows(&self, entity_id: u64) -> bool {
        self.targets.contains_key(&entity_id)
    }

    /// Forget entity
    pub fn forget(&mut self, entity_id: u64) {
        self.targets.remove(&entity_id);
    }

    /// Clear all targets
    pub fn clear(&mut self) {
        self.targets.clear();
    }
}

/// Perception system (manages all perception components)
pub struct PerceptionSystem {
    /// Active stimuli
    stimuli: Vec<Stimulus>,
    /// Next stimulus ID
    next_id: u64,
    /// Max stimuli
    pub max_stimuli: usize,
}

impl Default for PerceptionSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl PerceptionSystem {
    /// Create a new perception system
    #[must_use]
    pub fn new() -> Self {
        Self {
            stimuli: Vec::new(),
            next_id: 1,
            max_stimuli: 1000,
        }
    }

    /// Report a stimulus
    pub fn report(&mut self, mut stimulus: Stimulus) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        stimulus.id = id;
        self.stimuli.push(stimulus);

        // Limit stimuli count
        while self.stimuli.len() > self.max_stimuli {
            self.stimuli.remove(0);
        }

        id
    }

    /// Report noise at position
    pub fn report_noise(&mut self, position: Vec3, source: u64, radius: f32, strength: f32) {
        let mut stim = Stimulus::new(StimulusType::Hearing, position, source);
        stim.radius = radius;
        stim.strength = strength;
        self.report(stim);
    }

    /// Update all stimuli
    pub fn update(&mut self, delta_time: f32) {
        for stimulus in &mut self.stimuli {
            stimulus.age += delta_time;
        }

        // Remove expired
        self.stimuli.retain(|s| !s.expired());
    }

    /// Get stimuli in range
    #[must_use]
    pub fn get_stimuli_in_range(&self, position: Vec3, range: f32) -> Vec<&Stimulus> {
        self.stimuli.iter()
            .filter(|s| (s.position - position).length() <= range + s.radius)
            .collect()
    }

    /// Update perception component with active stimuli
    pub fn process_for(&self, perception: &mut AIPerception) {
        let range = perception.sight.range.max(perception.hearing.range);
        let stimuli = self.get_stimuli_in_range(perception.eye_position, range);
        
        for stimulus in stimuli {
            perception.process_stimulus(stimulus);
        }
    }

    /// Get active stimuli count
    #[must_use]
    pub fn stimuli_count(&self) -> usize {
        self.stimuli.len()
    }
}
