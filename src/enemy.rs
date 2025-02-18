use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use rand::Rng;
use std::ops::Range;

use crate::{
    asset_loader::SceneAssets,
    //moviment::{Acceleration, Velocity},
    schedule::InGameSet,
};

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -3.0..3.0;
const SPAWN_RANGE_Z: Range<f32> = -3.0..3.0;
//const SPAWN_TIME_SECONDS: f32 = 3.0;
const SPAWN_TIME_SECONDS: f32 = 0.1;

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
        2.0,
        rng.gen_range(SPAWN_RANGE_Z),
    );
    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero();
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;
    commands
        .spawn((
            Enemy,
            RigidBody::Dynamic,
            //LockedAxes::ROTATION_LOCKED,
            SceneRoot(scene_assets.enemy.clone()),
            Transform::from_translation(translation),
            Velocity {
                linvel: velocity,
                angvel: Vec3::ZERO,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Friction {
                    coefficient: 0.01,
                    combine_rule: CoefficientCombineRule::Min,
                },
                Collider::cuboid(0.4, 0.7, 0.4),
                Transform::from_xyz(0.0, 0.7, 0.0),
            ));
        });
}
