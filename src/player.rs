use bevy::prelude::*;

#[derive(Component)]
struct Player;



pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas: ResMut<Assets<TextureAtlas>>){
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
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        });
}