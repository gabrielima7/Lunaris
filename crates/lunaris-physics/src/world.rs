//! Physics world simulation

use crate::{
    collision::{ColliderShape, CollisionEvent, RaycastHit, RaycastQuery},
    rigidbody::{ColliderProperties, ForceMode, RigidbodyHandle, RigidbodyProperties, RigidbodyState},
    PhysicsConfig,
};
use lunaris_core::{id::Id, math::Vec3};
use std::collections::HashMap;

/// The physics world containing all simulation state
pub struct PhysicsWorld {
    config: PhysicsConfig,
    bodies: HashMap<RigidbodyHandle, RigidbodyData>,
    collision_events: Vec<CollisionEvent>,
    accumulator: f32,
}

struct RigidbodyData {
    entity_id: Id,
    properties: RigidbodyProperties,
    state: RigidbodyState,
    collider: Option<ColliderData>,
    pending_forces: Vec<(Vec3, ForceMode)>,
    pending_torques: Vec<(Vec3, ForceMode)>,
}

struct ColliderData {
    shape: ColliderShape,
    properties: ColliderProperties,
    offset: Vec3,
}

impl PhysicsWorld {
    /// Create a new physics world
    #[must_use]
    pub fn new(config: PhysicsConfig) -> Self {
        tracing::info!("Physics world created with gravity: {:?}", config.gravity);
        Self {
            config,
            bodies: HashMap::new(),
            collision_events: Vec::new(),
            accumulator: 0.0,
        }
    }

    /// Create a rigidbody
    pub fn create_rigidbody(
        &mut self,
        entity_id: Id,
        properties: RigidbodyProperties,
        position: Vec3,
    ) -> RigidbodyHandle {
        let handle = RigidbodyHandle(Id::new());
        
        self.bodies.insert(
            handle,
            RigidbodyData {
                entity_id,
                properties,
                state: RigidbodyState {
                    position,
                    ..Default::default()
                },
                collider: None,
                pending_forces: Vec::new(),
                pending_torques: Vec::new(),
            },
        );

        tracing::debug!("Created rigidbody {:?} for entity {:?}", handle, entity_id);
        handle
    }

    /// Attach a collider to a rigidbody
    pub fn attach_collider(
        &mut self,
        handle: RigidbodyHandle,
        shape: ColliderShape,
        properties: ColliderProperties,
        offset: Vec3,
    ) {
        if let Some(body) = self.bodies.get_mut(&handle) {
            body.collider = Some(ColliderData {
                shape,
                properties,
                offset,
            });
        }
    }

    /// Remove a rigidbody
    pub fn remove_rigidbody(&mut self, handle: RigidbodyHandle) {
        self.bodies.remove(&handle);
    }

    /// Get rigidbody state
    #[must_use]
    pub fn get_state(&self, handle: RigidbodyHandle) -> Option<RigidbodyState> {
        self.bodies.get(&handle).map(|b| b.state)
    }

    /// Set rigidbody position
    pub fn set_position(&mut self, handle: RigidbodyHandle, position: Vec3) {
        if let Some(body) = self.bodies.get_mut(&handle) {
            body.state.position = position;
        }
    }

    /// Set rigidbody rotation
    pub fn set_rotation(&mut self, handle: RigidbodyHandle, rotation: Vec3) {
        if let Some(body) = self.bodies.get_mut(&handle) {
            body.state.rotation = rotation;
        }
    }

    /// Set linear velocity
    pub fn set_linear_velocity(&mut self, handle: RigidbodyHandle, velocity: Vec3) {
        if let Some(body) = self.bodies.get_mut(&handle) {
            body.state.linear_velocity = velocity;
        }
    }

    /// Apply a force to a rigidbody
    pub fn apply_force(&mut self, handle: RigidbodyHandle, force: Vec3, mode: ForceMode) {
        if let Some(body) = self.bodies.get_mut(&handle) {
            body.pending_forces.push((force, mode));
        }
    }

    /// Apply a torque to a rigidbody
    pub fn apply_torque(&mut self, handle: RigidbodyHandle, torque: Vec3, mode: ForceMode) {
        if let Some(body) = self.bodies.get_mut(&handle) {
            body.pending_torques.push((torque, mode));
        }
    }

    /// Perform a raycast
    #[must_use]
    pub fn raycast(&self, query: &RaycastQuery) -> Option<RaycastHit> {
        // Simplified raycast - in real implementation would use spatial acceleration
        let mut closest: Option<RaycastHit> = None;

        for (_handle, body) in &self.bodies {
            if let Some(collider) = &body.collider {
                if !query.layers.can_interact(collider.properties.layers) {
                    continue;
                }

                // Simple sphere collision for demonstration
                if let ColliderShape::Shape3D(ref shape) = collider.shape {
                    if let crate::collision::ColliderShape3D::Sphere { radius } = shape {
                        let center = body.state.position + collider.offset;
                        if let Some(hit) = ray_sphere_intersection(
                            query.origin,
                            query.direction,
                            center,
                            *radius,
                        ) {
                            if hit.distance <= query.max_distance {
                                if closest.is_none() || hit.distance < closest.as_ref().unwrap().distance {
                                    closest = Some(RaycastHit {
                                        entity: body.entity_id,
                                        point: hit.point,
                                        normal: hit.normal,
                                        distance: hit.distance,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        closest
    }

    /// Step the simulation
    pub fn step(&mut self, delta_time: f32) {
        self.accumulator += delta_time;
        let timestep = self.config.timestep;
        let max_steps = self.config.max_substeps;

        let mut steps = 0;
        while self.accumulator >= timestep && steps < max_steps {
            self.fixed_step(timestep);
            self.accumulator -= timestep;
            steps += 1;
        }
    }

    fn fixed_step(&mut self, dt: f32) {
        // Clear previous collision events
        self.collision_events.clear();

        // Apply forces and integrate
        for body in self.bodies.values_mut() {
            if body.properties.body_type != crate::rigidbody::RigidbodyType::Dynamic {
                body.pending_forces.clear();
                body.pending_torques.clear();
                continue;
            }

            // Apply gravity
            let gravity = self.config.gravity * body.properties.gravity_scale;
            let mass = body.properties.mass;

            // Apply pending forces
            for (force, mode) in body.pending_forces.drain(..) {
                match mode {
                    ForceMode::Force => {
                        body.state.linear_velocity = body.state.linear_velocity + force * dt / mass;
                    }
                    ForceMode::Impulse => {
                        body.state.linear_velocity = body.state.linear_velocity + force / mass;
                    }
                    ForceMode::Acceleration => {
                        body.state.linear_velocity = body.state.linear_velocity + force * dt;
                    }
                    ForceMode::VelocityChange => {
                        body.state.linear_velocity = body.state.linear_velocity + force;
                    }
                }
            }

            // Apply gravity
            body.state.linear_velocity = body.state.linear_velocity + gravity * dt;

            // Apply damping
            body.state.linear_velocity = body.state.linear_velocity
                * (1.0 - body.properties.linear_damping * dt);
            body.state.angular_velocity = body.state.angular_velocity
                * (1.0 - body.properties.angular_damping * dt);

            // Integrate position
            body.state.position = body.state.position + body.state.linear_velocity * dt;
            body.state.rotation = body.state.rotation + body.state.angular_velocity * dt;
        }

        // TODO: Collision detection and response
    }

    /// Get collision events from the last step
    #[must_use]
    pub fn collision_events(&self) -> &[CollisionEvent] {
        &self.collision_events
    }

    /// Get physics config
    #[must_use]
    pub fn config(&self) -> &PhysicsConfig {
        &self.config
    }

    /// Set gravity
    pub fn set_gravity(&mut self, gravity: Vec3) {
        self.config.gravity = gravity;
    }
}

/// Ray-sphere intersection helper
fn ray_sphere_intersection(
    origin: Vec3,
    direction: Vec3,
    center: Vec3,
    radius: f32,
) -> Option<RaycastHit> {
    let oc = origin - center;
    let a = direction.dot(direction);
    let b = 2.0 * oc.dot(direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let t = (-b - discriminant.sqrt()) / (2.0 * a);
    if t < 0.0 {
        return None;
    }

    let point = origin + direction * t;
    let normal = (point - center).normalize();

    Some(RaycastHit {
        entity: Id::NULL,
        point,
        normal,
        distance: t,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_world() {
        let world = PhysicsWorld::new(PhysicsConfig::default());
        assert!((world.config().gravity.y + 9.81).abs() < 0.01);
    }

    #[test]
    fn create_rigidbody() {
        let mut world = PhysicsWorld::new(PhysicsConfig::default());
        let handle = world.create_rigidbody(
            Id::new(),
            RigidbodyProperties::dynamic(),
            Vec3::ZERO,
        );

        let state = world.get_state(handle).unwrap();
        assert_eq!(state.position, Vec3::ZERO);
    }

    #[test]
    fn gravity_applies() {
        let mut world = PhysicsWorld::new(PhysicsConfig::default());
        let handle = world.create_rigidbody(
            Id::new(),
            RigidbodyProperties::dynamic(),
            Vec3::new(0.0, 10.0, 0.0),
        );

        // Step for 1 second
        for _ in 0..60 {
            world.step(1.0 / 60.0);
        }

        let state = world.get_state(handle).unwrap();
        // Should have fallen due to gravity
        assert!(state.position.y < 10.0);
        assert!(state.linear_velocity.y < 0.0);
    }
}
