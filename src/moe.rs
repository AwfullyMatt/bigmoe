use crate::{GameState, Layers};
use bevy::prelude::*;
use seldom_pixel::prelude::*;

pub struct MoePlugin;
impl Plugin for MoePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Moed>()
            .add_systems(OnEnter(GameState::Playing), (spawn_moe, spawn_big_moe));
    }
}

#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub enum Moed {
    #[default]
    Regular,
    Big,
}

#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub enum Mode {
    One,   // Major
    Two,   // Natural Minor
    Three, // Dorian -- like minor but not as intense, a bit groovier
}
fn spawn_moe(mut commands: Commands, mut sprites: PxAssets<PxSprite>) {
    let sprite = sprites.load_animated(MOE, 1);
    commands.spawn((
        PxSpriteBundle::<Layers> {
            sprite,
            position: PxPosition::from(IVec2 { x: 24, y: 81 }),
            layer: Layers::Moe(0),
            ..default()
        },
        PxAnimationBundle {
            on_finish: PxAnimationFinishBehavior::Loop,
            ..default()
        },
    ));
}

fn spawn_big_moe(mut commands: Commands, mut sprites: PxAssets<PxSprite>) {
    let sprite = sprites.load_animated(BIGMOE, 1);
    commands.spawn((
        PxSpriteBundle::<Layers> {
            sprite,
            position: PxPosition::from(IVec2 { x: 240, y: 81 }),
            layer: Layers::Moe(0),
            ..default()
        },
        PxAnimationBundle {
            on_finish: PxAnimationFinishBehavior::Loop,
            ..default()
        },
    ));
}

const MOE: &str = "sprite/moe.png";
const BIGMOE: &str = "sprite/bigmoe.png";
