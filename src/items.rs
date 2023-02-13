use bevy:: prelude::*;
use bevy::math::Vec3Swizzles;
use bevy::sprite::collide_aabb::collide;
use crate::{GameState, SPRITE_DAMAGE_UP_SIZE, SPRITE_DOUBLE_SHOT_SIZE, SPRITE_PLAYER_SIZE, SPRITE_SPEED_UP_SIZE};
use crate::player::{Damage, DoubleShot, Player, Speed, Syringe};

#[derive(Component)]
pub struct DamageUp;

#[derive(Component)]
pub struct SpeedUp;

#[derive(Component)]
pub struct MoreSyringes;

#[derive(Component)]
pub struct ItemsPlugin;

pub fn spawn_items(mut commands: Commands, asset_server: Res<AssetServer>){
    let damage_up =  asset_server.load("Damage_Up.png");
    let speed_up =  asset_server.load("Speed_Up.png");
    let more_syringes =  asset_server.load("More_syringes.png");

    commands.spawn(SpriteBundle{
        texture: damage_up.clone(),
        transform: Transform{
            translation: Vec3::new(-80.0, -100.0, 1.0),
            scale: Vec3::splat(3.0),
            ..default()
        },
        ..default()


    }).insert(DamageUp);

    commands.spawn(SpriteBundle{
        texture: speed_up.clone(),
        transform: Transform{
            translation: Vec3::new(80.0, -100.0, 1.0),
            scale: Vec3::splat(3.0),
            ..default()
        },
        ..default()


    }).insert(SpeedUp);

    commands.spawn(SpriteBundle{
        texture: more_syringes.clone(),
        transform: Transform{
            translation: Vec3::new(0.0, -100.0, 1.0),
            scale: Vec3::splat(3.0),
            ..default()
        },
        ..default()


    }).insert(MoreSyringes);
}
pub fn pick_items(mut commands: Commands,
                  mut query_player: Query<(&Transform, &mut Speed, &mut DoubleShot), (With<Player>, Without<SpeedUp>, Without<MoreSyringes>, Without<DamageUp>)>,
                  query_speed: Query<(Entity, &Transform), (With<SpeedUp>, Without<Player>, Without<MoreSyringes>, Without<DamageUp>)>,
                  query_double_shot: Query<(Entity, &Transform),(With<MoreSyringes>, Without<Player>, Without<SpeedUp>, Without<DamageUp>)>,
                  query_damage_up: Query<(Entity, &Transform),(With<DamageUp>, Without<Player>, Without<SpeedUp>, Without<MoreSyringes>)>,
                  mut query_syringe: Query<&mut Damage, With<Syringe>>){
    let mut flag = 0;
    for (transform_player, mut speed, mut double_shot) in query_player.iter_mut() {
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
                flag += 1;
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
                flag += 1;
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
                for mut syringe_damage in query_syringe.iter_mut(){
                    syringe_damage.value = 3;
                    println!("damage up");
                }
                commands.entity(item_damage_up).despawn();
                flag += 1;
            }
        }


    }
}

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainRoom)
            .with_system(spawn_items))
        .add_system_set(SystemSet::on_update(GameState::MainRoom)
            .with_system(pick_items));
    }
}