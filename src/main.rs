use bevy::{app::AppExit, prelude::*, window::Cursor};

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
        // See: https://bevy-cheatbook.github.io/programming/app-builder.html#quitting-the-app
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, handle_exit)
        .run();
}

fn handle_exit(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Q) {
        exit.send(AppExit);
    }
}
