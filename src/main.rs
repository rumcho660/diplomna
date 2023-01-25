use std::iter::Once;
use std::time::Duration;
use bevy:: prelude::*;
use bevy::app::AppExit;


fn main() {
    App::new()
        .insert_resource(TimerOfGame(Timer::from_seconds(15.0, TimerMode::Once)))
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor{
                title: "Dr. Covid".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_system(timer1)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource)]
struct TimerOfGame (Timer);


fn timer1(time: Res<Time>, mut timer: ResMut<TimerOfGame> ,mut exit: EventWriter<AppExit>){
    if timer.0.tick(time.delta()).just_finished(){
        println!("ohh");
        exit.send(AppExit);
    }
}