//! Achievement System
//!
//! Platform achievements and progress tracking.

use std::collections::HashMap;

/// Achievement system
pub struct AchievementSystem {
    pub achievements: Vec<Achievement>,
    pub progress: HashMap<String, AchievementProgress>,
    pub platform: Platform,
    pub unlocked_count: u32,
}

/// Achievement
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub icon_locked: String,
    pub hidden: bool,
    pub points: u32,
    pub achievement_type: AchievementType,
    pub platform_ids: PlatformIds,
}

/// Achievement type
pub enum AchievementType {
    Standard,
    Progressive { target: u32 },
    Tiered { tiers: Vec<(u32, String)> },
}

/// Platform IDs
pub struct PlatformIds {
    pub steam: Option<String>,
    pub psn: Option<String>,
    pub xbox: Option<String>,
    pub nintendo: Option<String>,
    pub epic: Option<String>,
    pub gog: Option<String>,
}

/// Progress
#[derive(Clone)]
pub struct AchievementProgress {
    pub unlocked: bool,
    pub unlock_time: Option<u64>,
    pub current: u32,
    pub target: u32,
}

/// Platform
#[derive(Clone, Copy)]
pub enum Platform { Steam, PlayStation, Xbox, Nintendo, Epic, GOG, Offline }

impl AchievementSystem {
    pub fn new(platform: Platform) -> Self {
        Self { achievements: Vec::new(), progress: HashMap::new(), platform, unlocked_count: 0 }
    }

    pub fn register(&mut self, achievement: Achievement) {
        let progress = match &achievement.achievement_type {
            AchievementType::Standard => AchievementProgress { unlocked: false, unlock_time: None, current: 0, target: 1 },
            AchievementType::Progressive { target } => AchievementProgress { unlocked: false, unlock_time: None, current: 0, target: *target },
            AchievementType::Tiered { tiers } => AchievementProgress { unlocked: false, unlock_time: None, current: 0, target: tiers.last().map(|t| t.0).unwrap_or(1) },
        };
        self.progress.insert(achievement.id.clone(), progress);
        self.achievements.push(achievement);
    }

    pub fn unlock(&mut self, id: &str) -> bool {
        if let Some(progress) = self.progress.get_mut(id) {
            if progress.unlocked { return false; }
            progress.unlocked = true;
            progress.unlock_time = Some(0);
            progress.current = progress.target;
            self.unlocked_count += 1;
            self.sync_platform(id);
            return true;
        }
        false
    }

    pub fn add_progress(&mut self, id: &str, amount: u32) -> bool {
        if let Some(progress) = self.progress.get_mut(id) {
            if progress.unlocked { return false; }
            progress.current = (progress.current + amount).min(progress.target);
            if progress.current >= progress.target {
                return self.unlock(id);
            }
            self.sync_progress(id);
        }
        false
    }

    pub fn set_progress(&mut self, id: &str, value: u32) -> bool {
        if let Some(progress) = self.progress.get_mut(id) {
            if progress.unlocked { return false; }
            progress.current = value.min(progress.target);
            if progress.current >= progress.target {
                return self.unlock(id);
            }
            self.sync_progress(id);
        }
        false
    }

    fn sync_platform(&self, id: &str) {
        // Would call platform SDK
        match self.platform {
            Platform::Steam => { /* SteamAPI::SetAchievement */ }
            Platform::PlayStation => { /* Trophies API */ }
            Platform::Xbox => { /* Xbox Achievements API */ }
            _ => {}
        }
    }

    fn sync_progress(&self, id: &str) {
        // Would update platform progress indicator
    }

    pub fn get_completion(&self) -> f32 {
        if self.achievements.is_empty() { 0.0 }
        else { self.unlocked_count as f32 / self.achievements.len() as f32 }
    }

    pub fn is_unlocked(&self, id: &str) -> bool {
        self.progress.get(id).map(|p| p.unlocked).unwrap_or(false)
    }
}

impl Default for PlatformIds {
    fn default() -> Self {
        Self { steam: None, psn: None, xbox: None, nintendo: None, epic: None, gog: None }
    }
}
