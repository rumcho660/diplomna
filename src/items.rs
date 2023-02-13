use bevy:: prelude::*;
use crate::GameState;
use crate::player::{Player, Speed};

#[derive(Component)]
pub struct DamageUp;

#[derive(Component)]
pub struct SpeedUp;

#[derive(Component)]
pub struct MoreSyringes;

#[derive(Component)]
pub struct ItemsPlugin;

pub fn spawn_items(mut commands: Commands, asset_server: Res<AssetServer>){
    let damage_up =  asset_server.load("Damage_Up.png");
    let speed_up =  asset_server.load("Speed_Up.png");
    let more_syringes =  asset_server.load("More_syringes.png");

    commands.spawn(SpriteBundle{
        texture: damage_up.clone(),
        transform: Transform{
            translation: Vec3::new(-80.0, -100.0, 1.0),
            scale: Vec3::splat(3.0),
            ..default()
        },
        ..default()


    }).insert(DamageUp);

    commands.spawn(SpriteBundle{
        texture: speed_up.clone(),
        transform: Transform{
            translation: Vec3::new(80.0, -100.0, 1.0),
            scale: Vec3::splat(3.0),
            ..default()
        },
        ..default()


    }).insert(SpeedUp);

    commands.spawn(SpriteBundle{
        texture: more_syringes.clone(),
        transform: Transform{
            translation: Vec3::new(0.0, -100.0, 1.0),
            scale: Vec3::splat(3.0),
            ..default()
        },
        ..default()


    }).insert(MoreSyringes);
}



impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::MainRoom)
            .with_system(spawn_items));
    }
}