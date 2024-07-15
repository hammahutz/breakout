use crate::game::prelude::*;
use bevy::prelude::*;

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        paddle: SceneAsset::new("paddle.png", &asset_server),
        ball: SceneAsset::new("ball.png", &asset_server),
    };
}
