use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::Collidor,
    movement::MovingObjectBundle,
    schedule::InGameSet,
    spaceship::{Spaceship, SPACESHIP_RADIUS},
};

const VELOCITY_SCALAR: f32 = 5.;
const ACCELERATION_SCALAR: f32 = 1.;
const SPAWN_RANGE_X: Range<f32> = -25. ..25.;
const SPAWN_RANGE_Z: Range<f32> = 0. ..25.;
const SPAWN_TIME_SECONDS: f32 = 1.;
const ROTATE_SPEED: f32 = 2.5;
const RADIUS: f32 = 2.5;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            (spawn_asteroid, rotate_asteroid).in_set(InGameSet::EntityUpdates),
        );
    }
}

fn spawn_asteroid(
    mut cmd: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
    spaceship_query: Query<&Transform, With<Spaceship>>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let Ok(transform) = spaceship_query.get_single() else {
        return;
    };

    let mut rng = rand::thread_rng();

    let translation = random_asteroid_pos(&mut rng, transform.translation);

    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1. ..1.), 0., rng.gen_range(-1. ..1.)).normalize();
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    cmd.spawn((
        MovingObjectBundle {
            velocity: velocity.into(),
            acceleration: acceleration.into(),
            collidor: Collidor::new(RADIUS),
            model: SceneBundle {
                scene: scene_assets.asteroids.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            },
        },
        Asteroid,
    ));
}

fn random_asteroid_pos(rng: &mut rand::prelude::ThreadRng, spaceship_pos: Vec3) -> Vec3 {
    (0..)
        .map(|_| {
            Vec3::new(
                rng.gen_range(SPAWN_RANGE_X),
                0.,
                rng.gen_range(SPAWN_RANGE_Z),
            )
        })
        .find(|&asteroid_pos| {
            spaceship_pos.distance(asteroid_pos) > ((SPACESHIP_RADIUS + RADIUS) * 2.)
        })
        .unwrap_or_else(|| {
            Vec3::new(
                rng.gen_range(SPAWN_RANGE_X),
                0.,
                rng.gen_range(SPAWN_RANGE_Z),
            )
        })
}

fn rotate_asteroid(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATE_SPEED * time.delta_seconds());
    }
}
