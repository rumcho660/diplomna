mod menu;
mod timer;
mod sound;
mod player;
mod gamestory;
mod enemy;
mod rooms;

use bevy:: prelude::*;
use bevy::app::AppExit;
use bevy::text::Text2dBundle;
use bevy::window::close_on_esc;
use bevy_kira_audio::AudioPlugin;
use bevy_kira_audio::Audio;
use bevy_kira_audio::AudioControl;
use crate::enemy::EnemyPlugin;
use crate::gamestory::GameStoryPlugin;
use crate::menu::{MenusPlugin, quit_button_clicked, setup_menu, start_button_clicked};
use crate::player::{PlayerPlugin, Position, spawn_player};
use crate::rooms::RoomsPlugin;
use crate::sound::audio_game;
use crate::timer::{destroy_timer_el, timer_til_game_end, TimerEndGame, TimerPlugin};

const WINDOW_HEIGHT:f32 = 700.0;
const WINDOW_WIDTH:f32 = 900.0;
const MARGIN: f32 = 200.;

pub const SPRITE_ENEMY_SIZE: Vec2 = Vec2::new(32.0, 32.0);
pub const SPRITE_PlAYER_SIZE: Vec2 = Vec2::new(32.0, 32.0);
pub const SPRITE_SYRINGE_SIZE: Vec2 = Vec2::new(16.0, 16.0);



#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum GameState {
    GameStory,
    MainMenu,
    MainRoom,
    Room1,
    Room2,
    GameOver
}

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
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
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
        .add_plugin(RoomsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(audio_game)
        .add_system(close_on_esc)
        .run();

}
