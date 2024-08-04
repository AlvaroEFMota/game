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

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct PlayerSpel;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, (player_movement_controls, player_spel_controls).chain().in_set(InGameSet::UserInput))
            // .add_systems(Update, sync_player_camera)
        ;
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
    ));
}

fn player_movement_controls(
    keys: Res<Input<KeyCode>>,
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
    if keys.pressed(KeyCode::W) {
        direction += cam_transform.forward();
    }

    if keys.pressed(KeyCode::S) {
        direction += cam_transform.back();
    }

    if keys.pressed(KeyCode::A) {
        direction += cam_transform.left();
    }

    if keys.pressed(KeyCode::D) {
        direction += cam_transform.right();
    }

    if keys.pressed(KeyCode::Left) {
        rotation = PLAYER_ROTATION_SPEED * time.delta_seconds();
    } else if keys.pressed(KeyCode::Right) {
        rotation = -PLAYER_ROTATION_SPEED * time.delta_seconds();
    }

    direction.y = 0.0;

    player_transform.rotate_y(rotation);
    let moviment = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();
    player_transform.translation += moviment;
}

// fn player_movement_controls(
//     mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
//     keyboard_input: Res<Input<KeyCode>>,
//     time: Res<Time>,
// ) {

//     // X = (+)forward / (-)backward
//     // Y = (+)right / (-)left
//     // Z = (+)up / (-)down
//     let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
//         return;
//     };

//     let mut x_movement = 0.0;
//     let mut y_movement = 0.0;
//     let mut rotation = 0.0;
//     if keyboard_input.pressed(KeyCode::W) {
//         x_movement = PLAYER_SPEED;
//     } else if keyboard_input.pressed(KeyCode::S) {
//         x_movement = -PLAYER_SPEED;
//     }

//     if keyboard_input.pressed(KeyCode::A) {
//         y_movement = PLAYER_SPEED;
//     } else if keyboard_input.pressed(KeyCode::D) {
//         y_movement = -PLAYER_SPEED;
//     }

//     if keyboard_input.pressed(KeyCode::Left) {
//         rotation = PLAYER_ROTATION_SPEED * time.delta_seconds();
//     } else if keyboard_input.pressed(KeyCode::Right) {
//         rotation = -PLAYER_ROTATION_SPEED * time.delta_seconds();
//     }

//     transform.rotate_y(rotation);
//     velocity.value = -transform.forward() * x_movement + transform.right() * y_movement;
// }

fn player_spel_controls(
    mut command: Commands,
    query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let Ok(transform) = query.get_single() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::Space) {
        command.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * 40.0),
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

// pub fn sync_player_camera(
//     player: Query<&Transform, With<Player>>,
//     mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
// ) {
//     let Ok(player_transform) = player.get_single() else { return };
//     let Ok(mut camera_transform) = camera.get_single_mut() else { return };

//     *camera_transform = player_transform.clone();
// }

