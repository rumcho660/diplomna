mod menu;
mod timer;
mod sound;

use std::time::Duration;
use bevy:: prelude::*;
use bevy::app::AppExit;
use bevy::text::Text2dBundle;
use bevy::window::close_on_esc;
use bevy_kira_audio::AudioPlugin;
use bevy_kira_audio::Audio;
use bevy_kira_audio::AudioControl;
use crate::menu::GameState;
use crate::sound::audio_game;
use crate::timer::{destroy_timer_el, timer_til_game_end, TimerEndGame};


fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(TimerEndGame(Timer::from_seconds(11.0, TimerMode::Once)))
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor{
                title: "Dr. Covid".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_plugin(AudioPlugin)
        .add_state(GameState::MainGame)
        .add_startup_system(setup)
        .add_startup_system(audio_game)
        .add_system(close_on_esc)
        .add_system_set(
            SystemSet::on_update(GameState::MainGame)
                .with_system(timer_til_game_end)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver)
                .with_system(destroy_timer_el)
        )
        .run();

}
