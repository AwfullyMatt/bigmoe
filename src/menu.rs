use crate::loading::ImageAssets;
use crate::{AnimationData, GameState};
use bevy::prelude::*;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTitleSleep>()
            .add_systems(OnEnter(GameState::Menu), spawn_title)
            .add_systems(
                Update,
                (animate_title, animate_title_sleep, spawn_title_sleep)
                    .run_if(in_state(GameState::Menu)),
            );
    }
}
#[derive(Component)]
struct Title;
#[derive(Component)]
struct TitleSleep;
#[derive(Event)]
struct SpawnTitleSleep;

fn spawn_title(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: image_assets.title.clone(),
            transform: Transform::from_scale(Vec3::new(10., 10., 0.)),
            ..default()
        },
        AnimationData {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            frames: 16,
            count: 0,
        },
        Title,
    ));
}
fn spawn_title_sleep(
    mut ev_spawn_title_sleep: EventReader<SpawnTitleSleep>,
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    query_title: Query<Entity, With<Title>>,
) {
    for _ev in ev_spawn_title_sleep.read() {
        commands.spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: image_assets.titlesleep.clone(),
                transform: Transform::from_scale(Vec3::new(10., 10., 1.)),
                ..default()
            },
            AnimationData {
                timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                frames: 8,
                count: 0,
            },
            TitleSleep,
        ));
        if let Ok(entity) = query_title.get_single() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn animate_title(
    time: Res<Time>,
    mut query_data: Query<(&mut AnimationData, &mut TextureAtlasSprite), With<Title>>,
    mut ev_spawn_title_sleep: EventWriter<SpawnTitleSleep>,
) {
    for (mut data, mut sprite) in &mut query_data {
        data.timer.tick(time.delta());
        if data.timer.just_finished() {
            sprite.index = (sprite.index + 1) % data.frames;
            data.count += 1;
            if data.count >= data.frames - 1 {
                ev_spawn_title_sleep.send(SpawnTitleSleep);
            }
        }
    }
}

fn animate_title_sleep(
    time: Res<Time>,
    mut query_data: Query<(&mut AnimationData, &mut TextureAtlasSprite), With<TitleSleep>>,
) {
    for (mut data, mut sprite) in &mut query_data {
        data.timer.tick(time.delta());
        if data.timer.just_finished() {
            sprite.index = (sprite.index + 1) % data.frames;
            data.count += 1;
        }
    }
}
