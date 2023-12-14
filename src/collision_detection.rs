use std::collections::HashMap;

use bevy::prelude::*;

use crate::{astroids::Asteroid, schedule::InGameSet, spaceship::Spaceship};

#[derive(Component, Debug)]
pub struct Collidor {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collidor {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_detection.in_set(InGameSet::CollisionDetection),
        )
        .add_systems(
            Update,
            (
                despawn_on_collision::<Asteroid>,
                despawn_on_collision::<Spaceship>,
            )
                .in_set(InGameSet::DespawnEntitys),
        );
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collidor)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // Detect collisions
    for (entity_a, transform_a, collidor_a) in query.iter() {
        for (entity_b, transform_b, collidor_b) in query.iter() {
            if entity_a == entity_b {
                continue;
            }
            let distance = transform_a
                .translation()
                .distance(transform_b.translation());
            if distance < collidor_a.radius + collidor_b.radius {
                colliding_entities
                    .entry(entity_a)
                    .or_default()
                    .push(entity_b);
            }
        }
    }

    // Update Collidors
    for (entity, _, mut collidor) in query.iter_mut() {
        collidor.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collidor.colliding_entities.extend(collisions);
        }
    }
}

fn despawn_on_collision<T: Component>(
    mut cmd: Commands,
    query: Query<(Entity, &Collidor), With<T>>,
) {
    for (entity, collidor) in query.iter() {
        for &collied_entity in collidor.colliding_entities.iter() {
            if query.get(collied_entity).is_ok() {
                continue;
            }
            if let Some(cmd) = cmd.get_entity(entity) {
                cmd.despawn_recursive();
            }
        }
    }
}
