use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;

const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

struct Board {
    size: u8,
    physical_size: f32,
}

impl Board {
    fn new(size: u8) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE
            + f32::from(size + 1) * TILE_SPACER;
        Board {
            size,
            physical_size,
        }
    }
    fn cell_position_to_physical(&self, pos: u8) -> f32 {
        let offset =
            -self.physical_size / 2.0 + 0.5 * TILE_SIZE;

        offset
            + f32::from(pos) * TILE_SIZE
            + f32::from(pos + 1) * TILE_SPACER
    }
}

struct Points {
    value: u32,
}
struct Position {
    x: u8,
    y: u8,
}

pub struct TileText;

struct Materials {
    board: Handle<ColorMaterial>,
    tile_placeholder: Handle<ColorMaterial>,
    tile: Handle<ColorMaterial>,
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
            tile: materials
                .add(Color::rgb(0.9, 0.9, 1.0).into()),
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .init_resource::<Materials>()
        .add_startup_system(setup.system())
        .add_startup_system(spawn_board.system())
        .add_startup_system_to_stage(
            StartupStage::PostStartup,
            spawn_tiles.system(),
        )
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
    let board = Board::new(4);

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.board.clone(),
            sprite: Sprite::new(Vec2::new(
                board.physical_size,
                board.physical_size,
            )),
            ..Default::default()
        })
        .with_children(|builder| {
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
                        board.cell_position_to_physical(
                            tile.0,
                        ),
                        board.cell_position_to_physical(
                            tile.1,
                        ),
                        1.0,
                    ),
                    ..Default::default()
                });
            }
        })
        .insert(board);
}

fn spawn_tiles(
    mut commands: Commands,
    materials: Res<Materials>,
    query_board: Query<&Board>,
) {
    let board = query_board
        .single()
        .expect("always expect a board");

    let mut rng = rand::thread_rng();
    let starting_tiles: Vec<(u8, u8)> = (0..board.size)
        .cartesian_product(0..board.size)
        .choose_multiple(&mut rng, 2);
    for (x, y) in starting_tiles.iter() {
        let pos = Position { x: *x, y: *y };
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.tile.clone(),
                sprite: Sprite::new(Vec2::new(
                    TILE_SIZE, TILE_SIZE,
                )),
                transform: Transform::from_xyz(
                    board.cell_position_to_physical(pos.x),
                    board.cell_position_to_physical(pos.y),
                    1.0,
                ),
                ..Default::default()
            })
            .with_children(|child_builder| {
                child_builder
                    .spawn_bundle(Text2dBundle {
                        text: Text::with_section(
                            "2",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::BLACK,
                                ..Default::default()
                            },
                            TextAlignment {
                                vertical:
                                    VerticalAlign::Center,
                                horizontal:
                                    HorizontalAlign::Center,
                            },
                        ),
                        transform: Transform::from_xyz(
                            0.0, 0.0, 1.0,
                        ),
                        ..Default::default()
                    })
                    .insert(TileText);
            })
            .insert(Points { value: 2 })
            .insert(pos);
    }
}
