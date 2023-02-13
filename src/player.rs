use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::{GameState, SPRITE_ENEMY_SIZE, SPRITE_SYRINGE_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::enemy::Enemy;
use bevy::math::Vec3Swizzles;
use bevy::reflect::struct_partial_eq;


const TIME_STEP_PLAYER: f32 = 1.0/60.0;
const SPEED_SYRINGE: f32 = 25.0;
const DEFAULT_HEALTH: i32 = 200;
const DEFAULT_SPEED: f32 = 200.0;

#[derive(Component)]
pub struct PlayerPlugin;


#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct Syringe;

#[derive(Component)]
pub struct Velosity{
    pub(crate) x: f32, pub(crate) y: f32}


#[derive(Resource)]
pub struct DeadCount(pub i32);

#[derive(Resource)]
pub struct LimitDeads(pub i32);


#[derive(Resource)]
pub struct DeadChangeRoom(pub i32);


#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimerPlayer(Timer);


#[derive(Component)]
pub struct Health{
    pub value: i32
}

#[derive(Component)]
pub struct Damage{
    pub value: i32
}

#[derive(Component)]
pub struct Speed{
    pub value: f32
}

#[derive(Component)]
pub struct DoubleShot{
    pub value: bool
}



pub fn spawn_player(mut commands: Commands,
                    asset_server: Res<AssetServer>,
                    mut texture_atlases: ResMut<Assets<TextureAtlas>>){
    let texture_handle = asset_server.load("Doctor_Covid_final.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);


    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(2.5)),
            ..default()
        },
        AnimationTimerPlayer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    )).insert(Player)
        .insert(Health{value: DEFAULT_HEALTH})
        .insert(Velosity{x: 0.0, y:0.0})
        .insert(Speed{value: DEFAULT_SPEED})
        .insert(DoubleShot{value: false});
}


pub fn despawn_player(mut commands: Commands, query: Query< Entity, With<Player>>){
    for player in query.iter(){
        commands.entity(player).despawn();
    }
}

pub fn move_player(keyboard_input: Res<Input<KeyCode>>,
                    mut query: Query< (&mut Velosity, &mut Transform, &Speed), With<Player>>,
                    time: Res<Time>,
                    texture_atlases: Res<Assets<TextureAtlas>>,
                    mut query_animation: Query<(&mut AnimationTimerPlayer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>) {
    for (mut velocity ,mut _transform, speed) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::D) {
            velocity.x += 1.0 * TIME_STEP_PLAYER * speed.value;

            for (mut timer, mut sprite, texture_atlas_handle) in &mut query_animation {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
            }
        }

        if keyboard_input.pressed(KeyCode::A) {
            velocity.x -= 1.0 * TIME_STEP_PLAYER * speed.value;

            for (mut timer, mut sprite, texture_atlas_handle) in &mut query_animation {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
            }

        }

        if keyboard_input.pressed(KeyCode::W) {
            velocity.y += 1.0 * TIME_STEP_PLAYER * speed.value;

            for (mut timer, mut sprite, texture_atlas_handle) in &mut query_animation {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
            }

        }

        if keyboard_input.pressed(KeyCode::S) {
            velocity.y -= 1.0 * TIME_STEP_PLAYER * speed.value;


            for (mut timer, mut sprite, texture_atlas_handle) in &mut query_animation {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
            }

        }



        let mut transtalion =  &mut _transform.translation;
        transtalion.x = velocity.x;
        transtalion.y = velocity.y;

    }
}


pub fn leave_main_room(mut app_state: ResMut<State<GameState>>,
                       keyboard_input: Res<Input<KeyCode>> ){
    if keyboard_input.pressed(KeyCode::Space){
        app_state.set(GameState::Room1).expect("leaving failed");
    }
}


pub fn control_direction_syringe(keyboard_input: Res<Input<KeyCode>>,
                                 query_player: Query<(&Transform, &DoubleShot), (With<Player>, Without<Syringe>)>,
                                 mut query_enemy: Query<(Entity, &Velosity, &mut Transform), (With<Syringe>, Without<Player>)>,
                                 asset_server: Res<AssetServer>,
                                 mut commands: Commands){
    let syringe_right  = asset_server.load("Syringe_right.png");
    let syringe_left  = asset_server.load("Syringe_left.png");
    let syringe_up  = asset_server.load("Syringe_up.png");
    let syringe_down  = asset_server.load("Syringe_down.png");

    for (player_pos, double_shot) in query_player.iter(){
        if keyboard_input.just_pressed(KeyCode::Right) {
            let x = player_pos.translation.x;
            let y = player_pos.translation.y;

            if double_shot.value == true{
                commands.spawn(SpriteBundle {
                    texture: syringe_right.clone(),
                    transform: Transform{
                        translation: Vec3::new(x, y, 1.0),
                        scale: Vec3::splat(4.0),
                        ..default()
                    },
                    ..default()
                }).insert(Syringe)
                    .insert(Damage{value: 1})
                    .insert(Velosity{x: 1.0 , y: 0.5});
            }
            commands.spawn(SpriteBundle {
                texture: syringe_right.clone(),
                transform: Transform{
                    translation: Vec3::new(x, y, 1.0),
                    scale: Vec3::splat(4.0),
                    ..default()
                },
                ..default()
            }).insert(Syringe)
                .insert(Damage{value: 1})
                .insert(Velosity{x: 1.0 , y: 0.0});

        }



        if keyboard_input.just_pressed(KeyCode::Left) {
            let x = player_pos.translation.x;
            let y = player_pos.translation.y;

            if double_shot.value == true{
                commands.spawn(SpriteBundle {
                    texture: syringe_left.clone(),
                    transform: Transform{
                        translation: Vec3::new(x, y, 1.0),
                        scale: Vec3::splat(4.0),
                        ..default()
                    },
                    ..default()
                }).insert(Syringe)
                    .insert(Damage{value: 1})
                    .insert(Velosity{x: -1.0 , y: 0.5});
            }
            commands.spawn(SpriteBundle {
                texture: syringe_left.clone(),
                transform: Transform{
                    translation: Vec3::new(x, y, 1.0),
                    scale: Vec3::splat(4.0),
                    ..default()
                },
                ..default()
            }).insert(Syringe)
                .insert(Damage{value: 1})
                .insert(Velosity{x: -1.0 , y: 0.0});
        }



        if keyboard_input.just_pressed(KeyCode::Up) {
            let x = player_pos.translation.x;
            let y = player_pos.translation.y;


            if double_shot.value == true{
                commands.spawn(SpriteBundle {
                    texture: syringe_up.clone(),
                    transform: Transform{
                        translation: Vec3::new(x, y, 1.0),
                        scale: Vec3::splat(4.0),
                        ..default()
                    },
                    ..default()
                }).insert(Syringe)
                    .insert(Damage{value: 1})
                    .insert(Velosity{x: 0.5 , y: 1.0});
            }
            commands.spawn(SpriteBundle {
                texture: syringe_up.clone(),
                transform: Transform{
                    translation: Vec3::new(x, y, 1.0),
                    scale: Vec3::splat(4.0),
                    ..default()
                },
                ..default()
            }).insert(Syringe)
                .insert(Damage{value: 1})
                .insert(Velosity{x: 0.0 , y: 1.0});

        }



        if keyboard_input.just_pressed(KeyCode::Down) {
            let x = player_pos.translation.x;
            let y = player_pos.translation.y;


            if double_shot.value == true{
                commands.spawn(SpriteBundle {
                    texture: syringe_down.clone(),
                    transform: Transform{
                        translation: Vec3::new(x, y, 1.0),
                        scale: Vec3::splat(4.0),
                        ..default()
                    },
                    ..default()
                }).insert(Syringe)
                    .insert(Damage{value: 1})
                    .insert(Velosity{x: 0.5 , y: -1.0});
            }
            commands.spawn(SpriteBundle {
                texture: syringe_down.clone(),
                transform: Transform{
                    translation: Vec3::new(x, y, 1.0),
                    scale: Vec3::splat(4.0),
                    ..default()
                },
                ..default()
            }).insert(Syringe)
                .insert(Damage{value: 1})
                .insert(Velosity{x: 0.0 , y: -1.0});

        }
        for (entity ,velocity, mut transform) in query_enemy.iter_mut(){

            let mut translation = &mut transform.translation;
            translation.x += velocity.x * SPEED_SYRINGE;
            translation.y += velocity.y * SPEED_SYRINGE;

            if translation.y > WINDOW_HEIGHT / 2. + 100.0
                || translation.y < -WINDOW_HEIGHT / 2. - 100.0
                || translation.x > WINDOW_WIDTH / 2. + 100.0
                || translation.x < -WINDOW_WIDTH / 2. - 100.0{


                commands.entity(entity).despawn();

            }
        }
    }
}



pub fn syringe_hit(mut app_state: ResMut<State<GameState>>,
                   mut commands: Commands, query_syringe: Query<(Entity, &Damage, &Transform), With<Syringe>>,
                   mut query_enemy: Query<(Entity, &mut Health, &Transform), With<Enemy>>,
                   mut deadcount: ResMut<DeadCount>,
                   mut dead_change_room: ResMut<DeadChangeRoom>,
                   limit_deads: ResMut<LimitDeads>){

    for (syringe, damage ,transform_syringe) in query_syringe.iter(){
        let syringe_scale = Vec2::from(transform_syringe.scale.xy());

        for (enemy, mut health, transform_enemy) in query_enemy.iter_mut()  {
            let enemy_scale = Vec2::from(transform_enemy.scale.xy());

            let collide = collide(
                transform_syringe.translation,
                SPRITE_SYRINGE_SIZE * syringe_scale,
                transform_enemy.translation,
                SPRITE_ENEMY_SIZE * enemy_scale,
            );


            if let Some(_) = collide{
                health.value = health.value - damage.value;
                commands.entity(syringe).despawn();

                if health.value == 0{
                    deadcount.0 += 10;
                    commands.entity(enemy).despawn();

                    dead_change_room.0 += 1;


                    if dead_change_room.0 == limit_deads.0 {
                        app_state.set(GameState::Room2);
                    }
                }
            }
        }
    }
}

pub fn despawn_syringes(mut commands: Commands,
                        query: Query<Entity, With<Syringe>>){
    for syringes in query.iter(){
        commands.entity(syringes).despawn_recursive();
    }
}

impl Plugin for PlayerPlugin  {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainRoom)
            .with_system(spawn_player))
            .add_system_set(SystemSet::on_update(GameState::MainRoom)
                .with_system(leave_main_room)
                .with_system(control_direction_syringe)
                .with_system(move_player)
                .with_system(syringe_hit))
            .add_system_set(SystemSet::on_update(GameState::Room1)
                .with_system(control_direction_syringe)
                .with_system(move_player)
                .with_system(syringe_hit))
            .add_system_set(SystemSet::on_update(GameState::Room2)
                .with_system(control_direction_syringe)
                .with_system(move_player)
                .with_system(syringe_hit))
            .add_system_set(SystemSet::on_enter(GameState::GameOver)
                .with_system(despawn_player)
                .with_system(despawn_syringes));
    }
}