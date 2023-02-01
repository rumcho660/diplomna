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




#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);




pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>){
    let texture_handle = asset_server.load("doctor_covid_animated.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);


    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    )).insert(Player);
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