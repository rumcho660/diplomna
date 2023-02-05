use bevy:: prelude::*;
use crate::GameState;



pub struct GameStage {
    pub rooms_1: [i8; 5],
    pub rooms_2: [i8; 5],
    pub enemies: i32
}

#[derive(Component)]
pub struct RoomsPlugin;

#[derive(Component)]
pub struct MainRoom;

pub fn spawn_main_room(mut commands: Commands, asset_surver: Res<AssetServer>){

    let mut main_room = asset_surver.load("");

    commands.spawn(
        SpriteBundle {
            texture: main_room.clone(),
            transform: Transform {
                translation: Vec3::new(7.0, 7.0, 0.0),
                scale: Vec3::new(6.0, 6.0, 0.0),
                ..default()
            },
            ..default()
        }
    ).insert(MainRoom);

}



impl Plugin for RoomsPlugin{
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainRoom)
            .with_system(spawn_main_room))
        .add_system_set(SystemSet::on_enter(GameState::Room1)
            .with_system(spawn_main_room))
        .add_system_set(SystemSet::on_enter(GameState::Room2)
            .with_system(spawn_main_room));
    }
}