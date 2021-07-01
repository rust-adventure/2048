use bevy::prelude::*;
use itertools::Itertools;

const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

struct Board {
    size: u8,
}

struct Materials {
    board: Handle<ColorMaterial>,
    tile_placeholder: Handle<ColorMaterial>,
}

impl FromWorld for Materials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .unwrap();
        Materials {
            board: materials
                .add(Color::rgb(0.7, 0.7, 0.8).into()),
            tile_placeholder: materials
                .add(Color::rgb(0.75, 0.75, 0.9).into()),
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .init_resource::<Materials>()
        .add_startup_system(setup.system())
        .add_startup_system(spawn_board.system())
        .run()
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_board(
    mut commands: Commands,
    materials: Res<Materials>,
) {
    let board = Board { size: 4 };
    let physical_board_size = f32::from(board.size)
        * TILE_SIZE
        + f32::from(board.size + 1) * TILE_SPACER;

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.board.clone(),
            sprite: Sprite::new(Vec2::new(
                physical_board_size,
                physical_board_size,
            )),
            ..Default::default()
        })
        .with_children(|builder| {
            let offset = -physical_board_size / 2.0
                + 0.5 * TILE_SIZE;

            for tile in (0..board.size)
                .cartesian_product(0..board.size)
            {
                builder.spawn_bundle(SpriteBundle {
                    material: materials
                        .tile_placeholder
                        .clone(),
                    sprite: Sprite::new(Vec2::new(
                        TILE_SIZE, TILE_SIZE,
                    )),
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
                    ..Default::default()
                });
            }
        })
        .insert(board);
}
