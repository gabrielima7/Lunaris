//! Scene hierarchy (parent-child relationships)

use bevy_ecs::prelude::*;

/// Parent component (points to parent entity)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parent(pub Entity);

impl Parent {
    /// Get the parent entity
    #[must_use]
    pub const fn get(&self) -> Entity {
        self.0
    }
}

/// Children component (list of child entities)
#[derive(Component, Debug, Clone, Default)]
pub struct Children(pub Vec<Entity>);

impl Children {
    /// Create empty children list
    #[must_use]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Add a child
    pub fn add(&mut self, child: Entity) {
        if !self.0.contains(&child) {
            self.0.push(child);
        }
    }

    /// Remove a child
    pub fn remove(&mut self, child: Entity) {
        self.0.retain(|&c| c != child);
    }

    /// Get children
    #[must_use]
    pub fn get(&self) -> &[Entity] {
        &self.0
    }

    /// Iterate over children
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }

    /// Number of children
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Commands extension for hierarchy management
pub trait HierarchyCommands {
    /// Set parent of an entity
    fn set_parent(&mut self, child: Entity, parent: Entity);
    /// Remove parent from an entity
    fn remove_parent(&mut self, child: Entity);
    /// Despawn entity and all descendants
    fn despawn_recursive(&mut self, entity: Entity);
}

/// Build a hierarchy of entities
pub struct HierarchyBuilder<'a> {
    commands: &'a mut Commands<'a, 'a>,
    root: Entity,
    current: Entity,
}

/// Scene node for serialization
#[derive(Debug, Clone)]
pub struct SceneNode {
    /// Node name
    pub name: String,
    /// Entity ID (runtime only)
    pub entity: Option<Entity>,
    /// Child nodes
    pub children: Vec<SceneNode>,
}

impl SceneNode {
    /// Create a new scene node
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            entity: None,
            children: Vec::new(),
        }
    }

    /// Add a child node
    pub fn add_child(&mut self, child: SceneNode) {
        self.children.push(child);
    }

    /// Find a node by name (depth-first)
    #[must_use]
    pub fn find(&self, name: &str) -> Option<&SceneNode> {
        if self.name == name {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find(name) {
                return Some(found);
            }
        }
        None
    }

    /// Count all nodes in subtree
    #[must_use]
    pub fn count(&self) -> usize {
        1 + self.children.iter().map(SceneNode::count).sum::<usize>()
    }
}

/// Propagate transforms through hierarchy
pub fn propagate_transforms(
    root_query: Query<(Entity, &super::Transform3D), Without<Parent>>,
    children_query: Query<&Children>,
    mut transform_query: Query<(&super::Transform3D, &mut super::GlobalTransform3D)>,
) {
    for (entity, transform) in root_query.iter() {
        // Root entities have global = local
        if let Ok((_, mut global)) = transform_query.get_mut(entity) {
            global.position = transform.position;
            global.rotation = transform.rotation;
            global.scale = transform.scale;
        }

        // Propagate to children
        propagate_recursive(
            entity,
            transform,
            &children_query,
            &mut transform_query,
        );
    }
}

fn propagate_recursive(
    parent: Entity,
    parent_transform: &super::Transform3D,
    children_query: &Query<&Children>,
    transform_query: &mut Query<(&super::Transform3D, &mut super::GlobalTransform3D)>,
) {
    if let Ok(children) = children_query.get(parent) {
        for &child in children.iter() {
            if let Ok((local, mut global)) = transform_query.get_mut(child) {
                // Combine transforms (simplified - proper implementation would use matrices)
                global.position = parent_transform.position + local.position;
                global.rotation = parent_transform.rotation + local.rotation;
                global.scale = lunaris_core::math::Vec3::new(
                    parent_transform.scale.x * local.scale.x,
                    parent_transform.scale.y * local.scale.y,
                    parent_transform.scale.z * local.scale.z,
                );

                // Recurse
                let combined = super::Transform3D {
                    position: global.position,
                    rotation: global.rotation,
                    scale: global.scale,
                };
                propagate_recursive(child, &combined, children_query, transform_query);
            }
        }
    }
}
