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
        .add_systems(Startup, (setup_camera, setup_ui))
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

// Spawns the camera that draws UI
fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

/// Marker component for the text that displays the current status
#[derive(Component)]
struct WIPText;

// Spawns the UI
fn setup_ui(mut cmd: Commands) {
    cmd.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            ..default()
        },
        ..default()
    })
    .with_children(|root| {
        root.spawn((
            TextBundle::from_section(
                "Road Road (WIP)",
                TextStyle {
                    font_size: 24.0,
                    color: Color::BLACK,
                    ..default()
                },
            ),
            WIPText,
        ));
    });
}
