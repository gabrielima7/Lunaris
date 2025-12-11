//! Stable API and LTS Support
//!
//! Long-term support guarantees and API stability markers.

use std::collections::HashMap;

/// API stability level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stability {
    /// Stable - will not break in minor versions
    Stable,
    /// Beta - may change, but with deprecation warnings
    Beta,
    /// Experimental - may change without notice
    Experimental,
    /// Deprecated - will be removed in future version
    Deprecated,
    /// Internal - not for public use
    Internal,
}

/// API version info
#[derive(Debug, Clone)]
pub struct ApiVersion {
    /// Engine version
    pub engine_version: String,
    /// API version (may differ from engine)
    pub api_version: String,
    /// Minimum compatible version
    pub min_compatible: String,
    /// LTS until date
    pub lts_until: Option<String>,
    /// Deprecation warnings
    pub deprecations: Vec<Deprecation>,
}

impl Default for ApiVersion {
    fn default() -> Self {
        Self {
            engine_version: env!("CARGO_PKG_VERSION").to_string(),
            api_version: "1.0.0".to_string(),
            min_compatible: "0.1.0".to_string(),
            lts_until: Some("2027-12-31".to_string()),
            deprecations: Vec::new(),
        }
    }
}

/// Deprecation notice
#[derive(Debug, Clone)]
pub struct Deprecation {
    /// API item
    pub item: String,
    /// Since version
    pub since: String,
    /// Removal version
    pub removal: String,
    /// Replacement API
    pub replacement: Option<String>,
    /// Migration guide
    pub migration: Option<String>,
}

/// API documentation entry
#[derive(Debug, Clone)]
pub struct ApiDoc {
    /// Module path
    pub module: String,
    /// Item name
    pub name: String,
    /// Item type
    pub item_type: ApiItemType,
    /// Stability
    pub stability: Stability,
    /// Since version
    pub since: String,
    /// Description
    pub description: String,
    /// Example code
    pub example: Option<String>,
    /// Related items
    pub see_also: Vec<String>,
}

/// API item type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiItemType {
    Module,
    Struct,
    Enum,
    Trait,
    Function,
    Constant,
    Macro,
}

/// API registry for documentation
pub struct ApiRegistry {
    /// All API entries
    entries: HashMap<String, ApiDoc>,
    /// Version info
    pub version: ApiVersion,
    /// Categories
    categories: HashMap<String, Vec<String>>,
}

impl Default for ApiRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiRegistry {
    /// Create new registry
    #[must_use]
    pub fn new() -> Self {
        let mut registry = Self {
            entries: HashMap::new(),
            version: ApiVersion::default(),
            categories: HashMap::new(),
        };
        registry.register_core_apis();
        registry
    }

    fn register_core_apis(&mut self) {
        // Core module
        self.register(ApiDoc {
            module: "lunaris_core".to_string(),
            name: "init".to_string(),
            item_type: ApiItemType::Function,
            stability: Stability::Stable,
            since: "0.1.0".to_string(),
            description: "Initialize core engine systems including logging.".to_string(),
            example: Some("lunaris_core::init()?;".to_string()),
            see_also: vec!["lunaris_runtime::init".to_string()],
        });

        // ECS
        self.register(ApiDoc {
            module: "lunaris_ecs".to_string(),
            name: "World".to_string(),
            item_type: ApiItemType::Struct,
            stability: Stability::Stable,
            since: "0.1.0".to_string(),
            description: "The ECS world containing all entities and components.".to_string(),
            example: Some("let mut world = World::new();".to_string()),
            see_also: vec!["Entity".to_string(), "Component".to_string()],
        });

        // Renderer
        self.register(ApiDoc {
            module: "lunaris_renderer".to_string(),
            name: "Camera3D".to_string(),
            item_type: ApiItemType::Struct,
            stability: Stability::Stable,
            since: "0.1.0".to_string(),
            description: "3D perspective camera for scene rendering.".to_string(),
            example: Some("let camera = Camera3D::perspective(45.0, 16.0/9.0);".to_string()),
            see_also: vec!["Camera2D".to_string()],
        });

        // Physics
        self.register(ApiDoc {
            module: "lunaris_physics".to_string(),
            name: "PhysicsWorld".to_string(),
            item_type: ApiItemType::Struct,
            stability: Stability::Stable,
            since: "0.1.0".to_string(),
            description: "Physics simulation world.".to_string(),
            example: Some("let physics = PhysicsWorld::new(gravity);".to_string()),
            see_also: vec!["RigidbodyHandle".to_string()],
        });

        // Runtime
        self.register(ApiDoc {
            module: "lunaris_runtime".to_string(),
            name: "Application".to_string(),
            item_type: ApiItemType::Trait,
            stability: Stability::Stable,
            since: "0.1.0".to_string(),
            description: "Main application trait for game implementation.".to_string(),
            example: Some(r#"impl Application for MyGame {
    fn update(&mut self, dt: f32) { }
}"#.to_string()),
            see_also: vec!["AppRunner".to_string()],
        });

        // Scripting
        self.register(ApiDoc {
            module: "lunaris_scripting".to_string(),
            name: "ScriptEngine".to_string(),
            item_type: ApiItemType::Struct,
            stability: Stability::Stable,
            since: "0.1.0".to_string(),
            description: "Sandboxed Lua scripting engine.".to_string(),
            example: Some(r#"let engine = ScriptEngine::new(config)?;"#.to_string()),
            see_also: vec!["SandboxConfig".to_string()],
        });

        // New stable APIs
        self.register(ApiDoc {
            module: "lunaris_renderer".to_string(),
            name: "LumenGI".to_string(),
            item_type: ApiItemType::Struct,
            stability: Stability::Stable,
            since: "0.1.0".to_string(),
            description: "Lumen-like global illumination system.".to_string(),
            example: Some("let gi = LumenGI::new(config);".to_string()),
            see_also: vec!["LumenConfig".to_string()],
        });

        self.register(ApiDoc {
            module: "lunaris_renderer".to_string(),
            name: "VirtualGeometryManager".to_string(),
            item_type: ApiItemType::Struct,
            stability: Stability::Stable,
            since: "0.1.0".to_string(),
            description: "Nanite-like virtualized geometry system.".to_string(),
            example: Some("let vg = VirtualGeometryManager::new();".to_string()),
            see_also: vec!["VirtualizedMesh".to_string()],
        });

        // Add categories
        self.add_category("Core", vec![
            "lunaris_core::init".to_string(),
            "lunaris_core::Time".to_string(),
            "lunaris_core::Input".to_string(),
        ]);

        self.add_category("ECS", vec![
            "lunaris_ecs::World".to_string(),
            "lunaris_ecs::Entity".to_string(),
            "lunaris_ecs::Component".to_string(),
        ]);

        self.add_category("Rendering", vec![
            "lunaris_renderer::Camera3D".to_string(),
            "lunaris_renderer::LumenGI".to_string(),
            "lunaris_renderer::VirtualGeometryManager".to_string(),
        ]);

        self.add_category("Physics", vec![
            "lunaris_physics::PhysicsWorld".to_string(),
            "lunaris_physics::CharacterController".to_string(),
        ]);

        self.add_category("Scripting", vec![
            "lunaris_scripting::ScriptEngine".to_string(),
            "lunaris_scripting::Blueprint".to_string(),
        ]);
    }

    /// Register API entry
    pub fn register(&mut self, doc: ApiDoc) {
        let key = format!("{}::{}", doc.module, doc.name);
        self.entries.insert(key, doc);
    }

    /// Add category
    pub fn add_category(&mut self, name: &str, items: Vec<String>) {
        self.categories.insert(name.to_string(), items);
    }

    /// Get API by path
    #[must_use]
    pub fn get(&self, path: &str) -> Option<&ApiDoc> {
        self.entries.get(path)
    }

    /// Get all stable APIs
    #[must_use]
    pub fn stable_apis(&self) -> Vec<&ApiDoc> {
        self.entries.values()
            .filter(|doc| doc.stability == Stability::Stable)
            .collect()
    }

    /// Get deprecated APIs
    #[must_use]
    pub fn deprecated_apis(&self) -> Vec<&ApiDoc> {
        self.entries.values()
            .filter(|doc| doc.stability == Stability::Deprecated)
            .collect()
    }

    /// Search APIs
    #[must_use]
    pub fn search(&self, query: &str) -> Vec<&ApiDoc> {
        let query_lower = query.to_lowercase();
        self.entries.values()
            .filter(|doc| {
                doc.name.to_lowercase().contains(&query_lower) ||
                doc.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get categories
    #[must_use]
    pub fn categories(&self) -> &HashMap<String, Vec<String>> {
        &self.categories
    }

    /// Generate markdown documentation
    #[must_use]
    pub fn generate_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# Lunaris Engine API Reference\n\n");
        md.push_str(&format!("**Version:** {}\n", self.version.api_version));
        md.push_str(&format!("**Engine:** {}\n", self.version.engine_version));
        if let Some(lts) = &self.version.lts_until {
            md.push_str(&format!("**LTS Support Until:** {}\n", lts));
        }
        md.push_str("\n---\n\n");

        // Group by module
        let mut by_module: HashMap<&str, Vec<&ApiDoc>> = HashMap::new();
        for doc in self.entries.values() {
            by_module.entry(&doc.module).or_default().push(doc);
        }

        for (module, docs) in by_module {
            md.push_str(&format!("## {}\n\n", module));
            
            for doc in docs {
                let stability_badge = match doc.stability {
                    Stability::Stable => "🟢 Stable",
                    Stability::Beta => "🟡 Beta",
                    Stability::Experimental => "🟠 Experimental",
                    Stability::Deprecated => "🔴 Deprecated",
                    Stability::Internal => "⚫ Internal",
                };

                md.push_str(&format!("### {} `{}`\n\n", 
                    match doc.item_type {
                        ApiItemType::Struct => "struct",
                        ApiItemType::Enum => "enum",
                        ApiItemType::Trait => "trait",
                        ApiItemType::Function => "fn",
                        ApiItemType::Constant => "const",
                        ApiItemType::Macro => "macro",
                        ApiItemType::Module => "mod",
                    },
                    doc.name
                ));

                md.push_str(&format!("{} | Since {}\n\n", stability_badge, doc.since));
                md.push_str(&format!("{}\n\n", doc.description));

                if let Some(example) = &doc.example {
                    md.push_str("```rust\n");
                    md.push_str(example);
                    md.push_str("\n```\n\n");
                }
            }
        }

        md
    }
}

/// Migration helper
pub struct MigrationGuide {
    /// From version
    pub from_version: String,
    /// To version
    pub to_version: String,
    /// Changes
    pub changes: Vec<MigrationChange>,
}

/// Migration change
#[derive(Debug, Clone)]
pub struct MigrationChange {
    /// Change type
    pub change_type: ChangeType,
    /// Old API
    pub old: String,
    /// New API
    pub new: Option<String>,
    /// Description
    pub description: String,
    /// Code example
    pub example: Option<(String, String)>,
}

/// Change type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeType {
    /// API renamed
    Renamed,
    /// API moved
    Moved,
    /// Signature changed
    SignatureChanged,
    /// Behavior changed
    BehaviorChanged,
    /// Removed
    Removed,
    /// Added
    Added,
}

impl MigrationGuide {
    /// Create migration guide
    #[must_use]
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from_version: from.to_string(),
            to_version: to.to_string(),
            changes: Vec::new(),
        }
    }

    /// Add change
    pub fn add_change(&mut self, change: MigrationChange) {
        self.changes.push(change);
    }

    /// Generate markdown
    #[must_use]
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str(&format!("# Migration Guide: {} → {}\n\n", 
            self.from_version, self.to_version));

        for change in &self.changes {
            let icon = match change.change_type {
                ChangeType::Renamed => "📝",
                ChangeType::Moved => "📦",
                ChangeType::SignatureChanged => "🔧",
                ChangeType::BehaviorChanged => "⚠️",
                ChangeType::Removed => "❌",
                ChangeType::Added => "✨",
            };

            md.push_str(&format!("## {} {}\n\n", icon, change.description));
            md.push_str(&format!("**Old:** `{}`\n", change.old));
            if let Some(new) = &change.new {
                md.push_str(&format!("**New:** `{}`\n", new));
            }
            md.push_str("\n");

            if let Some((before, after)) = &change.example {
                md.push_str("### Before\n```rust\n");
                md.push_str(before);
                md.push_str("\n```\n\n### After\n```rust\n");
                md.push_str(after);
                md.push_str("\n```\n\n");
            }
        }

        md
    }
}

/// Feature flags for stable APIs
#[derive(Debug, Clone)]
pub struct FeatureFlags {
    /// Enabled features
    pub enabled: HashMap<String, bool>,
    /// Feature descriptions
    pub descriptions: HashMap<String, String>,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        let mut flags = Self {
            enabled: HashMap::new(),
            descriptions: HashMap::new(),
        };

        // Core features (always on)
        flags.register("ecs", "Entity Component System", true);
        flags.register("rendering", "GPU rendering", true);
        flags.register("physics", "Physics simulation", true);
        flags.register("scripting", "Lua scripting", true);

        // Optional features
        flags.register("lumen_gi", "Lumen-like global illumination", true);
        flags.register("nanite_mesh", "Virtualized geometry", true);
        flags.register("ray_tracing", "Hardware ray tracing", false);
        flags.register("metahuman", "MetaHuman digital humans", false);
        flags.register("vr", "VR/AR support", false);
        flags.register("networking", "Multiplayer networking", false);
        flags.register("ai_copilot", "AI assistant", false);

        flags
    }
}

impl FeatureFlags {
    /// Register feature
    pub fn register(&mut self, name: &str, description: &str, enabled: bool) {
        self.enabled.insert(name.to_string(), enabled);
        self.descriptions.insert(name.to_string(), description.to_string());
    }

    /// Check if feature is enabled
    #[must_use]
    pub fn is_enabled(&self, name: &str) -> bool {
        self.enabled.get(name).copied().unwrap_or(false)
    }

    /// Enable feature
    pub fn enable(&mut self, name: &str) {
        self.enabled.insert(name.to_string(), true);
    }

    /// Disable feature
    pub fn disable(&mut self, name: &str) {
        self.enabled.insert(name.to_string(), false);
    }

    /// List all features
    #[must_use]
    pub fn list(&self) -> Vec<(&str, &str, bool)> {
        self.descriptions.iter()
            .map(|(name, desc)| {
                let enabled = self.enabled.get(name).copied().unwrap_or(false);
                (name.as_str(), desc.as_str(), enabled)
            })
            .collect()
    }
}
