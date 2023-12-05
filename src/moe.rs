use crate::GameState;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct MoePlugin;
impl Plugin for MoePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MoeAction>::default())
            .init_resource::<ActionState<MoeAction>>()
            .insert_resource(MoeAction::moe_input_map())
            .add_state::<Moed>()
            .add_systems(OnEnter(GameState::Playing), spawn_moe);
    }
}

#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Moed {
    #[default]
    Minor,
    Major,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum MoeAction {
    Attack,
    Defend,
    Pause,
}

impl MoeAction {
    fn moe_input_map() -> InputMap<MoeAction> {
        InputMap::new([
            (UserInput::from(MouseButton::Right), Self::Defend),
            (UserInput::from(MouseButton::Left), Self::Attack),
        ])
    }
}

fn spawn_moe() {
    // let sprite = sprites.load_animated(MOE, 1);
    // commands.spawn((
    //     Name::from("Moe"),
    //     PxSpriteBundle::<Layers> {
    //         sprite,
    //         position: PxPosition::from(IVec2 { x: 24, y: 81 }),
    //         layer: Layers::Moe(0),
    //         ..default()
    //     },
    //     PxAnimationBundle {
    //         on_finish: PxAnimationFinishBehavior::Loop,
    //         ..default()
    //     },
    // ));
    // info!("Spawned Moe!");
}
