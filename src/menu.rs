use bevy::app::AppExit;
use bevy::prelude::*;
use crate::main;

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


pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(NodeBundle{
        style: Style{
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: BackgroundColor::from(Color::BLACK),
        ..default()
    }).insert(MainMenu)
        .with_children(|parent|{
            commands.spawn(TextBundle{
                style: Default::default(),
                text: Default::default(),
                calculated_size: Default::default(),
                focus_policy: Default::default(),
                transform: Default::default(),
                global_transform: Default::default(),
                visibility: Default::default(),
                computed_visibility: Default::default(),
                z_index: Default::default(),
                ..default()
            });
        });
}

/*pub fn setup_main_menu(mut commands: Commands, asset_server: ResMut<AssetServer> ) {
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
}*/