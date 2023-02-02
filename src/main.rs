mod menu;
mod timer;
mod sound;
mod player;
mod gamestory;
mod enemy;

use bevy:: prelude::*;
use bevy::app::AppExit;
use bevy::text::Text2dBundle;
use bevy::window::close_on_esc;
use bevy_kira_audio::AudioPlugin;
use bevy_kira_audio::Audio;
use bevy_kira_audio::AudioControl;
use crate::enemy::EnemyPlugin;
use crate::gamestory::GameStoryPlugin;
use crate::menu::{GameState, MenusPlugin, quit_button_clicked, setup_menu, start_button_clicked};
use crate::player::{PlayerPlugin, Position, spawn_player};
use crate::sound::audio_game;
use crate::timer::{destroy_timer_el, timer_til_game_end, TimerEndGame, TimerPlugin};


fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(TimerEndGame(Timer::from_seconds(11.0, TimerMode::Once)))
        .insert_resource(Position{x:0.0, y:0.0})
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor{
                title: "Dr. Covid".to_string(),
                ..default()
            },
            ..default()
        })
            .set(ImagePlugin::default_nearest()))
        .add_state(GameState::GameStory)
        .add_startup_system(setup_camera)
        .add_plugin(AudioPlugin)
        .add_plugin(GameStoryPlugin)
        .add_plugin(MenusPlugin)
        .add_plugin(TimerPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(audio_game)
        .add_system(close_on_esc)
        .run();

}
