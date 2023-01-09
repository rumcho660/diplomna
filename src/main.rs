use bevy:: prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor{
                title: "Dr. Covid".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}
