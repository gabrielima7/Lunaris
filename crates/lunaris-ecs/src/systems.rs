//! Core ECS systems

use bevy_ecs::prelude::*;

/// System label for transform propagation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransformPropagation;

/// System label for visibility propagation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VisibilityPropagation;

/// Update global transforms system
pub fn update_global_transforms(
    mut query: Query<
        (&super::components::Transform3D, &mut super::components::GlobalTransform3D),
        Without<super::hierarchy::Parent>,
    >,
) {
    for (local, mut global) in query.iter_mut() {
        global.position = local.position;
        global.rotation = local.rotation;
        global.scale = local.scale;
    }
}

/// Update visibility system
pub fn update_visibility(
    mut query: Query<(&super::components::Visibility, &mut super::components::ComputedVisibility)>,
) {
    for (vis, mut computed) in query.iter_mut() {
        computed.is_visible_in_hierarchy = vis.is_visible;
        computed.is_visible_in_view = vis.is_visible;
    }
}

/// Apply velocity to transform (2D)
pub fn apply_velocity_2d(
    time: Res<GameTime>,
    mut query: Query<(&super::components::Velocity2D, &mut super::components::Transform2D)>,
) {
    let dt = time.delta_seconds;
    for (velocity, mut transform) in query.iter_mut() {
        transform.position = transform.position + velocity.linear * dt;
        transform.rotation += velocity.angular * dt;
    }
}

/// Apply velocity to transform (3D)
pub fn apply_velocity_3d(
    time: Res<GameTime>,
    mut query: Query<(&super::components::Velocity3D, &mut super::components::Transform3D)>,
) {
    let dt = time.delta_seconds;
    for (velocity, mut transform) in query.iter_mut() {
        transform.position = transform.position + velocity.linear * dt;
        transform.rotation = transform.rotation + velocity.angular * dt;
    }
}

/// Game time resource
#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct GameTime {
    /// Delta time in seconds
    pub delta_seconds: f32,
    /// Total elapsed time
    pub elapsed_seconds: f32,
    /// Fixed timestep
    pub fixed_delta_seconds: f32,
    /// Current frame
    pub frame: u64,
}

/// Input resource
#[derive(Resource, Debug, Default)]
pub struct InputResource {
    /// Input state
    pub input: lunaris_core::input::Input,
}
