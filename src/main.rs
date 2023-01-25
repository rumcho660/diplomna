use std::time::Duration;
use bevy:: prelude::*;
use bevy::app::AppExit;
use bevy::text::Text2dBundle;



fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(TimerEndGame(Timer::from_seconds(10.0, TimerMode::Once)))
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor{
                title: "Dr. Covid".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(timer_til_game_end)
        .run();
}


//in one file due to problem

#[derive(Resource)]
struct TimerEndGame (Timer);


fn timer_til_game_end(mut timer_end: ResMut<TimerEndGame>, mut _exit: EventWriter<AppExit>, mut commands: Commands, asset_server: Res<AssetServer>){
    let font = asset_server.load("fonts/SuPostcode-VGeLe.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };

    timer_end.0.tick(Duration::from_secs_f32(0.01));


    if timer_end.0.elapsed_secs() == 0.0 {
        commands.spawn(Text2dBundle {
            text: Text::from_section("0", text_style.clone()),
            ..default()
        });
    }


    else if timer_end.0.elapsed_secs() == 1.0{
        println!("1")
    }

    else if timer_end.0.elapsed_secs() == 2.0{
        println!("2");

    }

    else if timer_end.0.elapsed_secs() == 3.0{
        println!("3");


    }

    else if timer_end.0.elapsed_secs() == 4.0{
        println!("4");

    }

    else if timer_end.0.elapsed_secs() == 5.0{
        println!("5");

    }


    else if timer_end.0.elapsed_secs() == 6.0{
        println!("6");

    }

    else if timer_end.0.elapsed_secs() == 7.0{
        println!("7");

    }

    else if timer_end.0.elapsed_secs() == 8.0{
        println!("8");

    }

    else if timer_end.0.elapsed_secs() == 9.0{
        println!("9");

    }

    else if timer_end.0.elapsed_secs() == 10.0{
        println!("10");
        _exit.send(AppExit);
    }


}

