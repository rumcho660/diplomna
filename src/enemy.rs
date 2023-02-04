use bevy:: prelude::*;
use bevy::sprite::collide_aabb::Collision;
use crate::menu::GameState;
use crate::player::{Player, Velosity};


#[derive(Component)]
pub struct Enemy;


#[derive(Component)]
pub struct EnemyPlugin;


#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimerEnemy(Timer);



pub fn collision_with_player(layer: Collision){

}

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>){
    let texture_handle = asset_server.load("Enemy_final.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);


    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(3.5)),
            ..default()
        },
        AnimationTimerEnemy(Timer::from_seconds(0.1, TimerMode::Repeating)),
    )).insert(Enemy)
        .insert(Velosity{x: 0.0, y: 0.0});
}


pub fn despawn_enemy(mut commands: Commands, mut query: Query<Entity, With<Enemy>>){
    for enemy in query.iter_mut(){
        commands.entity(enemy).despawn();
    }
}



pub fn move_enemy(mut query: Query<(&mut Velosity, &mut Transform), (With<Enemy>, Without<Player>)>,
                  player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
                  time: Res<Time>,
                  texture_atlases: Res<Assets<TextureAtlas>>,
                  mut query_animation: Query<(&mut AnimationTimerEnemy, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>) {

    for (mut velocity, mut enemy_pos) in query.iter_mut() {
        for player_pos in player_query.iter() {

            if enemy_pos.translation.x > player_pos.translation.x {
                velocity.x = -0.01;
            }

            if enemy_pos.translation.x < player_pos.translation.x {
                velocity.x = 0.01;
            }

            if enemy_pos.translation.y > player_pos.translation.y {
                velocity.y = -0.01;
            }

            if enemy_pos.translation.y < player_pos.translation.y {
                velocity.y = 0.01;
            }


            for (mut timer, mut sprite, texture_atlas_handle) in &mut query_animation {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
            }

            let mut translation = &mut enemy_pos.translation;


            translation.x += velocity.x * 20.0;
            translation.y += velocity.y * 20.0;

        }

    }
}


impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainGame)
            .with_system(spawn_enemy))
        .add_system_set(SystemSet::on_update(GameState::MainGame)
            .with_system(move_enemy))
        .add_system_set(SystemSet::on_enter(GameState::GameOver)
            .with_system(despawn_enemy));
    }
}