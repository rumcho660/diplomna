use bevy::window::{CompositeAlphaMode, CursorGrabMode, PresentMode};
use bevy::prelude::*;
use bevy::window::CursorIcon::Default;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor{
                title: "Dr. Covid".to_string(),
                ..default()
            },
            ..default()
        }))
        .run();

}

