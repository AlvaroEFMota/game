use bevy::prelude::*;


pub struct FloorPlugin;

impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_floor);
    }
}

fn spawn_floor(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let floor = PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0))),
        material: materials.add(Color::WHITE),
        ..default()
    };

    commands.spawn(floor);
}