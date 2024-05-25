use bevy::{prelude::*, render::render_resource::Texture};

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub paddle: Handle<Image>,
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        paddle: asset_server.load("paddle.png")
    };
}
