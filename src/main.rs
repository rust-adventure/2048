use bevy::{color::palettes::tailwind::*, prelude::*};

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::from(Srgba::hex("#1f2638")
                .expect("developer should have provided a valid hex code"))
        ))
        .insert_resource(Board { size: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2048".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, spawn_board))
        .run();
}

#[derive(Resource)]
struct Board {
    size: u16,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_board(mut commands: Commands, board: Res<Board>) {
    let tile_size = 40.;
    let board_world_size =
        f32::from(board.size) * tile_size;

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::from(SLATE_900),
            custom_size: Some(Vec2::splat(
                board_world_size,
            )),
            ..default()
        },
        ..default()
    });
}
