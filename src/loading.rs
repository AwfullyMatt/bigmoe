use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            GameState::Loading,
            "dynamic.assets.ron",
        )
        .add_collection_to_loading_state::<_, ImageAssets>(GameState::Loading);
    }
}

#[allow(dead_code)]
#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(key = "minormoe")]
    pub minormoe: Handle<Image>,
    #[asset(key = "majormoe")]
    pub majormoe: Handle<Image>,
    #[asset(key = "ian")]
    pub ian: Handle<Image>,
    #[asset(key = "title")]
    pub title: Handle<TextureAtlas>,
    #[asset(key = "titlesleep")]
    pub titlesleep: Handle<TextureAtlas>,
}
