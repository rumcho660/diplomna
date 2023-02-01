use bevy::prelude::*;
use bevy::transform;
use bevy::transform::TransformSystem;
use crate::menu::GameState;


const TIME_STEP: f32 = 1.0/60.0;
const SPEED: f32 = 200.0;



#[derive(Component)]
pub struct PlayerPlugin;


#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct Position{
    pub x: f32,
    pub y: f32,
}







pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>){
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
            ..default()
        }).insert(Player);
}


pub fn despawn_player(mut commands: Commands, query: Query< Entity, With<Player>>){
    for player in query.iter(){
        commands.entity(player).despawn();
    }
}

pub fn move_player(keyboard_input: Res<Input<KeyCode>>, mut position: ResMut<Position>, mut query: Query<&mut Transform, With<Player>>) {
    for mut _transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::D) {
            position.x += 1.0 * TIME_STEP * SPEED;

        }

        if keyboard_input.pressed(KeyCode::A) {
            position.x -= 1.0 * TIME_STEP * SPEED;

        }

        if keyboard_input.pressed(KeyCode::W) {
            position.y += 1.0 * TIME_STEP * SPEED;

        }

        if keyboard_input.pressed(KeyCode::S) {
            position.y -= 1.0 * TIME_STEP * SPEED;

        }

        let mut transtalion =  &mut _transform.translation;
        transtalion.x = position.x;
        transtalion.y = position.y;
    }
}


pub fn player_attack(){

}

impl Plugin for PlayerPlugin  {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainGame)
                .with_system(spawn_player))
            .add_system_set(SystemSet::on_update(GameState::MainGame)
                .with_system(move_player)
                .with_system(player_attack))
            .add_system_set(SystemSet::on_enter(GameState::GameOver)
                .with_system(despawn_player));
    }
}