pub mod ian;
pub mod metronome;
pub mod moe;

use crate::ian::IanPlugin;
use crate::metronome::MetronomePlugin;
use crate::moe::MoePlugin;
use bevy::prelude::*;
use bevy_framepace::FramepacePlugin;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_state::<SongState>()
            .add_plugins((MoePlugin, IanPlugin, FramepacePlugin, MetronomePlugin));
    }
}

#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub enum GameState {
    Menu,
    #[default]
    Playing,
}
#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub enum SongState {
    #[default]
    Stopped,
    Playing,
    Paused,
}

// constants
pub const SCREENRATIO: UVec2 = UVec2 { x: 288, y: 162 };
pub const PALETTEMAIN: &str = "palette/main.png";
