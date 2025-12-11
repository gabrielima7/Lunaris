//! Marketplace/Asset Store
//!
//! Plugin system and community assets.

use std::collections::HashMap;

/// Marketplace
pub struct Marketplace {
    pub assets: Vec<MarketplaceAsset>,
    pub installed: Vec<InstalledAsset>,
    pub publishers: Vec<Publisher>,
    pub categories: Vec<Category>,
    pub user: Option<MarketplaceUser>,
}

/// Marketplace asset
pub struct MarketplaceAsset {
    pub id: String,
    pub name: String,
    pub description: String,
    pub publisher: String,
    pub category: String,
    pub tags: Vec<String>,
    pub price: Price,
    pub rating: f32,
    pub downloads: u64,
    pub version: String,
    pub images: Vec<String>,
    pub engine_version: String,
}

/// Price
pub enum Price { Free, Paid(f64), Subscription }

/// Installed asset
pub struct InstalledAsset {
    pub asset_id: String,
    pub version: String,
    pub path: String,
    pub enabled: bool,
}

/// Publisher
pub struct Publisher {
    pub id: String,
    pub name: String,
    pub verified: bool,
    pub asset_count: u32,
}

/// Category
pub struct Category {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub parent: Option<String>,
}

/// User
pub struct MarketplaceUser {
    pub id: String,
    pub username: String,
    pub purchases: Vec<String>,
    pub wishlist: Vec<String>,
}

impl Marketplace {
    pub fn new() -> Self {
        Self { assets: Vec::new(), installed: Vec::new(), publishers: Vec::new(), categories: Self::default_categories(), user: None }
    }

    fn default_categories() -> Vec<Category> {
        vec![
            Category { id: "2d".into(), name: "2D Assets".into(), icon: "2d".into(), parent: None },
            Category { id: "3d".into(), name: "3D Assets".into(), icon: "3d".into(), parent: None },
            Category { id: "audio".into(), name: "Audio".into(), icon: "audio".into(), parent: None },
            Category { id: "vfx".into(), name: "VFX".into(), icon: "particles".into(), parent: None },
            Category { id: "tools".into(), name: "Tools".into(), icon: "wrench".into(), parent: None },
            Category { id: "templates".into(), name: "Templates".into(), icon: "template".into(), parent: None },
        ]
    }

    pub fn search(&self, query: &str) -> Vec<&MarketplaceAsset> {
        let q = query.to_lowercase();
        self.assets.iter().filter(|a| a.name.to_lowercase().contains(&q) || a.tags.iter().any(|t| t.to_lowercase().contains(&q))).collect()
    }

    pub fn install(&mut self, asset_id: &str, path: &str) -> Result<(), String> {
        if self.installed.iter().any(|i| i.asset_id == asset_id) { return Err("Already installed".into()); }
        let asset = self.assets.iter().find(|a| a.id == asset_id).ok_or("Asset not found")?;
        self.installed.push(InstalledAsset { asset_id: asset_id.into(), version: asset.version.clone(), path: path.into(), enabled: true });
        Ok(())
    }

    pub fn uninstall(&mut self, asset_id: &str) {
        self.installed.retain(|i| i.asset_id != asset_id);
    }

    pub fn update(&mut self, asset_id: &str) -> Result<(), String> {
        let latest = self.assets.iter().find(|a| a.id == asset_id).ok_or("Asset not found")?;
        if let Some(installed) = self.installed.iter_mut().find(|i| i.asset_id == asset_id) {
            installed.version = latest.version.clone();
        }
        Ok(())
    }
}

/// Plugin system
pub struct PluginRegistry {
    pub plugins: Vec<Plugin>,
}

/// Plugin
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub enabled: bool,
    pub entry_point: String,
}

impl PluginRegistry {
    pub fn new() -> Self { Self { plugins: Vec::new() } }
    
    pub fn load(&mut self, path: &str) -> Result<(), String> {
        // Would load plugin DLL/SO
        Ok(())
    }

    pub fn enable(&mut self, id: &str) {
        if let Some(p) = self.plugins.iter_mut().find(|p| p.id == id) { p.enabled = true; }
    }

    pub fn disable(&mut self, id: &str) {
        if let Some(p) = self.plugins.iter_mut().find(|p| p.id == id) { p.enabled = false; }
    }
}
