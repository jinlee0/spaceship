use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::Collidor,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    schedule::InGameSet,
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);
const SPACESHIP_SPEED: f32 = 25.;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
pub const SPACESHIP_RADIUS: f32 = 5.;

const MISSILE_SPEED: f32 = 50.;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSILE_RADIUS: f32 = 1.;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship).add_systems(
            Update,
            (spaceship_movement_controls, spaceship_weaon_controls)
                .chain()
                .in_set(InGameSet::UserInput),
        );
    }
}

fn spawn_spaceship(mut cmd: Commands, scene_assets: Res<SceneAssets>) {
    cmd.spawn((
        MovingObjectBundle {
            velocity: Vec3::ZERO.into(),
            acceleration: Acceleration::new(Vec3::ZERO),
            collidor: Collidor::new(SPACESHIP_RADIUS),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Spaceship,
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };

    let mut movement = 0.;
    let mut rotation = 0.;
    let mut roll = 0.;

    for key_code in keyboard_input.get_pressed() {
        match key_code {
            KeyCode::S => movement = -SPACESHIP_SPEED,
            KeyCode::W => movement = SPACESHIP_SPEED,
            KeyCode::D => rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds(),
            KeyCode::A => rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds(),
            KeyCode::ShiftLeft => roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds(),
            KeyCode::ControlLeft => roll = SPACESHIP_ROLL_SPEED * time.delta_seconds(),
            _ => {}
        }
    }

    velocity.value = -transform.forward() * movement;
    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);
}

fn spaceship_weaon_controls(
    mut cmd: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let Ok(transform) = query.get_single() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::Space) {
        cmd.spawn((
            MovingObjectBundle {
                velocity: (-transform.forward() * MISSILE_SPEED).into(),
                acceleration: Vec3::ZERO.into(),
                collidor: Collidor::new(MISSILE_RADIUS),
                model: SceneBundle {
                    scene: scene_assets.missiles.clone(),
                    transform: Transform::from_translation(
                        transform.translation - transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
                    ),
                    ..default()
                },
            },
            SpaceshipMissile,
        ));
    }
}
