use bevy:: prelude::*;
use crate::{GameState, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::player::Player;


#[derive(Component)]
pub struct RoomsPlugin;

#[derive(Component)]
pub struct MainRoom;

#[derive(Component)]
pub struct Room1;

#[derive(Component)]
pub struct Room2;


pub fn spawn_main_room(mut commands: Commands, asset_surver: Res<AssetServer>){

    let mut main_room = asset_surver.load("MainRoom.png");

    commands.spawn(
        SpriteBundle {
            texture: main_room.clone(),
            transform: Transform::from_scale(Vec3::splat(26.0)),
            ..default()
        }
    ).insert(MainRoom);

}



pub fn despawn_main_room(mut commands: Commands, query: Query< Entity, With<MainRoom>>){
    for main_room in query.iter(){
        commands.entity(main_room).despawn();
    }
}


pub fn spawn_room1(mut commands: Commands, asset_surver: Res<AssetServer>){



    let mut room1 = asset_surver.load("Room1.png");

    commands.spawn(
        SpriteBundle {
            texture: room1.clone(),
            transform: Transform::from_scale(Vec3::splat(26.0)),
            visibility: Visibility::VISIBLE,
            ..default()
        }
    ).insert(Room1);

}


pub fn despawn_room1(mut commands: Commands, query: Query< Entity, With<Room1>>){
    for room1 in query.iter(){
        commands.entity(room1).despawn();
    }
}


pub fn spawn_room2(mut commands: Commands, asset_surver: Res<AssetServer>){



    let mut room2 = asset_surver.load("Room2.png");

    commands.spawn(
        SpriteBundle {
            texture: room2.clone(),
            transform: Transform::from_scale(Vec3::splat(26.0)),
            visibility: Visibility::VISIBLE,
            ..default()
        }
    ).insert(Room2);

}


pub fn despawn_room2(mut commands: Commands, query: Query< Entity, With<Room1>>){
    for room2 in query.iter(){
        commands.entity(room2).despawn();
    }
}


impl Plugin for RoomsPlugin{
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainRoom)
            .with_system(spawn_main_room))
        .add_system_set(SystemSet::on_enter(GameState::Room1)
            .with_system(despawn_main_room)
            .with_system(spawn_room1))
        .add_system_set(SystemSet::on_enter(GameState::Room2)
            .with_system(despawn_room1)
            .with_system(spawn_room2))
        .add_system_set(SystemSet::on_enter(GameState::GameOver)
            .with_system(despawn_room2));
    }
}