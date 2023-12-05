use crate::GameState;
use bevy::prelude::*;

pub struct IanPlugin;
impl Plugin for IanPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Ian>()
            .add_systems(OnEnter(GameState::Playing), spawn_ian);
    }
}

#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub enum Ian {
    #[default]
    Minor, // 6. Aeolian -- flat 3/6/7, Natural Minor
    Door,      // 2.  Dorian -- flat 3/7, like minor but not as intense, a bit groovier
    Lid,       // 4. Lydian -- sharp 4, Kinda futury/prog-rock "avoid the V"
    MisterLid, // 5. Mixolydian -- flat 7, AC/DC
    Fridge,    // 3. Phrygian -- flat 2/3/6/7, rock lobster-ish, a bit spooky
    Major,     // 1. Ionian -- Major
    Loco,      // 7. Locrian -- flat 2/3/5/6/7, most awkward
    Frog,      // 3. Phrygian -- flat 2/3/6/7, rock lobster-ish, a bit spooky
}

fn spawn_ian() {
    // let sprite = sprites.load_animated(IAN, 1);
    // commands.spawn((
    //     PxSpriteBundle::<Layers> {
    //         sprite,
    //         position: PxPosition::from(IVec2 { x: 256, y: 81 }),
    //         layer: Layers::Ian(0),
    //         ..default()
    //     },
    //     PxAnimationBundle {
    //         on_finish: PxAnimationFinishBehavior::Loop,
    //         ..default()
    //     },
    // ));
    // info!("Spawned Ian!");
}

// constants
