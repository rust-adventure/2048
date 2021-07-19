use std::{cmp::Ordering, convert::TryFrom};

use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;

mod ui;
use ui::*;

const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

struct NewTileEvent;

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

#[derive(Debug)]
struct Points {
    value: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Position {
    x: u8,
    y: u8,
}

pub struct TileText;

struct Materials {
    board: Handle<ColorMaterial>,
    tile_placeholder: Handle<ColorMaterial>,
    tile: Handle<ColorMaterial>,
    none: Handle<ColorMaterial>,
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
            none: materials.add(Color::NONE.into()),
        }
    }
}

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

enum BoardShift {
    Left,
    Right,
    Up,
    Down,
}
impl BoardShift {
    fn sort(&self, a: &Position, b: &Position) -> Ordering {
        match self {
            BoardShift::Left => {
                match Ord::cmp(&a.y, &b.y) {
                    Ordering::Equal => Ord::cmp(&a.x, &b.x),
                    ordering => ordering,
                }
            }
            BoardShift::Right => {
                match Ord::cmp(&b.y, &a.y) {
                    std::cmp::Ordering::Equal => {
                        Ord::cmp(&b.x, &a.x)
                    }
                    a => a,
                }
            }
            BoardShift::Up => match Ord::cmp(&b.x, &a.x) {
                std::cmp::Ordering::Equal => {
                    Ord::cmp(&b.y, &a.y)
                }
                ordering => ordering,
            },
            BoardShift::Down => {
                match Ord::cmp(&a.x, &b.x) {
                    std::cmp::Ordering::Equal => {
                        Ord::cmp(&a.y, &b.y)
                    }
                    ordering => ordering,
                }
            }
        }
    }
    fn set_column_position(
        &self,
        board_size: u8,
        position: &mut Mut<Position>,
        index: u8,
    ) {
        match self {
            BoardShift::Left => {
                position.x = index;
            }
            BoardShift::Right => {
                position.x = board_size - 1 - index
            }
            BoardShift::Up => {
                position.y = board_size - 1 - index
            }
            BoardShift::Down => {
                position.y = index;
            }
        }
    }
    fn get_row_position(&self, position: &Position) -> u8 {
        match self {
            BoardShift::Left | BoardShift::Right => {
                position.y
            }
            BoardShift::Up | BoardShift::Down => position.x,
        }
    }
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

#[derive(Default)]
struct Game {
    score: u32,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .init_resource::<Materials>()
        .init_resource::<FontSpec>()
        .init_resource::<Game>()
        .add_plugin(GameUiPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(spawn_board.system())
        .add_startup_system_to_stage(
            StartupStage::PostStartup,
            spawn_tiles.system(),
        )
        .add_system(render_tile_points.system())
        .add_system(board_shift.system())
        .add_system(render_tiles.system())
        .add_system(new_tile_handler.system())
        .add_event::<NewTileEvent>()
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
    font_spec: Res<FontSpec>,
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
        spawn_tile(
            &mut commands,
            &materials,
            board,
            &font_spec,
            pos,
        );
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

fn board_shift(
    query_board: Query<&Board>,
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut tiles: Query<(Entity, &mut Position, &mut Points)>,
    mut tile_writer: EventWriter<NewTileEvent>,
    mut game: ResMut<Game>,
) {
    let board = query_board
        .single()
        .expect("expect there to be a board");

    let shift_direction =
        keyboard_input.get_just_pressed().find_map(
            |key_code| BoardShift::try_from(key_code).ok(),
        );

    if let Some(board_shift) = shift_direction {
        let mut it = tiles
            .iter_mut()
            .sorted_by(|a, b| board_shift.sort(&a.1, &b.1))
            .peekable();
        let mut column: u8 = 0;

        while let Some(mut tile) = it.next() {
            board_shift.set_column_position(
                board.size,
                &mut tile.1,
                column,
            );

            if let Some(tile_next) = it.peek() {
                if board_shift.get_row_position(&tile.1)
                    != board_shift
                        .get_row_position(&tile_next.1)
                {
                    // different rows, don't merge
                    column = 0;
                } else if tile.2.value != tile_next.2.value
                {
                    // different values, don't merge
                    column = column + 1;
                } else {
                    // merge
                    // despawn the next tile, and
                    // merge it with the current
                    // tile.
                    let real_next_tile = it.next()
                                    .expect("A peeked tile should always exist when we .next here");
                    tile.2.value = tile.2.value
                        + real_next_tile.2.value;

                    game.score += tile.2.value;

                    commands
                        .entity(real_next_tile.0)
                        .despawn_recursive();

                    // if the next, next tile
                    // (tile #3 of 3)
                    // isn't in the same row, reset
                    // x
                    // otherwise increment by one
                    if let Some(future) = it.peek() {
                        if board_shift
                            .get_row_position(&tile.1)
                            != board_shift
                                .get_row_position(&future.1)
                        {
                            column = 0;
                        } else {
                            column = column + 1;
                        }
                    }
                }
            }
        }
        tile_writer.send(NewTileEvent);
    }
}

fn render_tiles(
    mut tiles: Query<(
        &mut Transform,
        &Position,
        Changed<Position>,
    )>,
    query_board: Query<&Board>,
) {
    let board = query_board
        .single()
        .expect("expect there to be a board");
    for (mut transform, pos, pos_changed) in
        tiles.iter_mut()
    {
        if pos_changed {
            transform.translation.x =
                board.cell_position_to_physical(pos.x);
            transform.translation.y =
                board.cell_position_to_physical(pos.y);
        }
    }
}

fn new_tile_handler(
    mut tile_reader: EventReader<NewTileEvent>,
    mut commands: Commands,
    query_board: Query<&Board>,
    materials: Res<Materials>,
    tiles: Query<&Position>,
    font_spec: Res<FontSpec>,
) {
    let board = query_board
        .single()
        .expect("expect there to always be a board");

    for _event in tile_reader.iter() {
        // insert new tile
        let mut rng = rand::thread_rng();
        let possible_position: Option<Position> = (0
            ..board.size)
            .cartesian_product(0..board.size)
            .filter_map(|tile_pos| {
                let new_pos = Position {
                    x: tile_pos.0,
                    y: tile_pos.1,
                };
                match tiles
                    .iter()
                    .find(|&&pos| pos == new_pos)
                {
                    Some(_) => None,
                    None => Some(new_pos),
                }
            })
            .choose(&mut rng);

        if let Some(pos) = possible_position {
            spawn_tile(
                &mut commands,
                &materials,
                board,
                &font_spec,
                pos,
            );
        }
    }
}

fn spawn_tile(
    commands: &mut Commands,
    materials: &Res<Materials>,
    board: &Board,
    font_spec: &Res<FontSpec>,
    pos: Position,
) {
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
                            font: font_spec.family.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                            ..Default::default()
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
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
