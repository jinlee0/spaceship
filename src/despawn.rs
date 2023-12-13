use bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 100.;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_far_away_entities);
    }
}

fn despawn_far_away_entities(mut cmd: Commands, query: Query<(Entity, &GlobalTransform)>) {
    query.iter().for_each(|(entity, transform)| {
        if transform.translation().distance(Vec3::ZERO) > DESPAWN_DISTANCE {
            cmd.entity(entity).despawn_recursive();
        }
    })
}
