use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    physics::Ball,
    //moviment::{Acceleration, Velocity},
    schedule::InGameSet,
};
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const PLAYER_SPEED: f32 = 15.0;
const PLAYER_ROTATION_SPEED: f32 = 3.0;
const CAMERA_DISTANCE: f32 = 20.0;

#[derive(Component)]
pub struct Grounded(bool, Timer);

#[derive(Component, Debug)]
pub struct Camera {
    radius: f32,
    angle_x: f32,
    angle_y: f32,
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct PlayerCollider;

#[derive(Component, Debug)]
pub struct PlayerSpell;

#[derive(Component)]
pub struct LifeTime {
    timer: Timer,
}

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
            //.add_systems(Update, orbit_camera.in_set(InGameSet::EntityUpdates))
            .add_systems(Update, detect_ground.in_set(InGameSet::BeforeInput))
            .add_systems(Update, detect_ball.in_set(InGameSet::BeforeInput))
            .add_systems(Update, lifetime_system.in_set(InGameSet::BeforeInput));

        // .add_systems(Update, sync_player_camera)
    }
}

fn spawn_player(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands
        .spawn((
            Player,
            //RigidBody::KinematicPositionBased,
            RigidBody::Dynamic,
            KinematicCharacterController {
                // Don’t allow climbing slopes larger than 45 degrees.
                max_slope_climb_angle: 45_f32.to_radians(),
                // Automatically slide down on slopes smaller than 30 degrees.
                min_slope_slide_angle: 30_f32.to_radians(),
                up: Vec3::Y,
                //autostep: Some(CharacterAutostep {
                //    max_height: CharacterLength::Absolute(0.5),
                //    min_width: CharacterLength::Absolute(0.2),
                //    include_dynamic_bodies: true,
                //}),
                ..default()
            },
            Velocity {
                linvel: STARTING_VELOCITY,
                angvel: Vec3::ZERO,
            },
            Transform::from_xyz(0.0, 5.0, 0.0),
            LockedAxes::ROTATION_LOCKED,
            Restitution::coefficient(0.1),
            SceneRoot(scene_assets.player.clone()),
            //SceneRoot(scene_assets.fox.clone()),
            GravityScale(3.0),
            Grounded(false, Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .with_children(|parent| {
            parent.spawn((
                PlayerCollider,
                ActiveEvents::CONTACT_FORCE_EVENTS, // It needs to be together with Collider
                Collider::cuboid(0.3, 1.0, 0.3),
                Transform::from_xyz(0.0, 1.0, 0.0),
            ));
        });

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

//fn run_player_animation(player_query:)
//
fn detect_ball(
    mut player_collider_query: Query<Entity, With<PlayerCollider>>,
    mut ball_query: Query<Entity, With<Ball>>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    if let Ok(ball_entity) = ball_query.get_single() {
        let Ok(entity) = player_collider_query.get_single() else {
            return;
        };

        for event in contact_force_events.read() {
            let player_and_ball_involved = (event.collider1.eq(&entity)
                || event.collider2.eq(&entity))
                && (event.collider1.eq(&ball_entity) || event.collider2.eq(&ball_entity));

            //println!("player and ball collided {:?}", player_and_ball_involved);
            if player_and_ball_involved {
                //println!(
                //    "player collider = {:?}, ball_collider = {:?}, collider1 = {:?}, collider2 = {:?}",
                //    entity, ball_entity, event.collider1, event.collider2
                //);
                //println!("event = {:?}", event);
                let force_direction = event.max_force_direction.normalize();
                println!("force direction {:?}", force_direction);
            }
            //}
        }
    } else {
        return;
    };
}

fn detect_ground(
    mut player_collider_query: Query<Entity, With<PlayerCollider>>,
    mut player_query: Query<&mut Grounded, With<Player>>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    time: Res<Time>,
) {
    if let Ok(mut grounded) = player_query.get_single_mut() {
        grounded.0 = false;
        let Ok(entity) = player_collider_query.get_single() else {
            return;
        };

        for event in contact_force_events.read() {
            let player_is_involved = event.collider1.eq(&entity) || event.collider2.eq(&entity);

            if player_is_involved {
                /*
                println!("Player collider involved {:?}", player_is_involved);
                println!(
                    "player collider = {:?}, collider1 = {}, collider2 = {}",
                    entity, event.collider1, event.collider2
                );
                println!("event = {:?}", event);
                */
                let force_direction = event.max_force_direction.normalize();

                // Check if the collision normal indicates an upward-facing surface
                grounded.1.tick(time.delta());
                if force_direction.y < 0.8 && grounded.1.finished() {
                    grounded.0 = true; // Player is grounded
                    break; // Exit loop once grounded
                }
            }
            //}
        }
    } else {
        return;
    };
}

fn player_movement_controls(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut Grounded), With<Player>>,
    mut cam_query: Query<(&mut Transform, &mut Camera), (Without<Player>, With<Camera>)>,
) {
    let mut rotation = 0.0;
    let Ok((mut player_transform, mut player_velocity, mut grounded)) = player.get_single_mut()
    else {
        return;
    };

    let Ok((mut cam_transform, mut orbit_camera)) = cam_query.get_single_mut() else {
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

    if keys.pressed(KeyCode::Space) && grounded.0 {
        player_velocity.linvel.y = 12.0;
        grounded.0 = false;
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

    if keys.pressed(KeyCode::ArrowUp) {
        orbit_camera.angle_y =
            (orbit_camera.angle_y + time.delta_secs()).min(std::f32::consts::FRAC_PI_2 * 0.9);
        // Limitar para não passar de 90 graus
    }
    if keys.pressed(KeyCode::ArrowDown) {
        orbit_camera.angle_y =
            (orbit_camera.angle_y - time.delta_secs()).max(-std::f32::consts::FRAC_PI_2 * 0.9);
        // Limitar para não passar de -90 graus
    }

    let x = orbit_camera.radius * orbit_camera.angle_x.cos() * orbit_camera.angle_y.cos();
    let y = orbit_camera.radius * orbit_camera.angle_y.sin();
    let z = orbit_camera.radius * orbit_camera.angle_x.sin() * orbit_camera.angle_y.cos();

    cam_transform.translation = Vec3::new(x, y, z) + player_transform.translation;
    cam_transform.look_at(player_transform.translation, Vec3::Y);
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
