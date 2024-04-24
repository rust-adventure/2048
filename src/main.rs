use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(Board { size: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2048".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, spawn_board))
        .run()
}

const TILE_SIZE: f32 = 40.0;

#[derive(Resource)]
struct Board {
    size: u8,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_board(mut commands: Commands, board: Res<Board>) {
    let physical_board_size =
        f32::from(board.size) * TILE_SIZE;

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(
                physical_board_size,
            )),
            ..default()
        },
        ..default()
    });
}
