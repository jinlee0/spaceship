use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{asset_loader::SceneAssets, movement::MovingObjectBundle};

const VELOCITY_SCALAR: f32 = 5.;
const ACCELERATION_SCALAR: f32 = 1.;
const SPAWN_RANGE_X: Range<f32> = -25. ..25.;
const SPAWN_RANGE_Z: Range<f32> = 0. ..25.;
const SPAWN_TIME_SECONDS: f32 = 1.;

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
        .add_systems(Update, spawn_asteroid);
    }
}

fn spawn_asteroid(
    mut cmd: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.,
        rng.gen_range(SPAWN_RANGE_Z),
    );

    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1. ..1.), 0., rng.gen_range(-1. ..1.)).normalize();
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    cmd.spawn((
        MovingObjectBundle {
            velocity: velocity.into(),
            acceleration: acceleration.into(),
            model: SceneBundle {
                scene: scene_assets.asteroids.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            },
        },
        Asteroid,
    ));
}
