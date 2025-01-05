use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub enemy: Handle<Scene>,
    pub player: Handle<Scene>,
    pub spel: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        enemy: asset_server.load("Skeleton.glb#Scene0"),
        player: asset_server.load("Witch180r.glb#Scene0"),
        //player: asset_server.load("Witch.glb#Scene0"),
        //player: asset_server.load("Witch180rn.glb#Scene0"),
        spel: asset_server.load("Fire.glb#Scene0"),
    }
}
