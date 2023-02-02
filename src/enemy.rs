use bevy:: prelude::*;
use crate::menu::GameState;
use crate::player::{Player, Velosity};


#[derive(Component)]
pub struct Enemy;


#[derive(Component)]
pub struct EnemyPlugin;

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>){
    let enemy = asset_server.load("Covid_enemy1.png");


    commands.spawn(SpriteBundle {
        texture: enemy.clone(),
        sprite: Sprite {
            color: Default::default(),
            flip_x: false,
            flip_y: false,
            custom_size: Option::from((Vec2::new(100.0, 100.0))),
            ..default()
        },
        transform: Transform::from_xyz(0., 300., 0.),
        ..default()
    })
        .insert(Enemy)
        .insert(Velosity{x: 0.0, y: 0.0});
}


pub fn despawn_enemy(mut commands: Commands, mut query: Query<Entity, With<Enemy>>){
    for enemy in query.iter_mut(){
        commands.entity(enemy).despawn();
    }
}

pub fn enemy_movement (start_point: Vec3, end_point: Vec3) -> Vec3 {
    let mut direction = Vec3::new(0., 0., 0.);

    if start_point.x > end_point.x {
        direction += Vec3::new(-1.0, 0., 0.);
    }

    if start_point.x < end_point.x {
        direction += Vec3::new(1.0, 0., 0.);
    }

    if start_point.y > end_point.y {
        direction += Vec3::new(0., -1.0, 0.);
    }

    if start_point.y < end_point.y {
        direction += Vec3::new(0., 1.0, 0.);
    }

    if start_point.y == end_point.y && start_point.x == end_point.x{

    }

    return direction;
}

pub fn move_enemy(mut query: Query<(&mut Transform), (With<Enemy>, Without<Player>)>,
                  player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
                  mut app_state: ResMut<State<GameState>>) {
    for (mut enemy_pos) in query.iter_mut() {
        for player_pos in player_query.iter() {

            let direction = enemy_movement(enemy_pos.translation, player_pos.translation);
            let mut translation = &mut enemy_pos.translation;


            translation.x += direction.x * 2.0;
            translation.y += direction.y * 2.0;
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