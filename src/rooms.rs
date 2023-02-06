use bevy:: prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::{GameState, WINDOW_HEIGHT, WINDOW_WIDTH};



#[derive(Component)]
pub struct RoomsPlugin;

#[derive(Component)]
pub struct Room1;

#[derive(Component)]
pub struct Room2;


#[derive(Component)]
pub struct Wall;


#[derive(Component)]
pub struct Bed;


#[derive(Component)]
pub struct Something;


pub fn spawn_main_room(mut commands: Commands, asset_surver: Res<AssetServer>){
    let block = asset_surver.load("Block.png");



    commands.spawn(
        SpriteBundle {
            texture: block.clone(),
            transform: Transform{
                translation: Vec3::new(0.0, -WINDOW_HEIGHT/2.0 + 35.0, 0.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            ..default()
        }
    ).insert(Wall);



    commands.spawn(
        SpriteBundle {
            texture: block.clone(),
            transform: Transform{
                translation: Vec3::new(0.0, WINDOW_HEIGHT/2.0 - 35.0, 0.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            ..default()
        }
    ).insert(Wall);



    commands.spawn(
        SpriteBundle {
            texture: block.clone(),
            transform: Transform{
                translation: Vec3::new(-WINDOW_WIDTH/2.0 + 35.0, 0.0, 0.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            ..default()
        }
    ).insert(Wall);


    commands.spawn(
        SpriteBundle {
            texture: block.clone(),
            transform: Transform{
                translation: Vec3::new(WINDOW_WIDTH/2.0 - 35.0, 0.0, 0.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            ..default()
        }
    ).insert(Wall);

}


pub fn enemy_attack(mut commands: Commands, query_player: Query<(Entity, &Transform), With<Player>>, query_enemy: Query<(&Transform), With<Wall>> ){

    for (player, mut health,  transform_player) in query_player.iter_mut(){
        let player_scale = Vec2::from(transform_player.scale.xy());

        for (damage, transform_enemy) in query_enemy.iter()  {
            let enemy_scale = Vec2::from(transform_enemy.scale.xy());

            let collide = collide(
                transform_player.translation,
                SPRITE_PLAYER_SIZE * player_scale,
                transform_enemy.translation,
                SPRITE_ENEMY_SIZE * enemy_scale,
            );


            if let Some(_) = collide{
                health.value -= damage.value;


                if health.value == 0{
                    commands.entity(player).despawn();
                    app_state.set(GameState::GameOver).expect("error in gameover in player.rs");
                }
            }
        }
    }
}


pub fn despawn_main_room(mut commands: Commands, query: Query< Entity, With<Wall>>){
    for walls in query.iter(){
        commands.entity(walls).despawn();
    }
}


pub fn spawn_room1(mut commands: Commands, asset_surver: Res<AssetServer>){



    let room1 = asset_surver.load("Room1.png");

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



    let room2 = asset_surver.load("Room2.png");

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