use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use crate::menu::GameState;
use crate::menu::GameState::GameOver;

pub fn audio_game(audio: Res<Audio>,  asset_server: Res<AssetServer>){

    audio.play(asset_server.load("looperman-l-4278453-0321103-free-young-thug-synth-prod-jdabz.wav"));
    audio.set_volume(0.5);
}