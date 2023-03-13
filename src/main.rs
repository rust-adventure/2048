use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;
use std::{cmp::Ordering, convert::TryFrom};

mod colors;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("#1f2638").unwrap(),
        ))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2048".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<FontSpec>()
        .add_startup_systems(
            (
                setup,
                spawn_board,
                apply_system_buffers,
                spawn_tiles,
            )
                .chain(),
        )
        .add_systems((render_tile_points, board_shift))
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

#[derive(Debug, Component)]
struct Points {
    value: u32,
}

#[derive(Debug, Component)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Component)]
struct TileText;

#[derive(Resource)]
struct FontSpec {
    family: Handle<Font>,
}

impl FromWorld for FontSpec {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource_mut::<AssetServer>()
            .unwrap();
        FontSpec {
            family: asset_server
                .load("fonts/FiraSans-Bold.ttf"),
        }
    }
}

#[derive(Component)]
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

fn spawn_board(mut commands: Commands) {
    let board = Board::new(4);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: colors::BOARD,
                custom_size: Some(Vec2::new(
                    board.physical_size,
                    board.physical_size,
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
                        color: colors::TILE_PLACEHOLDER,
                        custom_size: Some(Vec2::new(
                            TILE_SIZE, TILE_SIZE,
                        )),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(
                            tile.0,
                        ),
                        board.cell_position_to_physical(
                            tile.1,
                        ),
                        1.0,
                    ),
                    ..default()
                });
            }
        })
        .insert(board);
}

fn spawn_tiles(
    mut commands: Commands,
    query_board: Query<&Board>,
    font_spec: Res<FontSpec>,
) {
    let board = query_board.single();

    let mut rng = rand::thread_rng();
    let starting_tiles: Vec<(u8, u8)> = (0..board.size)
        .cartesian_product(0..board.size)
        .choose_multiple(&mut rng, 2);

    for (x, y) in starting_tiles.iter() {
        let pos = Position { x: *x, y: *y };
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: colors::TILE,
                    custom_size: Some(Vec2::new(
                        TILE_SIZE, TILE_SIZE,
                    )),
                    ..default()
                },
                transform: Transform::from_xyz(
                    board.cell_position_to_physical(pos.x),
                    board.cell_position_to_physical(pos.y),
                    1.0,
                ),
                ..default()
            })
            .with_children(|child_builder| {
                child_builder
                    .spawn(Text2dBundle {
                        text: Text::from_section(
                            "4",
                            TextStyle {
                                font: font_spec
                                    .family
                                    .clone(),
                                font_size: 40.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_alignment(
                            TextAlignment::Center,
                        ),
                        transform: Transform::from_xyz(
                            0.0, 0.0, 1.0,
                        ),
                        ..default()
                    })
                    .insert(TileText);
            })
            .insert(Points { value: 2 })
            .insert(pos);
    }
}

fn render_tile_points(
    mut texts: Query<&mut Text, With<TileText>>,
    tiles: Query<(&Points, &Children)>,
) {
    for (points, children) in tiles.iter() {
        if let Some(entity) = children.first() {
            let mut text = texts
                .get_mut(*entity)
                .expect("expected Text to exist");
            let mut text_section = text.sections.first_mut().expect("expect first section to be accessible as mutable");
            text_section.value = points.value.to_string()
        }
    }
}

enum BoardShift {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<&KeyCode> for BoardShift {
    type Error = &'static str;

    fn try_from(
        value: &KeyCode,
    ) -> Result<Self, Self::Error> {
        match value {
            KeyCode::Left => Ok(BoardShift::Left),
            KeyCode::Up => Ok(BoardShift::Up),
            KeyCode::Right => Ok(BoardShift::Right),
            KeyCode::Down => Ok(BoardShift::Down),
            _ => Err("not a valid board_shift key"),
        }
    }
}

fn board_shift(
    input: Res<Input<KeyCode>>,
    mut tiles: Query<(Entity, &mut Position, &mut Points)>,
) {
    let shift_direction =
        input.get_just_pressed().find_map(|key_code| {
            BoardShift::try_from(key_code).ok()
        });

    match shift_direction {
        Some(BoardShift::Left) => {
            dbg!("left");
            let mut it =
                tiles.iter_mut().sorted_by(|a, b| {
                    match Ord::cmp(&a.1.y, &b.1.y) {
                        Ordering::Equal => {
                            Ord::cmp(&a.1.x, &b.1.x)
                        }
                        ordering => ordering,
                    }
                });
        }
        Some(BoardShift::Right) => {
            dbg!("right");
        }
        Some(BoardShift::Up) => {
            dbg!("up");
        }
        Some(BoardShift::Down) => {
            dbg!("down");
        }
        None => (),
    }
}
