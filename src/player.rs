use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    moviment::{Acceleration, MovingObjectBundle, Velocity},
    schedule::InGameSet,
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const PLAYER_SPEED: f32 = 8.0;
const PLAYER_ROTATION_SPEED: f32 = 4.0;
const CAMERA_DISTANCE: f32 = 20.0;

#[derive(Component, Debug)]
pub struct Camera;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct PlayerSpel;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(
                Update,
                player_movement_controls.in_set(InGameSet::EntityUpdates),
            )
            .add_systems(
                Update,
                player_spel_controls.in_set(InGameSet::EntityUpdates),
            );
        // .add_systems(Update, sync_player_camera)
    }
}

fn spawn_player(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(STARTING_VELOCITY),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.player.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Player,
    )).with_children(|parent| {
        parent.spawn((
            Camera,
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, CAMERA_DISTANCE / 7.0, CAMERA_DISTANCE)
                .looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            }

        ));
    });
}

fn player_movement_controls(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<&mut Transform, With<Player>>,
    cam: Query<&Transform, (With<Camera>, Without<Player>)>,
) {
    let mut rotation = 0.0;
    let Ok(mut player_transform) = player.get_single_mut() else {
        return;
    };
    let Ok(cam_transform) = cam.get_single() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    //forward
    if keys.pressed(KeyCode::KeyW) {
        direction += cam_transform.forward().as_vec3();
    }

    if keys.pressed(KeyCode::KeyS) {
        direction += cam_transform.back().as_vec3();
    }

    if keys.pressed(KeyCode::KeyA) {
        direction += cam_transform.left().as_vec3();
    }

    if keys.pressed(KeyCode::KeyD) {
        direction += cam_transform.right().as_vec3();
    }

    if keys.pressed(KeyCode::ArrowLeft) {
        rotation = PLAYER_ROTATION_SPEED * time.delta_seconds();
    } else if keys.pressed(KeyCode::ArrowRight) {
        rotation = -PLAYER_ROTATION_SPEED * time.delta_seconds();
    }

    direction.y = 0.0;

    player_transform.rotate_y(rotation);
    let moviment = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();
    player_transform.translation += moviment;
}


fn player_spel_controls(
    mut command: Commands,
    query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let Ok(transform) = query.get_single() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::Space) {
        command.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(transform.forward() * 40.0),
                acceleration: Acceleration::new(Vec3::ZERO),
                model: SceneBundle {
                    scene: scene_assets.spel.clone(),
                    transform: Transform::from_translation(
                        transform.translation + Vec3::new(0.0, 0.0, -5.0),
                    ),
                    ..default()
                },
            },
            PlayerSpel,
        ));
    }
}
