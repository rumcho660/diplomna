use std::time::Duration;
use bevy:: prelude::*;
use bevy::app::AppExit;
use crate::GameState;

#[derive(Resource)]
pub struct TimerEndGame (pub Timer);

#[derive(Component)]
pub struct TimerItem;

pub fn timer_til_game_end(mut timer_end: ResMut<TimerEndGame>, mut _exit: EventWriter<AppExit>, mut commands: Commands, asset_server: Res<AssetServer>, mut app_state: ResMut<State<GameState>>){



    let font1 = asset_server.load("ARCADECLASSIC.TTF");
    let font2 = asset_server.load("FFFFORWA.TTF");

    let text_style = TextStyle {
        font: font1,
        font_size: 20.0,
        color: Color::RED,
    };

    let text_style_over = TextStyle {
        font: font2,
        font_size: 60.0,
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
        app_state.push(GameState::GameOver).expect("Problem with pushing GameOver state");
        commands.spawn(Text2dBundle {
            text: Text::from_section("Time ran out\n
            Try agan by restarting the game", text_style_over.clone()),
            transform: Transform::from_xyz(-550.0, 100.0, 0.0),
            ..default()
        });



    }


}

pub fn destroy_timer_el(mut commands: Commands, query: Query<Entity, With<TimerItem>>){
    for timer in query.iter() {
        commands.entity(timer).despawn_recursive();
    }
}