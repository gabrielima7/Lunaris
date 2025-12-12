//! Scene system for saving and loading game scenes
//!
//! Provides serialization and management of game worlds.

use lunaris_core::{
    id::Id,
    Result,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Scene identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SceneId(pub Id);

impl SceneId {
    /// Generate a new scene ID
    #[must_use]
    pub fn new() -> Self {
        Self(Id::new())
    }
}

impl Default for SceneId {
    fn default() -> Self {
        Self::new()
    }
}

/// A game scene containing entities and their components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    /// Scene ID
    pub id: SceneId,
    /// Scene name
    pub name: String,
    /// Scene entities
    pub entities: Vec<EntityData>,
    /// Scene metadata
    pub metadata: SceneMetadata,
}

/// Scene metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SceneMetadata {
    /// Author
    pub author: Option<String>,
    /// Creation time
    pub created: Option<String>,
    /// Last modified
    pub modified: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Tags
    pub tags: Vec<String>,
}

/// Serialized entity data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityData {
    /// Entity ID
    pub id: u64,
    /// Entity name
    pub name: String,
    /// Parent entity ID (if any)
    pub parent: Option<u64>,
    /// Entity tags
    pub tags: Vec<String>,
    /// Components
    pub components: Vec<ComponentData>,
}

/// Serialized component data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ComponentData {
    /// Transform 2D component
    Transform2D {
        position: [f32; 2],
        rotation: f32,
        scale: [f32; 2],
    },
    /// Transform 3D component
    Transform3D {
        position: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    },
    /// Sprite component
    Sprite {
        texture: String,
        color: [f32; 4],
        flip_x: bool,
        flip_y: bool,
    },
    /// Camera component
    Camera {
        is_active: bool,
        priority: i32,
        clear_color: Option<[f32; 4]>,
    },
    /// Camera 2D settings
    Camera2D {
        zoom: f32,
    },
    /// Camera 3D settings
    Camera3D {
        fov: f32,
        near: f32,
        far: f32,
    },
    /// Rigidbody component
    Rigidbody {
        body_type: String,
        mass: f32,
        gravity_scale: f32,
    },
    /// Collider component
    Collider {
        shape: ColliderShapeData,
        is_trigger: bool,
    },
    /// Audio source component
    AudioSource {
        clip: String,
        volume: f32,
        looping: bool,
        play_on_start: bool,
    },
    /// Script component
    Script {
        path: String,
        properties: HashMap<String, serde_json::Value>,
    },
    /// Custom component (for extensions)
    Custom {
        name: String,
        data: serde_json::Value,
    },
}

/// Collider shape data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ColliderShapeData {
    /// Circle/sphere
    Circle { radius: f32 },
    /// Rectangle/box
    Rectangle { width: f32, height: f32 },
    /// Box 3D
    Box3D { width: f32, height: f32, depth: f32 },
    /// Capsule
    Capsule { height: f32, radius: f32 },
}

impl Scene {
    /// Create a new empty scene
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: SceneId::new(),
            name: name.into(),
            entities: Vec::new(),
            metadata: SceneMetadata::default(),
        }
    }

    /// Add an entity to the scene
    pub fn add_entity(&mut self, entity: EntityData) {
        self.entities.push(entity);
    }

    /// Find entity by ID
    #[must_use]
    pub fn find_entity(&self, id: u64) -> Option<&EntityData> {
        self.entities.iter().find(|e| e.id == id)
    }

    /// Find entity by name
    #[must_use]
    pub fn find_entity_by_name(&self, name: &str) -> Option<&EntityData> {
        self.entities.iter().find(|e| e.name == name)
    }

    /// Get root entities (no parent)
    #[must_use]
    pub fn root_entities(&self) -> Vec<&EntityData> {
        self.entities.iter().filter(|e| e.parent.is_none()).collect()
    }

    /// Get children of an entity
    #[must_use]
    pub fn children_of(&self, parent_id: u64) -> Vec<&EntityData> {
        self.entities
            .iter()
            .filter(|e| e.parent == Some(parent_id))
            .collect()
    }

    /// Save scene to JSON file
    ///
    /// # Errors
    ///
    /// Returns error if serialization or file writing fails
    pub fn save_json(&self, path: impl AsRef<Path>) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| lunaris_core::Error::Asset(e.to_string()))?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load scene from JSON file
    ///
    /// # Errors
    ///
    /// Returns error if file reading or deserialization fails
    pub fn load_json(path: impl AsRef<Path>) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        serde_json::from_str(&json).map_err(|e| lunaris_core::Error::Asset(e.to_string()))
    }

    /// Save scene to binary format (MessagePack)
    ///
    /// # Errors
    ///
    /// Returns error if serialization fails
    pub fn save_binary(&self, path: impl AsRef<Path>) -> Result<()> {
        let data = rmp_serde::to_vec(self)
            .map_err(|e| lunaris_core::Error::Asset(e.to_string()))?;
        std::fs::write(path, data)?;
        Ok(())
    }

    /// Load scene from binary format
    ///
    /// # Errors
    ///
    /// Returns error if file reading or deserialization fails
    pub fn load_binary(path: impl AsRef<Path>) -> Result<Self> {
        let data = std::fs::read(path)?;
        rmp_serde::from_slice(&data).map_err(|e| lunaris_core::Error::Asset(e.to_string()))
    }
}

impl EntityData {
    /// Create a new entity
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Id::new().raw(),
            name: name.into(),
            parent: None,
            tags: Vec::new(),
            components: Vec::new(),
        }
    }

    /// Add a component
    pub fn add_component(&mut self, component: ComponentData) {
        self.components.push(component);
    }

    /// Set parent
    #[must_use]
    pub fn with_parent(mut self, parent_id: u64) -> Self {
        self.parent = Some(parent_id);
        self
    }

    /// Add a tag
    #[must_use]
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Check if has a tag
    #[must_use]
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    /// Get a component by type
    #[must_use]
    pub fn get_component<F, T>(&self, f: F) -> Option<T>
    where
        F: Fn(&ComponentData) -> Option<T>,
    {
        self.components.iter().find_map(f)
    }
}

/// Scene manager for handling multiple scenes
pub struct SceneManager {
    /// Loaded scenes
    scenes: HashMap<SceneId, Scene>,
    /// Current active scene
    active_scene: Option<SceneId>,
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneManager {
    /// Create a new scene manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            active_scene: None,
        }
    }

    /// Create a new scene
    pub fn create_scene(&mut self, name: impl Into<String>) -> SceneId {
        let scene = Scene::new(name);
        let id = scene.id;
        self.scenes.insert(id, scene);
        id
    }

    /// Load a scene from file
    ///
    /// # Errors
    ///
    /// Returns error if loading fails
    pub fn load_scene(&mut self, path: impl AsRef<Path>) -> Result<SceneId> {
        let scene = if path.as_ref().extension().map_or(false, |e| e == "json") {
            Scene::load_json(path)?
        } else {
            Scene::load_binary(path)?
        };
        let id = scene.id;
        self.scenes.insert(id, scene);
        Ok(id)
    }

    /// Save a scene to file
    ///
    /// # Errors
    ///
    /// Returns error if saving fails
    pub fn save_scene(&self, id: SceneId, path: impl AsRef<Path>) -> Result<()> {
        let scene = self.scenes.get(&id).ok_or_else(|| {
            lunaris_core::Error::Asset(format!("Scene not found: {:?}", id))
        })?;

        if path.as_ref().extension().map_or(false, |e| e == "json") {
            scene.save_json(path)
        } else {
            scene.save_binary(path)
        }
    }

    /// Set active scene
    pub fn set_active(&mut self, id: SceneId) {
        if self.scenes.contains_key(&id) {
            self.active_scene = Some(id);
        }
    }

    /// Get active scene
    #[must_use]
    pub fn active_scene(&self) -> Option<&Scene> {
        self.active_scene.and_then(|id| self.scenes.get(&id))
    }

    /// Get active scene mutably
    pub fn active_scene_mut(&mut self) -> Option<&mut Scene> {
        let id = self.active_scene?;
        self.scenes.get_mut(&id)
    }

    /// Get a scene by ID
    #[must_use]
    pub fn get_scene(&self, id: SceneId) -> Option<&Scene> {
        self.scenes.get(&id)
    }

    /// Unload a scene
    pub fn unload_scene(&mut self, id: SceneId) {
        self.scenes.remove(&id);
        if self.active_scene == Some(id) {
            self.active_scene = None;
        }
    }

    /// List all loaded scenes
    #[must_use]
    pub fn list_scenes(&self) -> Vec<(SceneId, &str)> {
        self.scenes.iter().map(|(id, s)| (*id, s.name.as_str())).collect()
    }
}

/// Prefab - reusable entity template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prefab {
    /// Prefab name
    pub name: String,
    /// Root entity
    pub root: EntityData,
    /// Child entities
    pub children: Vec<EntityData>,
}

impl Prefab {
    /// Create a new prefab from an entity
    #[must_use]
    pub fn new(name: impl Into<String>, root: EntityData) -> Self {
        Self {
            name: name.into(),
            root,
            children: Vec::new(),
        }
    }

    /// Add a child entity
    pub fn add_child(&mut self, child: EntityData) {
        self.children.push(child);
    }

    /// Instantiate the prefab (creates new IDs)
    #[must_use]
    pub fn instantiate(&self) -> (EntityData, Vec<EntityData>) {
        let mut root = self.root.clone();
        root.id = Id::new().raw();

        let children: Vec<EntityData> = self
            .children
            .iter()
            .map(|c| {
                let mut child = c.clone();
                child.id = Id::new().raw();
                child.parent = Some(root.id);
                child
            })
            .collect();

        (root, children)
    }

    /// Save prefab to file
    ///
    /// # Errors
    ///
    /// Returns error if saving fails
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| lunaris_core::Error::Asset(e.to_string()))?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load prefab from file
    ///
    /// # Errors
    ///
    /// Returns error if loading fails
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        serde_json::from_str(&json).map_err(|e| lunaris_core::Error::Asset(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_scene() {
        let mut scene = Scene::new("Test Scene");
        
        let mut player = EntityData::new("Player");
        player.add_component(ComponentData::Transform2D {
            position: [100.0, 200.0],
            rotation: 0.0,
            scale: [1.0, 1.0],
        });
        player.add_component(ComponentData::Sprite {
            texture: "player.png".to_string(),
            color: [1.0, 1.0, 1.0, 1.0],
            flip_x: false,
            flip_y: false,
        });

        scene.add_entity(player);
        assert_eq!(scene.entities.len(), 1);
    }

    #[test]
    fn scene_hierarchy() {
        let mut scene = Scene::new("Hierarchy Test");

        let parent = EntityData::new("Parent");
        let parent_id = parent.id;
        scene.add_entity(parent);

        let child = EntityData::new("Child").with_parent(parent_id);
        scene.add_entity(child);

        assert_eq!(scene.root_entities().len(), 1);
        assert_eq!(scene.children_of(parent_id).len(), 1);
    }
}
