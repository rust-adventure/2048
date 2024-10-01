use bevy::{color::palettes::tailwind::*, prelude::*};
use itertools::Itertools;

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
    let tile_spacer = 10.;
    let board_world_size = f32::from(board.size)
        * tile_size
        + f32::from(board.size + 1) * tile_spacer;

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::from(SLATE_900),
                custom_size: Some(Vec2::splat(
                    board_world_size,
                )),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            let offset =
                -board_world_size / 2.0 + tile_size / 2.0;

            for tile in (0..board.size)
                .cartesian_product(0..board.size)
            {
                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(
                            0.54, 0.64, 0.72,
                        ),
                        custom_size: Some(Vec2::splat(
                            tile_size,
                        )),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        offset
                            + f32::from(tile.0) * tile_size
                            + f32::from(tile.0 + 1)
                                * tile_spacer,
                        offset
                            + f32::from(tile.1) * tile_size
                            + f32::from(tile.1 + 1)
                                * tile_spacer,
                        1.0,
                    ),
                    ..default()
                });
            }
        });
}
