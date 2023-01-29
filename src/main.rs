use std::time::Duration;
use bevy:: prelude::*;
use bevy::app::AppExit;
use bevy::text::Text2dBundle;
use bevy::window::close_on_esc;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(TimerEndGame(Timer::from_seconds(11.0, TimerMode::Once)))
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor{
                title: "Dr. Covid".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_state(GameState::MainGame)
        .add_startup_system(setup)
        .add_system(close_on_esc)
        .add_system_set(
            SystemSet::on_update(GameState::MainGame)
                .with_system(timer_til_game_end)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver)
                .with_system(destroy_timer_el)
        )
        .run();

}













//DUE TO PROBLEMS EVERYTHING IS IN ONE FILE

#[derive(Resource)]
struct TimerEndGame (Timer);

#[derive(Component)]
struct TimerItem;

fn timer_til_game_end(mut timer_end: ResMut<TimerEndGame>, mut _exit: EventWriter<AppExit>, mut commands: Commands, asset_server: Res<AssetServer>, mut app_state: ResMut<State<GameState>>){



    let font1 = asset_server.load("ARCADECLASSIC.TTF");
    let font2 = asset_server.load("FFFFORWA.TTF");

    let text_style = TextStyle {
        font: font1,
        font_size: 20.0,
        color: Color::RED,
    };

    let text_style_over = TextStyle {
        font: font2,
        font_size: 90.0,
        color: Color::BLACK,
    };



    timer_end.0.tick(Duration::from_secs_f32(0.02));



    if timer_end.0.elapsed_secs() == 1.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("**********", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }

    else if timer_end.0.elapsed_secs() == 2.0{

        commands.spawn(Text2dBundle {
            text: Text::from_section("********************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);
    }

    else if timer_end.0.elapsed_secs() == 3.0{

        commands.spawn(Text2dBundle {
            text: Text::from_section("******************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);
    }

    else if timer_end.0.elapsed_secs() == 4.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("****************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);
    }

    else if timer_end.0.elapsed_secs() == 5.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("**************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }


    else if timer_end.0.elapsed_secs() == 6.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }

    else if timer_end.0.elapsed_secs() == 7.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("**********************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }

    else if timer_end.0.elapsed_secs() == 8.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("********************************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }

    else if timer_end.0.elapsed_secs() == 9.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("******************************************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);

    }

    else if timer_end.0.elapsed_secs() == 10.0{
        commands.spawn(Text2dBundle {
            text: Text::from_section("*************************************************************************************************************************", text_style.clone()),
            transform: Transform::from_xyz(-620.0, 350.0, 0.0),
            ..default()
        }).insert(TimerItem);




    }
    else if timer_end.0.elapsed_secs() == 11.0{
        app_state.push(GameState::GameOver);
        commands.spawn(Text2dBundle {
            text: Text::from_section("Time ran out\n
            Try agan by restarting the game", text_style_over.clone()),
            transform: Transform::from_xyz(-200.0, 0.0, 0.0),
            ..default()
        });



    }


}

fn destroy_timer_el(mut commands: Commands, query: Query<Entity, With<TimerItem>>){
    for timer in query.iter() {
        commands.entity(timer).despawn_recursive();
    }
}







//Menu
#[derive(Component)]
pub struct MainMenu;
pub struct MainGame;
pub struct GameOver;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum GameState {
    MainGame,
    MainMenu,
    GameOver
}

#[derive(Component, Copy, Clone)]
pub enum MenuItem {
    Start,
    Quit
}

pub struct MenusPlugin;

pub fn setup_main_menu(mut commands: Commands, asset_server: ResMut<AssetServer> ) {
    let font = asset_server.load("ARCADECLASSIC.TTF");

    let menu_textstyle = TextStyle {
        font: font.clone(),
        font_size: 70.0,
        color: Color::WHITE,
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
                text: Text::from_section("Dr. Covid", menu_textstyle.clone()),
                ..Text2dBundle::default()
            });

            spawn_button(&mut parent, font.clone(), MenuItem::Start);
            spawn_button(&mut parent, font.clone(), MenuItem::Quit);
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
            align_items: AlignItems::Center,
            align_self: AlignSelf::Stretch,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::SpaceEvenly,
            size: Size{
                width: Val::Percent(30.0),
                height: Val::Percent(30.0),
            },
            ..Style::default()
        },
        ..ButtonBundle::default()
    })
        .insert(menu_item)
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
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


pub fn handle_menu_item_interactions(
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<State<GameState>>,
    query: Query<(&Interaction, &MenuItem)>,
) {
    query.for_each(|(interaction, item)| match interaction {
        // Define what should happen when the buttons are clicked.
        Interaction::Clicked => match item {
            // When the play button is clicked, we push the `MainGame` state to
            // start the game.
            MenuItem::Start => {
                app_state
                    .push(GameState::MainGame);

            }
            MenuItem::Quit => app_exit_events.send(AppExit),
        },
        Interaction::Hovered => {}
        _ => {}
    });
}



pub fn despawn_menu_items(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}


impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {

        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(setup_main_menu),
        )

            .add_system_set(
                SystemSet::on_resume(GameState::MainMenu)
                    .with_system(setup_main_menu)
            )

            .add_system_set(
                SystemSet::on_pause(GameState::MainMenu)
                    .with_system(despawn_menu_items)
            )

            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu)
                    .with_system(main)
            )

            .add_system_set(
                SystemSet::on_resume(GameState::MainMenu)
                    .with_system(handle_menu_item_interactions)
            );
    }
}
