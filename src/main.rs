use bevy::{prelude::*, window::Cursor};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Road Race".into(),
                resizable: false,
                cursor: Cursor {
                    visible: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .run();
}
