use bevy::prelude::*;

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
}

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub paddle: SceneAsset,
    pub ball: SceneAsset,
}
