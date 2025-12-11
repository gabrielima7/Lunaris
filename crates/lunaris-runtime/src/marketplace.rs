//! Plugin and Marketplace System
//!
//! Extensibility framework for third-party plugins.

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Plugin state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginState {
    /// Not loaded
    Unloaded,
    /// Loading
    Loading,
    /// Loaded and ready
    Loaded,
    /// Active (running)
    Active,
    /// Error state
    Error,
    /// Disabled by user
    Disabled,
}

/// Plugin category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginCategory {
    /// Core engine extension
    Core,
    /// Rendering/graphics
    Rendering,
    /// Physics
    Physics,
    /// Audio
    Audio,
    /// Scripting
    Scripting,
    /// Editor tools
    Editor,
    /// AI/Navigation
    AI,
    /// Networking
    Networking,
    /// Asset pipeline
    Assets,
    /// Platform-specific
    Platform,
    /// Gameplay systems
    Gameplay,
    /// Art/content
    Content,
    /// Other
    Other,
}

/// Semantic version
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub prerelease: Option<String>,
}

impl SemVer {
    /// Create new version
    #[must_use]
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            prerelease: None,
        }
    }

    /// Parse from string
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() >= 3 {
            Some(Self {
                major: parts[0].parse().ok()?,
                minor: parts[1].parse().ok()?,
                patch: parts[2].split('-').next()?.parse().ok()?,
                prerelease: s.split('-').nth(1).map(|s| s.to_string()),
            })
        } else {
            None
        }
    }

    /// Check if compatible with required version
    #[must_use]
    pub fn is_compatible(&self, required: &Self) -> bool {
        if self.major != required.major {
            return self.major > required.major;
        }
        if self.minor < required.minor {
            return false;
        }
        if self.minor == required.minor && self.patch < required.patch {
            return false;
        }
        true
    }
}

impl std::fmt::Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(pre) = &self.prerelease {
            write!(f, "-{}", pre)?;
        }
        Ok(())
    }
}

/// Plugin dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Plugin ID
    pub plugin_id: String,
    /// Required version
    pub version: String,
    /// Is optional
    pub optional: bool,
}

/// Plugin manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Unique plugin ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Version
    pub version: SemVer,
    /// Description
    pub description: String,
    /// Author
    pub author: String,
    /// Website/repo URL
    pub url: Option<String>,
    /// License
    pub license: String,
    /// Category
    pub category: PluginCategory,
    /// Tags
    pub tags: Vec<String>,
    /// Dependencies
    pub dependencies: Vec<Dependency>,
    /// Required engine version
    pub engine_version: String,
    /// Entry point (Rust crate or script)
    pub entry_point: String,
    /// Config schema (JSON Schema)
    pub config_schema: Option<String>,
}

/// Plugin config
#[derive(Debug, Clone)]
pub struct PluginConfig {
    /// Config values
    pub values: HashMap<String, ConfigValue>,
}

/// Config value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<ConfigValue>),
    Object(HashMap<String, ConfigValue>),
}

/// Plugin info (runtime)
#[derive(Debug, Clone)]
pub struct PluginInfo {
    /// Manifest
    pub manifest: PluginManifest,
    /// Install path
    pub path: PathBuf,
    /// State
    pub state: PluginState,
    /// Config
    pub config: PluginConfig,
    /// Load order
    pub load_order: i32,
    /// Error message (if any)
    pub error: Option<String>,
}

/// Plugin hook for editor integration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorHook {
    /// Main menu
    MainMenu,
    /// Asset browser context menu
    AssetContextMenu,
    /// Entity context menu
    EntityContextMenu,
    /// Viewport toolbar
    ViewportToolbar,
    /// Properties panel
    PropertiesPanel,
    /// Toolbar
    MainToolbar,
    /// Status bar
    StatusBar,
}

/// Plugin API for extending the engine
pub trait PluginAPI: Send + Sync {
    /// Plugin ID
    fn id(&self) -> &str;

    /// Plugin name
    fn name(&self) -> &str;

    /// Version
    fn version(&self) -> SemVer;

    /// Initialize plugin
    fn init(&mut self) -> Result<(), PluginError>;

    /// Shutdown plugin
    fn shutdown(&mut self) -> Result<(), PluginError>;

    /// Called every frame (optional)
    fn update(&mut self, _dt: f32) {}

    /// Called on editor render (optional)
    fn render_ui(&self) {}

    /// Get config schema (optional)
    fn config_schema(&self) -> Option<&str> { None }

    /// Apply config (optional)
    fn apply_config(&mut self, _config: &PluginConfig) -> Result<(), PluginError> { Ok(()) }
}

/// Plugin error
#[derive(Debug, Clone)]
pub enum PluginError {
    /// Plugin not found
    NotFound(String),
    /// Invalid manifest
    InvalidManifest(String),
    /// Dependency not met
    DependencyNotMet { plugin: String, dependency: String },
    /// Version mismatch
    VersionMismatch { required: String, found: String },
    /// Load error
    LoadError(String),
    /// Init error
    InitError(String),
    /// Config error
    ConfigError(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(id) => write!(f, "Plugin not found: {}", id),
            Self::InvalidManifest(e) => write!(f, "Invalid manifest: {}", e),
            Self::DependencyNotMet { plugin, dependency } => {
                write!(f, "Plugin {} requires {}", plugin, dependency)
            }
            Self::VersionMismatch { required, found } => {
                write!(f, "Version mismatch: required {}, found {}", required, found)
            }
            Self::LoadError(e) => write!(f, "Load error: {}", e),
            Self::InitError(e) => write!(f, "Init error: {}", e),
            Self::ConfigError(e) => write!(f, "Config error: {}", e),
        }
    }
}

impl std::error::Error for PluginError {}

/// Plugin registry
pub struct PluginRegistry {
    /// Installed plugins
    plugins: HashMap<String, PluginInfo>,
    /// Plugin search paths
    search_paths: Vec<PathBuf>,
    /// Active plugins
    active: Vec<String>,
    /// Engine version
    engine_version: SemVer,
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginRegistry {
    /// Create new registry
    #[must_use]
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            search_paths: vec![
                PathBuf::from("plugins"),
                PathBuf::from("~/.lunaris/plugins"),
            ],
            active: Vec::new(),
            engine_version: SemVer::new(0, 1, 0),
        }
    }

    /// Add search path
    pub fn add_search_path(&mut self, path: PathBuf) {
        self.search_paths.push(path);
    }

    /// Scan for plugins
    pub fn scan(&mut self) -> Vec<PluginInfo> {
        let mut found = Vec::new();

        for search_path in &self.search_paths {
            if let Ok(entries) = std::fs::read_dir(search_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let manifest_path = path.join("plugin.toml");
                        if manifest_path.exists() {
                            if let Ok(info) = self.load_manifest(&path) {
                                found.push(info);
                            }
                        }
                    }
                }
            }
        }

        found
    }

    fn load_manifest(&self, path: &PathBuf) -> Result<PluginInfo, PluginError> {
        // Would parse actual TOML manifest
        // For now, create example
        let manifest = PluginManifest {
            id: path.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string(),
            name: "Example Plugin".to_string(),
            version: SemVer::new(1, 0, 0),
            description: "An example plugin".to_string(),
            author: "Lunaris Team".to_string(),
            url: None,
            license: "MIT".to_string(),
            category: PluginCategory::Other,
            tags: vec![],
            dependencies: vec![],
            engine_version: "0.1.0".to_string(),
            entry_point: "lib.rs".to_string(),
            config_schema: None,
        };

        Ok(PluginInfo {
            manifest,
            path: path.clone(),
            state: PluginState::Unloaded,
            config: PluginConfig { values: HashMap::new() },
            load_order: 0,
            error: None,
        })
    }

    /// Install plugin from path
    pub fn install(&mut self, path: &PathBuf) -> Result<String, PluginError> {
        let info = self.load_manifest(path)?;
        let id = info.manifest.id.clone();
        
        // Check engine version compatibility
        if let Some(required) = SemVer::parse(&info.manifest.engine_version) {
            if !self.engine_version.is_compatible(&required) {
                return Err(PluginError::VersionMismatch {
                    required: info.manifest.engine_version.clone(),
                    found: self.engine_version.to_string(),
                });
            }
        }

        self.plugins.insert(id.clone(), info);
        Ok(id)
    }

    /// Enable plugin
    pub fn enable(&mut self, id: &str) -> Result<(), PluginError> {
        let plugin = self.plugins.get_mut(id)
            .ok_or_else(|| PluginError::NotFound(id.to_string()))?;

        // Check dependencies
        for dep in &plugin.manifest.dependencies {
            if !dep.optional && !self.plugins.contains_key(&dep.plugin_id) {
                return Err(PluginError::DependencyNotMet {
                    plugin: id.to_string(),
                    dependency: dep.plugin_id.clone(),
                });
            }
        }

        plugin.state = PluginState::Loaded;
        
        if !self.active.contains(&id.to_string()) {
            self.active.push(id.to_string());
        }

        Ok(())
    }

    /// Disable plugin
    pub fn disable(&mut self, id: &str) -> Result<(), PluginError> {
        let plugin = self.plugins.get_mut(id)
            .ok_or_else(|| PluginError::NotFound(id.to_string()))?;

        plugin.state = PluginState::Disabled;
        self.active.retain(|x| x != id);

        Ok(())
    }

    /// Get plugin
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&PluginInfo> {
        self.plugins.get(id)
    }

    /// Get all plugins
    #[must_use]
    pub fn all(&self) -> Vec<&PluginInfo> {
        self.plugins.values().collect()
    }

    /// Get active plugins
    #[must_use]
    pub fn active(&self) -> &[String] {
        &self.active
    }

    /// Uninstall plugin
    pub fn uninstall(&mut self, id: &str) -> Result<(), PluginError> {
        self.disable(id)?;
        self.plugins.remove(id);
        Ok(())
    }
}

/// Marketplace client
pub struct MarketplaceClient {
    /// API endpoint
    pub endpoint: String,
    /// Auth token
    pub token: Option<String>,
    /// Cache directory
    pub cache_dir: PathBuf,
}

impl Default for MarketplaceClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketplaceClient {
    /// Create new client
    #[must_use]
    pub fn new() -> Self {
        Self {
            endpoint: "https://marketplace.lunaris.dev/api/v1".to_string(),
            token: None,
            cache_dir: PathBuf::from(".cache/marketplace"),
        }
    }

    /// Search plugins
    pub fn search(&self, query: &str, category: Option<PluginCategory>) -> Vec<MarketplaceEntry> {
        // Would call API
        // Return example entries
        vec![
            MarketplaceEntry {
                id: "example-plugin".to_string(),
                name: "Example Plugin".to_string(),
                description: "An example marketplace plugin".to_string(),
                author: "Community".to_string(),
                version: SemVer::new(1, 0, 0),
                downloads: 1000,
                rating: 4.5,
                category: category.unwrap_or(PluginCategory::Other),
                price: None,
            }
        ]
    }

    /// Download plugin
    pub fn download(&self, id: &str) -> Result<PathBuf, MarketplaceError> {
        // Would download actual plugin
        let path = self.cache_dir.join(id);
        Ok(path)
    }

    /// Rate plugin
    pub fn rate(&self, id: &str, rating: u8) -> Result<(), MarketplaceError> {
        if self.token.is_none() {
            return Err(MarketplaceError::AuthRequired);
        }
        // Would call API
        Ok(())
    }
}

/// Marketplace entry
#[derive(Debug, Clone)]
pub struct MarketplaceEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: SemVer,
    pub downloads: u64,
    pub rating: f32,
    pub category: PluginCategory,
    pub price: Option<f32>,
}

/// Marketplace error
#[derive(Debug, Clone)]
pub enum MarketplaceError {
    /// Network error
    NetworkError(String),
    /// Auth required
    AuthRequired,
    /// Not found
    NotFound,
    /// Rate limit
    RateLimited,
}

impl std::fmt::Display for MarketplaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NetworkError(e) => write!(f, "Network error: {}", e),
            Self::AuthRequired => write!(f, "Authentication required"),
            Self::NotFound => write!(f, "Plugin not found"),
            Self::RateLimited => write!(f, "Rate limited"),
        }
    }
}

impl std::error::Error for MarketplaceError {}
