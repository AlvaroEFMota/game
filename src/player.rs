use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    //moviment::{Acceleration, Velocity},
    schedule::InGameSet,
};
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const PLAYER_SPEED: f32 = 10.0;
const PLAYER_ROTATION_SPEED: f32 = 4.0;
const CAMERA_DISTANCE: f32 = 20.0;

#[derive(Component)]
pub struct LifeTime {
    timer: Timer,
}

#[derive(Component, Debug)]
pub struct Camera {
    radius: f32,
    angle_x: f32,
    angle_y: f32,
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct PlayerSpell;

impl LifeTime {
    fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

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
            )
            .add_systems(Update, orbit_camera.in_set(InGameSet::EntityUpdates))
            .add_systems(Update, lifetime_system.in_set(InGameSet::EntityUpdates));
        // .add_systems(Update, sync_player_camera)
    }
}

fn spawn_player(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        Player,
        Collider::ball(0.5),
        RigidBody::Dynamic,
        KinematicCharacterController {
            up: Vec3::Y,
            ..default()
        },
        Velocity {
            linvel: STARTING_VELOCITY,
            angvel: Vec3::ZERO,
        },
        SpatialBundle::from_transform(Transform::default().with_translation(Vec3::Y * 10f32)),
        LockedAxes::ROTATION_LOCKED,
        SceneRoot(scene_assets.player.clone()),
        //Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)),
    ));
    /*.with_children(|parent| {
        parent.spawn((
            Camera {
                radius: CAMERA_DISTANCE,
                angle_x: std::f32::consts::FRAC_PI_2,
                angle_y: std::f32::consts::FRAC_PI_8,
            },
            Camera3d { ..default() },
            Transform::from_xyz(0.0, CAMERA_DISTANCE / 7.0, CAMERA_DISTANCE)
                .looking_at(Vec3::ZERO, Vec3::Y),
        ));
    });*/
    commands.spawn((
        Camera {
            radius: CAMERA_DISTANCE,
            angle_x: std::f32::consts::FRAC_PI_2,
            angle_y: std::f32::consts::FRAC_PI_8,
        },
        Camera3d { ..default() },
        Transform::from_xyz(0.0, CAMERA_DISTANCE / 7.0, CAMERA_DISTANCE)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/*fn orbit_camera(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Camera)>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Camera>)>,
) {
    let Ok((mut transform, mut orbit_camera)) = query.get_single_mut() else {
        return;
    };
    let Ok(mut player_transform) = player_query.get_single_mut() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        orbit_camera.angle_y =
            (orbit_camera.angle_y + time.delta_secs()).min(std::f32::consts::FRAC_PI_2);
        // Limitar para não passar de 90 graus
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        orbit_camera.angle_y =
            (orbit_camera.angle_y - time.delta_secs()).max(-std::f32::consts::FRAC_PI_2);
        // Limitar para não passar de -90 graus
    }

    let x = orbit_camera.radius * orbit_camera.angle_x.cos() * orbit_camera.angle_y.cos();
    let y = orbit_camera.radius * orbit_camera.angle_y.sin();
    let z = orbit_camera.radius * orbit_camera.angle_x.sin() * orbit_camera.angle_y.cos();

    transform.translation = Vec3::new(x, y, z) + player_transform.translation;
    transform.look_at(player_transform.translation, Vec3::Y);
}*/

fn orbit_camera(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Camera)>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Camera>)>,
) {
    let Ok((mut transform, mut orbit_camera)) = query.get_single_mut() else {
        return;
    };
    let Ok(mut player_transform) = player_query.get_single_mut() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        orbit_camera.angle_y =
            (orbit_camera.angle_y + time.delta_secs()).min(std::f32::consts::FRAC_PI_2);
        // Limitar para não passar de 90 graus
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        orbit_camera.angle_y =
            (orbit_camera.angle_y - time.delta_secs()).max(-std::f32::consts::FRAC_PI_2);
        // Limitar para não passar de -90 graus
    }

    let x = orbit_camera.radius * orbit_camera.angle_x.cos() * orbit_camera.angle_y.cos();
    let y = orbit_camera.radius * orbit_camera.angle_y.sin();
    let z = orbit_camera.radius * orbit_camera.angle_x.sin() * orbit_camera.angle_y.cos();

    transform.translation = Vec3::new(x, y, z) + player_transform.translation;
    transform.look_at(player_transform.translation, Vec3::Y);
}

fn player_movement_controls(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut cam: Query<(&Transform, &mut Camera), (With<Camera>, Without<Player>)>,
) {
    let mut rotation = 0.0;
    let Ok((mut player_transform, mut player_velocity)) = player.get_single_mut() else {
        return;
    };
    let Ok((cam_transform, mut orbit_camera)) = cam.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    //forward
    if keys.pressed(KeyCode::KeyW) {
        direction += player_transform.forward().as_vec3();
    }

    if keys.pressed(KeyCode::KeyS) {
        direction += player_transform.back().as_vec3();
    }

    if keys.pressed(KeyCode::KeyA) {
        direction += cam_transform.left().as_vec3();
    }

    if keys.pressed(KeyCode::KeyD) {
        direction += cam_transform.right().as_vec3();
    }

    if keys.pressed(KeyCode::Space) {
        player_velocity.linvel += Vec3::new(0.0, 1.0, 0.0);
    }

    if keys.pressed(KeyCode::ArrowLeft) {
        rotation = PLAYER_ROTATION_SPEED * time.delta_secs();
    } else if keys.pressed(KeyCode::ArrowRight) {
        rotation = -PLAYER_ROTATION_SPEED * time.delta_secs();
    }

    direction.y = 0.0;

    player_transform.rotate_y(rotation);
    orbit_camera.angle_x -= rotation;
    let moviment = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    player_transform.translation += moviment;
}

fn player_spel_controls(
    mut command: Commands,
    query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let Ok(player_transform) = query.get_single() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::KeyE) {
        command.spawn((
            RigidBody::Dynamic,
            Collider::ball(0.7),
            PlayerSpell,
            LifeTime::new(0.7),
            LockedAxes::ROTATION_LOCKED,
            //Velocity::new(player_transform.forward() * 200.0),
            Velocity {
                linvel: player_transform.forward() * 100.0,
                angvel: Vec3::ZERO,
            },
            //Acceleration::new(Vec3::ZERO),
            SceneRoot(scene_assets.spel.clone()),
            Transform::from_translation(
                player_transform.translation
                    + player_transform.forward() * 2.0
                    + Vec3::new(0.0, 1.0, 0.0),
            ),
        ));
    }
}

fn lifetime_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut LifeTime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            //commands.entity(entity).despawn_recursive();
            commands.entity(entity).despawn();
        }
    }
}
