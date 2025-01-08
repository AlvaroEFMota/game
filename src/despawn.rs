use bevy::prelude::*;

use crate::{enemy::Enemy, player::PlayerSpell, schedule::InGameSet};

const DESPAWN_DISTANCE: f32 = 300.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            despawn_fay_away_entityes.in_set(InGameSet::DespawnEntities),
        );
    }
}

fn despawn_fay_away_entityes(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Or<(With<Enemy>, With<PlayerSpell>)>>,
) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);

        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
            //commands.entity(entity).despawn();
        }
    }
}
