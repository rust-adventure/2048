use bevy::prelude::*;
use itertools::Itertools;

mod colors;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("#1f2638")
              .expect("developer should have provided a valid hex code")
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
        .run()
}

const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

#[derive(Resource)]
struct Board {
    size: u8,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_board(mut commands: Commands, board: Res<Board>) {
    let physical_board_size = f32::from(board.size)
        * TILE_SIZE
        + f32::from(board.size + 1) * TILE_SPACER;

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: colors::BOARD,
                custom_size: Some(Vec2::splat(
                    physical_board_size,
                )),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            for tile in (0..board.size)
                .cartesian_product(0..board.size)
            {
                let offset = -physical_board_size / 2.0
                    + TILE_SIZE / 2.0;

                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: colors::TILE_PLACEHOLDER,
                        custom_size: Some(Vec2::splat(
                            TILE_SIZE,
                        )),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        offset
                            + f32::from(tile.0) * TILE_SIZE
                            + f32::from(tile.0 + 1)
                                * TILE_SPACER,
                        offset
                            + f32::from(tile.1) * TILE_SIZE
                            + f32::from(tile.1 + 1)
                                * TILE_SPACER,
                        1.0,
                    ),
                    ..default()
                });
            }
        });
}
