use bevy::prelude::*;

mod asset_loader;
mod debug;
mod despawn;
mod enemy;
mod floor;
mod moviment;
mod player;
mod schedule;

use asset_loader::AssetLoaderPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use enemy::EnemyPlugin;
use floor::FloorPlugin;
use moviment::MovimentPlugin;
use player::PlayerPlugin;
use schedule::SchedulePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 900.,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovimentPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(FloorPlugin)
        .run();
}
