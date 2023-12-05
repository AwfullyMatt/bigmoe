use crate::SongState;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use std::time::Duration;

pub struct MetronomePlugin;
impl Plugin for MetronomePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBeat>()
            .insert_resource(CurrentSong::default())
            .add_systems(
                Update,
                (
                    spawn_beat,
                    despawn_beat,
                    mark_active,
                    mark_hittable,
                    tick_beat_timers,
                    tick_current_song_timer,
                )
                    .run_if(in_state(SongState::Playing)),
            );
    }
}

#[derive(Event, Clone)]
pub struct SpawnBeat(pub Beat);

#[derive(Component, Clone)]
pub struct Beat {
    pub delivery: Timer,
    pub duration: Option<Timer>,
    pub hittable: bool,
    pub index: usize,
    pub spawn_time: f32,
}
#[derive(Component, Clone)]
pub struct Active;

#[derive(Clone)]
pub struct Song {
    pub beats: Vec<Beat>,
    pub cbi: usize,
    pub sbi: usize,
    pub duration: f32,
    pub tempo: Tempo,
}
#[derive(Clone, Copy, Default)]
pub struct Tempo {
    pub top: u32,
    pub bottom: u32,
    pub bpm: u32,
}

// resources

#[derive(Resource, Clone, Default)]
pub struct CurrentSong {
    pub song: Option<Song>,
    pub stopwatch: Stopwatch,
}

// systems

fn tick_current_song_timer(mut current_song: ResMut<CurrentSong>, time: Res<Time>) {
    current_song.stopwatch.tick(time.delta());
}
fn tick_beat_timers(mut query_beat: Query<&mut Beat>, time: Res<Time>) {
    for mut beat in query_beat.iter_mut() {
        beat.delivery.tick(time.delta());
        if let Some(mut timer) = beat.duration.clone() {
            timer.tick(time.delta());
        }
    }
}
// fn timing(query_beat: Query<&Beat, With<Active>>, query_action: Query<&ActionState<MoeAction>>) {
// if let Ok(action_state) = query_action.get_single() {
//     if action_state.just_pressed(MoeAction::Attack)
//         || action_state.just_pressed(MoeAction::Defend)
//     {
//         if let Ok(beat) = query_beat.get_single() {
//             match beat.hittable {
//                 true => {
//                     info!("On Time.");
//                 }
//                 false => {
//                     info!("Off Time.");
//                 }
//             }
//         } else {
//             info!("No Active Beat.")
//         }
//     }
// }
// }
fn mark_active(
    mut query_beat: Query<(Entity, &mut Beat), Without<Active>>,
    mut commands: Commands,
    current_song: Res<CurrentSong>,
) {
    if let Some(song) = &current_song.song {
        for (entity, beat) in query_beat.iter_mut() {
            if beat.index == song.cbi {
                commands.entity(entity).insert(Active);
            }
        }
    }
}
fn mark_hittable(mut query_beat: Query<&mut Beat, With<Active>>) {
    for mut beat in query_beat.iter_mut() {
        if beat.delivery.percent() >= 0.95 {
            beat.hittable = true;
        }
    }
}
fn spawn_beat(mut ev_spawn_beat: EventWriter<SpawnBeat>, current_song: Res<CurrentSong>) {
    if let Some(mut song) = current_song.song.clone() {
        if let Some(beat) = song.beats.iter().nth(song.cbi) {
            if beat.spawn_time <= current_song.stopwatch.elapsed_secs() {
                let delivery = Timer::new(
                    Duration::from_secs_f32((60.0 / song.tempo.bpm as f32) * song.tempo.top as f32),
                    TimerMode::Once,
                );
                let duration = match &beat.duration {
                    None => None,
                    Some(duration) => Some(duration.clone()),
                };
                ev_spawn_beat.send(SpawnBeat(Beat {
                    delivery,
                    duration,
                    hittable: false,
                    index: song.sbi,
                    spawn_time: beat.spawn_time, // ToDo
                                                 // awkward since it's not needed after spawn
                }));
                info!("Spawned Beat {:?}", song.sbi);
                song.sbi += 1;
            }
        }
    }
}
fn despawn_beat(
    query_beat: Query<(Entity, &Beat)>,
    mut commands: Commands,
    mut current_song: ResMut<CurrentSong>,
) {
    for (entity, beat) in query_beat.iter() {
        if let Some(duration) = &beat.duration {
            if duration.finished() {
                commands.entity(entity).despawn_recursive();
                info!("Despawned Beat {:?}.", beat.index);
                if let Some(ref mut song) = &mut current_song.song {
                    song.cbi += 1;
                }
            }
        }
    }
}
