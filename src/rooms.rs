use bevy:: prelude::*;
use bevy::math::Vec3Swizzles;
use rand::Rng;
use rand::thread_rng;
use bevy::sprite::collide_aabb::collide;
use crate::{GameState, SPRITE_BED_SIZE, SPRITE_PLAYER_SIZE, SPRITE_SOMETHING_SIZE, SPRITE_WALL_SIZE, TypeDeath, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::enemy::{spawn_enemy_wave1, spawn_enemy_wave2};
use crate::player::{Health, LimitDeads, Player};


pub const ROOMS_SIZE: f32 = 8.0;

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
pub struct SalineStand;

#[derive(Component)]
pub struct MainFloor;


pub fn wall_blocks_build(mut commands: Commands,
                         asset_server: Res<AssetServer>){
    let block = asset_server.load("Block.png");


    let mut x1 = 0.0;
    let mut x2 = 0.0;
    let mut x3 = 0.0;
    let mut x4 = 0.0;
    while x1 < WINDOW_WIDTH/2.0 {
        commands.spawn(
            SpriteBundle {
                texture: block.clone(),
                transform: Transform{
                    translation: Vec3::new(x1, -WINDOW_HEIGHT/2.0 + 32.0, 1.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Wall);
        x1 += 64.0;

        commands.spawn(
            SpriteBundle {
                texture: block.clone(),
                transform: Transform{
                    translation: Vec3::new(x2, -WINDOW_HEIGHT/2.0 + 32.0, 1.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Wall);
        x2 -= 64.0;

        commands.spawn(
            SpriteBundle {
                texture: block.clone(),
                transform: Transform{
                    translation: Vec3::new(x3, WINDOW_HEIGHT/2.0 - 32.0, 1.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Wall);
        x3 += 64.0;


        commands.spawn(
            SpriteBundle {
                texture: block.clone(),
                transform: Transform{
                    translation: Vec3::new(x4, WINDOW_HEIGHT/2.0 - 32.0, 1.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Wall);
        x4 -= 64.0;
    }



    let mut y1 = 0.0;
    let mut y2 = 0.0;
    let mut y3 = 0.0;
    let mut y4 = 0.0;
    while y1 < WINDOW_HEIGHT/2.0{
        commands.spawn(
            SpriteBundle {
                texture: block.clone(),
                transform: Transform{
                    translation: Vec3::new(-WINDOW_WIDTH/2.0 + 32.0, y1, 1.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Wall);

        y1 += 64.0;

        commands.spawn(
            SpriteBundle {
                texture: block.clone(),
                transform: Transform{
                    translation: Vec3::new(-WINDOW_WIDTH/2.0 + 32.0, y2, 1.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Wall);

        y2 -= 64.0;

        commands.spawn(
            SpriteBundle {
                texture: block.clone(),
                transform: Transform{
                    translation: Vec3::new(WINDOW_WIDTH/2.0 - 32.0, y3, 1.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Wall);

        y3 += 64.0;


        commands.spawn(
            SpriteBundle {
                texture: block.clone(),
                transform: Transform{
                    translation: Vec3::new(WINDOW_WIDTH/2.0 - 32.0, y4, 1.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Wall);

        y4 -= 64.0;

    }
}

pub fn room1_furniture(commands: &mut Commands,
                       asset_server: &AssetServer){
    let bed = asset_server.load("Bed.png");
    let mut right_side= 0.0;
    let mut left_side= 0.0;
    let mut i =0;

    while i <=2{

        commands.spawn(
            SpriteBundle {
                texture: bed.clone(),
                transform: Transform{
                    translation: Vec3::new(WINDOW_WIDTH/2.0 - 200.0, right_side, 1.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Bed);

        right_side += 90.0;

        commands.spawn(
            SpriteBundle {
                texture: bed.clone(),
                transform: Transform{
                    translation: Vec3::new(-WINDOW_WIDTH/2.0 + 200.0, left_side, 1.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            }
        ).insert(Bed);

        left_side -= 90.0;

        i += 1;
    }
}

pub fn room2_furniture(commands: &mut Commands,
                       asset_server: &AssetServer){
    let saline_stand = asset_server.load("Saline_stand.png");


    commands.spawn(
        SpriteBundle {
            texture: saline_stand.clone(),
            transform: Transform {
                translation: Vec3::new(170.0, 90.0, 1.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            ..default()
        }
    ).insert(SalineStand);


    commands.spawn(
        SpriteBundle {
            texture: saline_stand.clone(),
            transform: Transform{
                translation: Vec3::new(-170.0, 90.0, 1.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            ..default()
        }
    ).insert(SalineStand);

    commands.spawn(
        SpriteBundle {
            texture: saline_stand.clone(),
            transform: Transform {
                translation: Vec3::new(-20.0, 220.0, 1.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            ..default()
        }
    ).insert(SalineStand);


    commands.spawn(
        SpriteBundle {
            texture: saline_stand.clone(),
            transform: Transform{
                translation: Vec3::new(-20.0, -90.0, 1.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            ..default()
        }
    ).insert(SalineStand);
}

pub fn choose_room(mut commands: &mut Commands,
                   asset_server: &AssetServer,
                   mut texture_atlases: &mut Assets<TextureAtlas>,
                   mut limit_deads: &mut LimitDeads) -> &'static str {
    let range = 0.0f64..100.0f64;
    let odds = thread_rng().gen_range(range);
    let mut room= "";

    if odds>=0.0 && odds<=50.0{
        room1_furniture(&mut commands, &asset_server);
        spawn_enemy_wave1(&mut commands, &asset_server, &mut texture_atlases);
        limit_deads.0 += 4;
        room = "Room1.png";
    }



    else if odds>=50.0 && odds<=100.0{
        room2_furniture(&mut commands, &asset_server);
        spawn_enemy_wave2(&mut commands, &asset_server, &mut texture_atlases);
        limit_deads.0 += 3;
        room = "Room2.png";
    }
    return room;
}


pub fn spawn_main_room(mut commands: Commands, asset_server: Res<AssetServer>){
    let main_floor = asset_server.load("Main_room.png");

    commands.spawn(
        SpriteBundle {
            texture: main_floor.clone(),
            transform: Transform{
                scale: Vec3::splat(ROOMS_SIZE),
                ..default()
            },
            ..default()
        }
    ).insert(MainFloor);
}



pub fn spawn_room1(mut commands: Commands,
                   asset_server: Res<AssetServer>,
                   mut texture_atlases:  ResMut<Assets<TextureAtlas>>,
                   mut limit_deads: ResMut<LimitDeads>){

    let room1= asset_server.load(choose_room(&mut commands, &asset_server, &mut texture_atlases, &mut limit_deads));

    commands.spawn(
        SpriteBundle {
            texture: room1.clone(),
            transform: Transform{
                scale: Vec3::splat(ROOMS_SIZE),
                ..default()
            },
            ..default()
        }
    ).insert(Room1);

}



pub fn spawn_room2(mut commands: Commands,
                   asset_server: Res<AssetServer>,
                   mut texture_atlases:  ResMut<Assets<TextureAtlas>>,
                   mut limit_deads: ResMut<LimitDeads>){

    let room2= asset_server.load(choose_room(&mut commands, &asset_server, &mut texture_atlases, &mut limit_deads));

    commands.spawn(
        SpriteBundle {
            texture: room2.clone(),
            transform: Transform{
                scale: Vec3::splat(ROOMS_SIZE),
                ..default()
            },
            ..default()
        }
    ).insert(Room2);

}






//despawns
pub fn despawn_blocks(mut commands: Commands,
                      query: Query< Entity, With<Wall>>){
    for blocks in query.iter(){
        commands.entity(blocks).despawn();
    }
}

pub fn despawn_main_floor(mut commands: Commands,
                          query: Query< Entity, With<MainFloor>>){
    for floor in query.iter(){
        commands.entity(floor).despawn();
    }
}

pub fn despawn_room1(mut commands: Commands,
                     query: Query< Entity, With<Room1>>){
    for room1 in query.iter(){
        commands.entity(room1).despawn();
    }
}

pub fn despawn_room2(mut commands: Commands,
                     query: Query< Entity, With<Room1>>){
    for room2 in query.iter(){
        commands.entity(room2).despawn();
    }
}

pub fn despawn_bed(mut commands: Commands,
                   query: Query< Entity, With<Bed>>){
    for beds in query.iter(){
        commands.entity(beds).despawn();
    }
}

pub fn despawn_something(mut commands: Commands,
                         query: Query< Entity, With<SalineStand>>){
    for saline_stand in query.iter(){
        commands.entity(saline_stand).despawn();
    }
}








pub fn hitting_objects(mut app_state: ResMut<State<GameState>>,
                       mut commands: Commands,
                       mut query_player: Query<(Entity, &Transform, &mut Health), (With<Player>, Without<Wall>, Without<Bed>, Without<SalineStand>)>,
                       query_wall: Query<&Transform, (With<Wall>, Without<Player>, Without<Bed>, Without<SalineStand>)>,
                       query_bed: Query<&Transform, (With<Bed>, Without<Player>, Without<Wall>, Without<SalineStand>)>,
                       query_saline_stand: Query<&Transform, (With<SalineStand>, Without<Player>, Without<Bed>, Without<Wall>)>,
                       mut type_dead: ResMut<TypeDeath> ){

    for (entity, transform_player, mut health) in query_player.iter_mut(){
        let player_scale = Vec2::from(transform_player.scale.xy());

        for  transform_wall in query_wall.iter()  {
            let wall_scale = Vec2::from(transform_wall.scale.xy());

            let collide_wall = collide(
                transform_player.translation,
                SPRITE_PLAYER_SIZE * player_scale,
                transform_wall.translation,
                SPRITE_WALL_SIZE * wall_scale,
            );

            if let Some(_) = collide_wall{


                type_dead.0 = 2;

                health.value -= 300;
                commands.entity(entity).despawn();
                app_state.set(GameState::GameOver);
            }
        }


        for  transform_bed in query_bed.iter()  {
            let bed_scale = Vec2::from(transform_bed.scale.xy());

            let collide_bed = collide(
                transform_player.translation,
                SPRITE_PLAYER_SIZE * player_scale,
                transform_bed.translation,
                SPRITE_BED_SIZE * bed_scale,
            );


            if let Some(_) = collide_bed{

                health.value -= 1;
                if health.value <= 0{
                    type_dead.0 = 3;
                    commands.entity(entity).despawn();
                    app_state.set(GameState::GameOver);
                }
            }
        }


        for  transform_saline_stand in query_saline_stand.iter()  {
            let saline_stand_scale = Vec2::from(transform_saline_stand.scale.xy());

            let collide_saline_stand = collide(
                transform_player.translation,
                SPRITE_PLAYER_SIZE * player_scale,
                transform_saline_stand.translation,
                SPRITE_SOMETHING_SIZE * saline_stand_scale,
            );


            if let Some(_) = collide_saline_stand{

                health.value -= 1;
                if health.value <= 0{
                    type_dead.0 = 4;
                    commands.entity(entity).despawn();
                    app_state.set(GameState::GameOver);
                }
            }
        }
    }
}






impl Plugin for RoomsPlugin{
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainRoom)
            .with_system(spawn_main_room)
            .with_system(wall_blocks_build))
            .add_system_set(SystemSet::on_update(GameState::MainRoom)
                .with_system(hitting_objects))
            .add_system_set(SystemSet::on_enter(GameState::Room1)
                .with_system(despawn_main_floor)
                .with_system(spawn_room1))
            .add_system_set(SystemSet::on_update(GameState::Room1)
                .with_system(hitting_objects))
            .add_system_set(SystemSet::on_enter(GameState::Room2)
                .with_system(despawn_room1)
                .with_system(despawn_bed)
                .with_system(despawn_something)
                .with_system(spawn_room2))
            .add_system_set(SystemSet::on_update(GameState::Room2)
                .with_system(hitting_objects))
            .add_system_set(SystemSet::on_enter(GameState::GameOver)
                .with_system(despawn_something)
                .with_system(despawn_bed)
                .with_system(despawn_blocks)
                .with_system(despawn_room2));
    }
}