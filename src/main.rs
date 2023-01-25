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

    let font = asset_server.load("Yesillow.otf");
    let text_style = TextStyle {
        font,
        font_size: 100.0,
        color: Color::RED,
    };



    timer_end.0.tick(Duration::from_secs_f32(0.01));


    if timer_end.0.elapsed_secs() == 1.0{
        let onesec = commands.spawn(Text2dBundle {
            text: Text::from_section("1 sec", text_style.clone()),
            ..default()
        });

    }

    else if timer_end.0.elapsed_secs() == 2.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("1 sec 2 sec", text_style.clone()),
            ..default()
        });

    }

    else if timer_end.0.elapsed_secs() == 3.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("1 sec 2 sec 3 sec", text_style.clone()),
            ..default()
        });

    }

    else if timer_end.0.elapsed_secs() == 4.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("1 sec 2 sec 3 sec 4 sec", text_style.clone()),
            ..default()
        });

    }

    else if timer_end.0.elapsed_secs() == 5.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("1 sec 2 sec 3 sec 4 sec 5 sec", text_style.clone()),
            ..default()
        });

    }


    else if timer_end.0.elapsed_secs() == 6.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("1 sec 2 sec 3 sec 4 sec 5 sec 6 sec", text_style.clone()),
            ..default()
        });

    }

    else if timer_end.0.elapsed_secs() == 7.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("1 sec 2 sec 3 sec 4 sec 5 sec 6 sec 7 sec", text_style.clone()),
            ..default()
        });

    }

    else if timer_end.0.elapsed_secs() == 8.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("1 sec 2 sec 3 sec 4 sec 5 sec 6 sec 7 sec 8 sec", text_style.clone()),
            ..default()
        });

    }

    else if timer_end.0.elapsed_secs() == 9.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section(" 1 sec 2 sec 3 sec 4 sec 5 sec 6 sec 7 sec 8 sec 9 sec", text_style.clone()),
            ..default()
        });

    }

    else if timer_end.0.elapsed_secs() == 10.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("1 sec 2 sec 3 sec 4 sec 5 sec 6 sec 7 sec 8 sec 9 sec 10 sec", text_style.clone()),
            ..default()
        });
        _exit.send(AppExit);
    }


}

