//! Hot Reload System
//!
//! Live code and asset reloading without restart.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

/// Reload event type
#[derive(Debug, Clone)]
pub enum ReloadEvent {
    /// Script file changed
    Script(PathBuf),
    /// Asset changed
    Asset(PathBuf, AssetType),
    /// Shader changed
    Shader(PathBuf),
    /// Scene changed
    Scene(PathBuf),
    /// Config changed
    Config(PathBuf),
}

/// Asset type for reload
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetType {
    Texture,
    Mesh,
    Material,
    Audio,
    Animation,
    Font,
    Prefab,
}

/// File watcher
pub struct FileWatcher {
    /// Watched paths
    watched: HashMap<PathBuf, WatchedFile>,
    /// Pending events
    events: Vec<ReloadEvent>,
    /// Is enabled
    pub enabled: bool,
    /// Poll interval (ms)
    pub poll_interval_ms: u32,
    /// Last poll time
    last_poll: std::time::Instant,
}

/// Watched file info
#[derive(Debug, Clone)]
struct WatchedFile {
    /// Last modified time
    modified: SystemTime,
    /// File type
    file_type: WatchedFileType,
}

/// Watched file type
#[derive(Debug, Clone, Copy)]
enum WatchedFileType {
    Script,
    Shader,
    Asset(AssetType),
    Scene,
    Config,
}

impl Default for FileWatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl FileWatcher {
    /// Create new watcher
    #[must_use]
    pub fn new() -> Self {
        Self {
            watched: HashMap::new(),
            events: Vec::new(),
            enabled: cfg!(debug_assertions),
            poll_interval_ms: 500,
            last_poll: std::time::Instant::now(),
        }
    }

    /// Watch a file
    pub fn watch(&mut self, path: PathBuf, file_type: WatchedFileType) {
        if let Ok(metadata) = std::fs::metadata(&path) {
            if let Ok(modified) = metadata.modified() {
                self.watched.insert(path, WatchedFile { modified, file_type });
            }
        }
    }

    /// Watch script directory
    pub fn watch_scripts(&mut self, dir: &std::path::Path) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |e| e == "rs" || e == "lua") {
                    self.watch(path, WatchedFileType::Script);
                }
            }
        }
    }

    /// Watch shader directory
    pub fn watch_shaders(&mut self, dir: &std::path::Path) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |e| {
                    e == "wgsl" || e == "glsl" || e == "hlsl" || e == "vert" || e == "frag"
                }) {
                    self.watch(path, WatchedFileType::Shader);
                }
            }
        }
    }

    /// Poll for changes
    pub fn poll(&mut self) -> Vec<ReloadEvent> {
        if !self.enabled {
            return Vec::new();
        }

        let now = std::time::Instant::now();
        if now.duration_since(self.last_poll).as_millis() < self.poll_interval_ms as u128 {
            return Vec::new();
        }
        self.last_poll = now;

        let mut events = Vec::new();

        for (path, watched) in &mut self.watched {
            if let Ok(metadata) = std::fs::metadata(path) {
                if let Ok(modified) = metadata.modified() {
                    if modified > watched.modified {
                        watched.modified = modified;
                        
                        let event = match watched.file_type {
                            WatchedFileType::Script => ReloadEvent::Script(path.clone()),
                            WatchedFileType::Shader => ReloadEvent::Shader(path.clone()),
                            WatchedFileType::Asset(asset_type) => {
                                ReloadEvent::Asset(path.clone(), asset_type)
                            }
                            WatchedFileType::Scene => ReloadEvent::Scene(path.clone()),
                            WatchedFileType::Config => ReloadEvent::Config(path.clone()),
                        };
                        
                        events.push(event);
                    }
                }
            }
        }

        events
    }

    /// Get pending events
    #[must_use]
    pub fn take_events(&mut self) -> Vec<ReloadEvent> {
        std::mem::take(&mut self.events)
    }
}

/// Hot reload manager
pub struct HotReloadManager {
    /// File watcher
    pub watcher: FileWatcher,
    /// Reload callbacks
    callbacks: HashMap<String, Box<dyn Fn(&ReloadEvent) + Send + Sync>>,
    /// Reload history
    history: Vec<ReloadHistoryEntry>,
    /// Max history
    pub max_history: usize,
    /// Auto apply
    pub auto_apply: bool,
}

/// Reload history entry
#[derive(Debug, Clone)]
pub struct ReloadHistoryEntry {
    /// Path
    pub path: PathBuf,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
    /// Success
    pub success: bool,
    /// Error message
    pub error: Option<String>,
}

impl Default for HotReloadManager {
    fn default() -> Self {
        Self::new()
    }
}

impl HotReloadManager {
    /// Create new manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            watcher: FileWatcher::new(),
            callbacks: HashMap::new(),
            history: Vec::new(),
            max_history: 100,
            auto_apply: true,
        }
    }

    /// Register reload callback
    pub fn on_reload<F>(&mut self, name: &str, callback: F)
    where
        F: Fn(&ReloadEvent) + Send + Sync + 'static,
    {
        self.callbacks.insert(name.to_string(), Box::new(callback));
    }

    /// Process reload events
    pub fn update(&mut self) {
        let events = self.watcher.poll();
        
        for event in events {
            if self.auto_apply {
                self.apply_reload(&event);
            }
            
            // Record history
            let entry = ReloadHistoryEntry {
                path: match &event {
                    ReloadEvent::Script(p) | ReloadEvent::Shader(p) |
                    ReloadEvent::Asset(p, _) | ReloadEvent::Scene(p) |
                    ReloadEvent::Config(p) => p.clone(),
                },
                timestamp: std::time::SystemTime::now(),
                success: true,
                error: None,
            };
            
            self.history.push(entry);
            if self.history.len() > self.max_history {
                self.history.remove(0);
            }
        }
    }

    fn apply_reload(&self, event: &ReloadEvent) {
        for callback in self.callbacks.values() {
            callback(event);
        }
    }

    /// Get reload history
    #[must_use]
    pub fn history(&self) -> &[ReloadHistoryEntry] {
        &self.history
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

/// Live script reload state
pub struct LiveScriptState {
    /// Current script hash
    current_hash: u64,
    /// Variables to preserve
    preserved_vars: HashMap<String, Vec<u8>>,
    /// Is dirty
    pub dirty: bool,
}

impl Default for LiveScriptState {
    fn default() -> Self {
        Self::new()
    }
}

impl LiveScriptState {
    /// Create new state
    #[must_use]
    pub fn new() -> Self {
        Self {
            current_hash: 0,
            preserved_vars: HashMap::new(),
            dirty: false,
        }
    }

    /// Preserve variable
    pub fn preserve(&mut self, name: &str, data: Vec<u8>) {
        self.preserved_vars.insert(name.to_string(), data);
    }

    /// Restore variable
    #[must_use]
    pub fn restore(&self, name: &str) -> Option<&Vec<u8>> {
        self.preserved_vars.get(name)
    }

    /// Clear preserved
    pub fn clear(&mut self) {
        self.preserved_vars.clear();
    }

    /// Update hash
    pub fn update_hash(&mut self, new_hash: u64) {
        if new_hash != self.current_hash {
            self.current_hash = new_hash;
            self.dirty = true;
        }
    }
}
