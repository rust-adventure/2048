use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2048".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
