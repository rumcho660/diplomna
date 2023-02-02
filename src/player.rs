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


#[derive(Component)]
pub struct Syringe;

#[derive(Component)]
pub struct Velosity{x: f32, y: f32}

#[derive(Resource)]
pub struct Position{
    pub x: f32,
    pub y: f32,
}




#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);




pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>){
    let texture_handle = asset_server.load("Doctor_Covid_final.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);


    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(3.5)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    )).insert(Player)
        .insert(Velosity{x: 0.0, y: 0.0});
}


pub fn despawn_player(mut commands: Commands, query: Query< Entity, With<Player>>){
    for player in query.iter(){
        commands.entity(player).despawn();
    }
}

pub fn move_player(keyboard_input: Res<Input<KeyCode>>, mut position: ResMut<Position>, mut query: Query<&mut Transform, With<Player>>, time: Res<Time>, texture_atlases: Res<Assets<TextureAtlas>>, mut query_animation: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>, )>) {
    for mut _transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::D) {
            position.x += 1.0 * TIME_STEP * SPEED;
            for (mut timer, mut sprite, texture_atlas_handle) in &mut query_animation {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
            }
        }

        if keyboard_input.pressed(KeyCode::A) {
            position.x -= 1.0 * TIME_STEP * SPEED;

            for (mut timer, mut sprite, texture_atlas_handle) in &mut query_animation {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
            }

        }

        if keyboard_input.pressed(KeyCode::W) {
            position.y += 1.0 * TIME_STEP * SPEED;

            for (mut timer, mut sprite, texture_atlas_handle) in &mut query_animation {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
            }

        }

        if keyboard_input.pressed(KeyCode::S) {
            position.y -= 1.0 * TIME_STEP * SPEED;


            for (mut timer, mut sprite, texture_atlas_handle) in &mut query_animation {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
            }

        }

        let mut transtalion =  &mut _transform.translation;
        transtalion.x = position.x;
        transtalion.y = position.y;
    }
}


pub fn player_shoot(keyboard_input: Res<Input<KeyCode>>, query_player: Query<&Transform, With<Player>>, asset_server: Res<AssetServer>, mut commands: Commands, mut position: ResMut<Position>){
    let laser_sprite  = asset_server.load("Syringe.png");


    for player_pos in query_player.iter(){
        if keyboard_input.just_pressed(KeyCode::Right) {
            let x = player_pos.translation.x;
            let y = player_pos.translation.y;



            commands.spawn(SpriteBundle {
                texture: laser_sprite.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    scale: Vec3::new(6.0, 6.0, 0.0),
                    ..default()
                },
                ..default()
            }).insert(Syringe)
                .insert(Velosity{x: 1.0 , y: 0.0});

        }



        if keyboard_input.just_pressed(KeyCode::Left) {
            let x = player_pos.translation.x;
            let y = player_pos.translation.y;


            commands.spawn(SpriteBundle {
                texture: laser_sprite.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    scale: Vec3::new(6.0, 6.0, 0.0),
                    ..default()

                },
                ..default()
            }).insert(Syringe)
                .insert(Velosity{x: -1.0 , y: 0.0});
        }
    }
}


pub fn movable(mut query: Query<(Entity, &Velosity, &mut Transform)>) {
    for (entity, velocity, mut transform) in query.iter_mut() {
        let mut translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * SPEED;
        translation.y += velocity.y * TIME_STEP * SPEED;
    }
}



pub fn despawn_syringes(mut commands: Commands, query: Query<Entity, With<Syringe>>){
    for syringes in query.iter(){
        commands.entity(syringes).despawn_recursive();
    }
}

impl Plugin for PlayerPlugin  {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainGame)
                .with_system(spawn_player))
            .add_system_set(SystemSet::on_update(GameState::MainGame)
                .with_system(player_shoot)
                .with_system(move_player)
                .with_system(movable))
            .add_system_set(SystemSet::on_enter(GameState::GameOver)
                .with_system(despawn_player)
                .with_system(despawn_syringes));
    }
}