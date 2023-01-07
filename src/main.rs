use bevy::window::{CompositeAlphaMode, CursorGrabMode, PresentMode};
use bevy::prelude::*;


fn main() {
    App::new()
        .insert_resource(WindowDescriptor{
            title: "Dr. Covid".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();

}

