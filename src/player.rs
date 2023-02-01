use bevy::prelude::*;
use crate::menu::GameState;


#[derive(Component)]
pub struct PlayerPlugin;


#[derive(Component)]
pub struct Player;



pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas: ResMut<Assets<TextureAtlas>>){
    let sprite = asset_server.load("Doctor_Covid3.png");


    commands.spawn(SpriteBundle {
            texture: sprite.clone(),
            sprite: Sprite {
                color: Default::default(),
                flip_x: false,
                flip_y: false,
                custom_size: Option::from((Vec2::new(100.0, 100.0))),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        }).insert(Player);
}


pub fn despawn_player(mut commands: Commands, query: Query< Entity, With<Player>>){
    for player in query.iter(){
        commands.entity(player).despawn();
    }
}

pub fn player_move(){


}


impl Plugin for PlayerPlugin  {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainGame)
            .with_system(spawn_player))
            .add_system_set(SystemSet::on_enter(GameState::GameOver)
                .with_system(despawn_player));
    }
}