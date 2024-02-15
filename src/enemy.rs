use bevy::{ecs::system::InsertResource, prelude::*};
use rand::Rng;
use std::ops::Range;

use crate::{
    asset_loader::SceneAssets, moviment::{Acceleration, MovingObjectBundle, Velocity}, schedule::InGameSet
};

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 3.0;

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Resource, Debug, Default)]
pub struct SpawnTimer {
    timer: Timer,
}
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_enemy.in_set(InGameSet::EntityUpdates));
    }
}

fn spawn_enemy(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.0,
        rng.gen_range(SPAWN_RANGE_Z),
    );
    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero();
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(velocity),
            acceleration: Acceleration::new(acceleration),
            model: SceneBundle {
                scene: scene_assets.enemy.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            },
        },
        Enemy,
    ));
}
