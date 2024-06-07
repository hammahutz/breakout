use bevy::prelude::*;

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

#[derive(Debug, Default, Clone)]
pub struct SceneAsset {
    pub image: Handle<Image>,
}

impl SceneAsset {
    pub fn new(path: &'static str, asset_server: &Res<AssetServer>) -> SceneAsset {
        SceneAsset {
            image: asset_server.load(path),
        }
    }
    pub fn size(self, assets: Res<Assets<Image>>) -> Rectangle {
        Rectangle::from_size(assets.get(self.image.id()).unwrap().size_f32())
    }
}

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub paddle: SceneAsset,
    pub ball: SceneAsset,
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        paddle: SceneAsset::new("paddle.png", &asset_server),
        ball: SceneAsset::new("ball.png", &asset_server),
    };
}
