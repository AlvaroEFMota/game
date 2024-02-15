use bevy::prelude::*;

mod asset_loader;
mod camera;
mod debug;
mod enemy;
mod moviment;
mod player;
mod despawn;
mod schedule;

use asset_loader::AssetLoaderPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;
use enemy::EnemyPlugin;
use moviment::MovimentPlugin;
use player::PlayerPlugin;
use despawn::DespawnPlugin;
use schedule::SchedulePlugin;

#[derive(Resource, Debug, Default)]
struct GameState {
    is_playing: bool,
}

fn main() {
    App::new()
        .init_resource::<GameState>()
        // .insert_resource(GameState { is_playing: false })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(CameraPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovimentPlugin)
        // .add_plugins(DebugPlugin)
        .add_plugins(EnemyPlugin)
        // .add_plugins(DespawnPlugin)
        .add_plugins(SchedulePlugin)
        .run();
}
