use bevy::{color::palettes::tailwind::*, prelude::*};
use itertools::Itertools;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::from(Srgba::hex("#1f2638")
                .expect("developer should have provided a valid hex code"))
        ))
        .insert_resource(Board::new(4))
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

#[derive(Resource, Clone)]
struct Board {
    size: u16,
    world_size: f32,
    tile_size: f32,
    tile_spacer: f32,
}

impl Board {
    fn new(size: u16) -> Self {
        let tile_size: f32 = 40.0;
        let tile_spacer: f32 = 10.0;

        let world_size = f32::from(size) * tile_size
            + f32::from(size + 1) * tile_spacer;
        Board {
            size,
            world_size,
            tile_size,
            tile_spacer,
        }
    }
    fn grid_to_world_position(&self, pos: u16) -> f32 {
        let offset =
            -self.world_size / 2.0 + 0.5 * self.tile_size;

        offset
            + f32::from(pos) * self.tile_size
            + f32::from(pos + 1) * self.tile_spacer
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_board(mut commands: Commands, board: Res<Board>) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::from(SLATE_900),
                custom_size: Some(Vec2::splat(
                    board.world_size,
                )),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            for tile in (0..board.size)
                .cartesian_product(0..board.size)
            {
                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(
                            0.54, 0.64, 0.72,
                        ),
                        custom_size: Some(Vec2::splat(
                            board.tile_size,
                        )),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board
                            .grid_to_world_position(tile.0),
                        board
                            .grid_to_world_position(tile.1),
                        1.0,
                    ),
                    ..default()
                });
            }
        });
}
