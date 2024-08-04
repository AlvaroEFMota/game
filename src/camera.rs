use crate::player::Player;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 20.0;
use std::f32::consts::PI;

#[derive(Component, Debug)]
pub struct Camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            // .add_systems(Update, camera_controller);
            .add_systems(Update, orbital_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, CAMERA_DISTANCE / 10.0, CAMERA_DISTANCE)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Camera,
    ));
}

fn orbital_camera(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    match (player_query.get_single(), camera_query.get_single_mut()) {
        (Ok(player_tansform), Ok(mut camera_transform)) => {
            *camera_transform = Transform::from_translation(
                player_tansform.translation
                    + player_tansform.back().normalize() * CAMERA_DISTANCE * -1.0
                    + Vec3::new(0.0, 5.0, 0.0),
            )
            .looking_at(player_tansform.translation, Vec3::Y);
        }
        _ => {}
    }
}

// fn spawn_camera(mut commands: Commands) {
//     commands.spawn((Camera3dBundle {
//         transform: Transform::from_xyz(0.0, CAMERA_DISTANCE / 10.0, CAMERA_DISTANCE)
//             .looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     },CameraController::default(), Camera));
// }

// #[derive(Component)]
// struct CameraController {
//     pub enabled: bool,
//     pub sensitivity: f32,
//     pub key_forward: KeyCode,
//     pub key_back: KeyCode,
//     pub key_left: KeyCode,
//     pub key_right: KeyCode,
//     pub key_up: KeyCode,
//     pub key_down: KeyCode,
//     pub key_run: KeyCode,
//     pub walk_speed: f32,
//     pub run_speed: f32,
//     pub friction: f32,
//     pub pitch: f32,
//     pub yaw: f32,
//     pub velocity: Vec3,
// }

// impl Default for CameraController {
//     fn default() -> Self {
//         Self {
//             enabled: true,
//             sensitivity: 0.5,
//             key_forward: KeyCode::W,
//             key_back: KeyCode::S,
//             key_left: KeyCode::A,
//             key_right: KeyCode::D,
//             key_up: KeyCode::E,
//             key_down: KeyCode::Q,
//             key_run: KeyCode::ShiftLeft,
//             walk_speed: 10.0,
//             run_speed: 30.0,
//             friction: 0.5,
//             pitch: 0.0,
//             yaw: 0.0,
//             velocity: Vec3::ZERO,
//         }
//     }
// }

// fn camera_controller(
//     time: Res<Time>,
//     mut mouse_events: EventReader<MouseMotion>,
//     key_input: Res<Input<KeyCode>>,
//     mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
// ) {
//     let dt = time.delta_seconds();

//     // Handle mouse input
//     let mut mouse_delta = Vec2::ZERO;
//     for mouse_event in mouse_events.read() {
//         mouse_delta += mouse_event.delta;
//     }

//     for (mut transform, mut options) in &mut query {
//         if !options.enabled {
//             continue;
//         }

//         // Handle key input
//         let mut axis_input = Vec3::ZERO;
//         if key_input.pressed(options.key_forward) {
//             axis_input.z += 1.0;
//         }
//         if key_input.pressed(options.key_back) {
//             axis_input.z -= 1.0;
//         }
//         if key_input.pressed(options.key_right) {
//             axis_input.x += 1.0;
//         }
//         if key_input.pressed(options.key_left) {
//             axis_input.x -= 1.0;
//         }
//         if key_input.pressed(options.key_up) {
//             axis_input.y += 1.0;
//         }
//         if key_input.pressed(options.key_down) {
//             axis_input.y -= 1.0;
//         }

//         // Apply movement update
//         if axis_input != Vec3::ZERO {
//             let max_speed = if key_input.pressed(options.key_run) {
//                 options.run_speed
//             } else {
//                 options.walk_speed
//             };
//             options.velocity = axis_input.normalize() * max_speed;
//         } else {
//             let friction = options.friction.clamp(0.0, 1.0);
//             options.velocity *= 1.0 - friction;
//             if options.velocity.length_squared() < 1e-6 {
//                 options.velocity = Vec3::ZERO;
//             }
//         }
//         let forward = transform.forward();
//         let right = transform.right();
//         transform.translation += options.velocity.x * dt * right
//             + options.velocity.y * dt * Vec3::Y
//             + options.velocity.z * dt * forward;

//         if mouse_delta != Vec2::ZERO {
//             // Apply look update
//             options.pitch = (options.pitch - mouse_delta.y * 0.5 * options.sensitivity * dt)
//                 .clamp(-PI / 2., PI / 2.);
//             options.yaw -= mouse_delta.x * options.sensitivity * dt;
//             transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, options.yaw, options.pitch);
//         }
//     }
// }
