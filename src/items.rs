use bevy:: prelude::*;
use bevy::math::Vec3Swizzles;
use bevy::sprite::collide_aabb::collide;
use crate::{GameState, SPRITE_DAMAGE_UP_SIZE, SPRITE_DOUBLE_SHOT_SIZE, SPRITE_PLAYER_SIZE, SPRITE_SPEED_UP_SIZE};
use crate::player::{Damage, DoubleShot, Player, Speed};

#[derive(Component)]
pub struct DamageUp;

#[derive(Component)]
pub struct SpeedUp;

#[derive(Component)]
pub struct MoreSyringes;

#[derive(Component)]
pub struct ItemsPlugin;

pub fn spawn_items(mut commands: Commands,
                   asset_server: Res<AssetServer>){
    let damage_up =  asset_server.load("Damage_up.png");
    let speed_up =  asset_server.load("Speed_Up.png");
    let more_syringes =  asset_server.load("More_syringes.png");

    commands.spawn(SpriteBundle{
        texture: damage_up.clone(),
        transform: Transform{
            translation: Vec3::new(-80.0, -100.0, 1.0),
            scale: Vec3::splat(2.5),
            ..default()
        },
        ..default()


    }).insert(DamageUp);

    commands.spawn(SpriteBundle{
        texture: speed_up.clone(),
        transform: Transform{
            translation: Vec3::new(80.0, -100.0, 1.0),
            scale: Vec3::splat(2.5),
            ..default()
        },
        ..default()


    }).insert(SpeedUp);

    commands.spawn(SpriteBundle{
        texture: more_syringes.clone(),
        transform: Transform{
            translation: Vec3::new(0.0, -100.0, 1.0),
            scale: Vec3::splat(2.5),
            ..default()
        },
        ..default()


    }).insert(MoreSyringes);
}
pub fn pick_items(mut commands: Commands,
                  mut query_player: Query<(&Transform, &mut Speed, &mut DoubleShot, &mut Damage), With<Player>>,
                  query_speed: Query<(Entity, &Transform), With<SpeedUp>>,
                  query_double_shot: Query<(Entity, &Transform), With<MoreSyringes>>,
                  query_damage_up: Query<(Entity, &Transform), With<DamageUp>>){
    for (transform_player, mut speed, mut double_shot, mut damage) in query_player.iter_mut() {
        let player_scale = Vec2::from(transform_player.scale.xy());

        for (item_speed, transform_item_speed) in query_speed.iter() {
            let speed_item_scale = Vec2::from(transform_item_speed.scale.xy());

            let collide_speed_item = collide(
                transform_player.translation,
                SPRITE_PLAYER_SIZE * player_scale,
                transform_item_speed.translation,
                SPRITE_SPEED_UP_SIZE * speed_item_scale,
            );

            if let Some(_) = collide_speed_item {
                speed.value = 400.0;
                commands.entity(item_speed).despawn();
            }
        }

        for (item_double_shot, transform_item_double_shot) in query_double_shot.iter() {
            let double_shot_item_scale = Vec2::from(transform_item_double_shot.scale.xy());

            let collide_double_shot_item = collide(
                transform_player.translation,
                SPRITE_PLAYER_SIZE * player_scale,
                transform_item_double_shot.translation,
                SPRITE_DOUBLE_SHOT_SIZE * double_shot_item_scale,
            );

            if let Some(_) = collide_double_shot_item {
                double_shot.value = true;
                commands.entity(item_double_shot).despawn();
            }
        }

        for (item_damage_up, transform_item_damage_up) in query_damage_up.iter() {
            let damage_up_item_scale = Vec2::from(transform_item_damage_up.scale.xy());

            let collide_damage_up_item = collide(
                transform_player.translation,
                SPRITE_PLAYER_SIZE * player_scale,
                transform_item_damage_up.translation,
                SPRITE_DAMAGE_UP_SIZE * damage_up_item_scale,
            );

            if let Some(_) = collide_damage_up_item {
                damage.value = 4;
                commands.entity(item_damage_up).despawn();
            }
        }


    }
}


pub fn despawn_damage_up(mut commands: Commands,
                        query: Query<Entity, With<DamageUp>>){
    for damage_up in query.iter(){
        commands.entity(damage_up).despawn_recursive();
    }
}

pub fn despawn_speed_up(mut commands: Commands,
                        query: Query<Entity, With<SpeedUp>>){
    for speed_up in query.iter(){
        commands.entity(speed_up).despawn_recursive();
    }
}

pub fn despawn_double_shot(mut commands: Commands,
                        query: Query<Entity, With<MoreSyringes>>){
    for double_shot in query.iter(){
        commands.entity(double_shot).despawn_recursive();
    }
}


impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainRoom)
            .with_system(spawn_items))
        .add_system_set(SystemSet::on_update(GameState::MainRoom)
            .with_system(pick_items))
        .add_system_set(SystemSet::on_exit(GameState::MainRoom)
            .with_system(despawn_damage_up)
            .with_system(despawn_speed_up)
            .with_system(despawn_double_shot));
    }
}