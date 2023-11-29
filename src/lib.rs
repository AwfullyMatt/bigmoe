mod moe;

use crate::moe::MoePlugin;
use bevy::prelude::*;
use bevy_framepace::FramepacePlugin;
use seldom_pixel::prelude::px_layer;
use seldom_pixel::PxPlugin;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins(PxPlugin::<Layers>::new(SCREENRATIO, PALETTEMAIN.into()))
            .add_plugins(MoePlugin)
            .add_plugins(FramepacePlugin);
    }
}

#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub enum GameState {
    Menu,
    #[default]
    Playing,
}

#[derive(Reflect)]
#[px_layer]
pub enum Layers {
    #[default]
    Default,
    Moe(usize),
}

// constants
pub const SCREENRATIO: UVec2 = UVec2 { x: 288, y: 162 };
pub const PALETTEMAIN: &str = "palette/main.png";
