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
        .add_state(GameState::MainMenu)
        //.add_system(setup_main_menu)
        .add_system(timer_til_game_end)
        .run();

}













//DUE TO PROBLEMS EVERYTHING IS IN ONE FILE

#[derive(Resource)]
struct TimerEndGame (Timer);


fn timer_til_game_end(mut timer_end: ResMut<TimerEndGame>, mut _exit: EventWriter<AppExit>, mut commands: Commands, asset_server: Res<AssetServer>){



    let font = asset_server.load("ARCADECLASSIC.TTF");
    let text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::RED,
    };



    timer_end.0.tick(Duration::from_secs_f32(0.01));


    if timer_end.0.elapsed_secs() == 1.0{
        let mut _one = commands.spawn(Text2dBundle {
            text: Text::from_section("**********", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        });

    }

    else if timer_end.0.elapsed_secs() == 2.0{

        let mut _two = commands.spawn(Text2dBundle {
            text: Text::from_section("********************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        });
    }

    else if timer_end.0.elapsed_secs() == 3.0{

        let mut _three = commands.spawn(Text2dBundle {
            text: Text::from_section("******************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        });
    }

    else if timer_end.0.elapsed_secs() == 4.0{
        let mut _four = commands.spawn(Text2dBundle {
            text: Text::from_section("****************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        });
    }

    else if timer_end.0.elapsed_secs() == 5.0{
        let mut _five = commands.spawn(Text2dBundle {
            text: Text::from_section("**************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        });

    }


    else if timer_end.0.elapsed_secs() == 6.0{
        let mut _6 = commands.spawn(Text2dBundle {
            text: Text::from_section("************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        });

    }

    else if timer_end.0.elapsed_secs() == 7.0{
        let mut _seven = commands.spawn(Text2dBundle {
            text: Text::from_section("**********************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        });

    }

    else if timer_end.0.elapsed_secs() == 8.0{
        let mut  _eight = commands.spawn(Text2dBundle {
            text: Text::from_section("********************************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        });

    }

    else if timer_end.0.elapsed_secs() == 9.0{
        let mut _nine = commands.spawn(Text2dBundle {
            text: Text::from_section("******************************************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        });

    }

    else if timer_end.0.elapsed_secs() == 10.0{
        let mut _ten = commands.spawn(Text2dBundle {
            text: Text::from_section("***************************************************************************************************************Game Over", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        });
        _exit.send(AppExit);
    }


}





//Menu



#[derive(Component)]
struct MainMenu;
struct MainGame;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
enum GameState {
    MainGame,
    MainMenu
}


#[derive(Component, Copy, Clone)]
pub enum MenuItem {
    Start,
    Quit
}

pub fn setup_main_menu(mut commands: Commands, asset_server: ResMut<AssetServer> ) {
    let font = asset_server.load("ARCADECLASSIC.TTF");

    let menu_tytle_style = TextStyle {
        font,
        font_size: 40.0,
        color: Color::WHITE
    };


    commands.spawn(NodeBundle {
        style: Style{
            direction: Direction::RightToLeft,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::SpaceEvenly,
            size: Size{
                width: Val::Percent(100.0),
                height: Val::Percent(50.0),
            },
            ..Style::default()
        },
        background_color: BackgroundColor(Color::CRIMSON),
        visibility: Visibility{ is_visible: false },
        ..NodeBundle::default()
    })
        .insert(MainMenu)
        .with_children(|mut parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section("Dr. Covid", menu_tytle_style.clone()),
                ..Text2dBundle::default()
            });
        });
}


fn spawn_button(parent: &mut ChildBuilder, font: Handle<Font>, menu_item: MenuItem) {
    let button_style = TextStyle{
        font,
        font_size: 20.0,
        color: Color::WHITE
    } ;


    parent.spawn(ButtonBundle {
        style: Style{
            align_items: Default::default(),
            align_self: Default::default(),
            align_content: Default::default(),
            justify_content: Default::default(),
            size: Default::default(),
            ..Style::default()
        },
        ..ButtonBundle::default()
    })
        .insert(menu_item)
        .with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    match menu_item {
                        MenuItem::Start => "Start",
                        MenuItem::Quit => "Quit"


                    },button_style.clone()

                ),
                ..Text2dBundle::default()
            });
        });
}


