use std::{
    cmp::Ordering,
    collections::HashMap,
    convert::{TryFrom, TryInto},
    ops::Range,
};

use bevy::prelude::*;
use bevy_easings::*;
use itertools::Itertools;
use rand::prelude::*;

mod ui;
use ui::*;
mod colors;

const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

#[derive(Event)]
struct NewTileEvent;

#[derive(Resource)]
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

    fn tiles(&self) -> impl Iterator<Item = (u8, u8)> {
        (0..self.size).cartesian_product(0..self.size)
    }
}

#[derive(Debug, PartialEq, Component)]
struct Points {
    value: u32,
}

#[derive(
    Debug, PartialEq, Copy, Clone, Eq, Hash, Component,
)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Component)]
pub struct TileText;

#[derive(Debug)]
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
                a.y.cmp(&b.y).then(a.x.cmp(&b.x))
            }
            BoardShift::Right => {
                b.y.cmp(&a.y).then(b.x.cmp(&a.x))
            }
            BoardShift::Up => {
                b.x.cmp(&a.x).then(b.y.cmp(&a.y))
            }
            BoardShift::Down => {
                a.x.cmp(&b.x).then(a.y.cmp(&b.y))
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
            KeyCode::ArrowLeft => Ok(BoardShift::Left),
            KeyCode::ArrowUp => Ok(BoardShift::Up),
            KeyCode::ArrowRight => Ok(BoardShift::Right),
            KeyCode::ArrowDown => Ok(BoardShift::Down),
            _ => Err("not a valid board_shift key"),
        }
    }
}

#[derive(Default, Resource)]
struct Game {
    score: u32,
    score_best: u32,
}

#[derive(
    Default, Debug, Clone, Eq, PartialEq, Hash, States,
)]
enum RunState {
    #[default]
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("#1f2638").unwrap(),
        ))
        .insert_resource(Board::new(4))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "2048".into(),
                    ..default()
                }),
                ..default()
            }),
            EasingsPlugin,
            GameUiPlugin,
        ))
        .init_resource::<Game>()
        .init_state::<RunState>()
        .add_systems(Startup, (setup, spawn_board).chain())
        .add_systems(
            Update,
            (
                board_shift,
                new_tile_handler,
                render_tile_points,
                render_tiles,
                end_game,
            )
                .chain()
                .run_if(in_state(RunState::Playing)),
        )
        .add_systems(
            OnEnter(RunState::Playing),
            (game_reset, spawn_tiles),
        )
        .add_event::<NewTileEvent>()
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_board(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    board: Res<Board>,
) {
    let panel_slicer = TextureSlicer {
        border: BorderRect::square(20.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };
    commands
        .spawn((SpriteBundle {
            texture: asset_server.load("panel.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(
                    board.physical_size + 70.,
                )),
                ..default()
            },
            ..default()
        }, ImageScaleMode::Sliced(
            panel_slicer.clone(),
        ),))
        .with_children(|builder| {
            for tile in board.tiles() {
                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: colors::palette::TILE_PLACEHOLDER,
                        custom_size: Some(Vec2::splat(
                            TILE_SIZE,
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
        });
}

fn spawn_tiles(mut commands: Commands, board: Res<Board>) {
    let mut rng = rand::thread_rng();
    let starting_tiles: Vec<(u8, u8)> =
        board.tiles().choose_multiple(&mut rng, 2);
    for (x, y) in starting_tiles.iter() {
        let pos = Position { x: *x, y: *y };
        spawn_tile(
            &mut commands,
            &board,
            pos,
            #[cfg(test)]
            {
                Points { value: 2 }
            },
        );
    }
}

fn render_tile_points(
    mut texts: Query<
        (&mut Text, &mut Transform),
        With<TileText>,
    >,
    tiles: Query<(&Points, &Children)>,
) {
    for (points, children) in tiles.iter() {
        if let Some(entity) = children.first() {
            let mut text = texts
                .get_mut(*entity)
                .expect("expected Text to exist");
            text.0.sections[0].value =
                points.value.to_string();
            *text.1 = text.1.with_scale(Vec3::splat(
                1.0 / points.value.to_string().len() as f32,
            ));
        }
    }
}

fn board_shift(
    board: Res<Board>,
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut tiles: Query<(Entity, &mut Position, &mut Points)>,
    mut tile_writer: EventWriter<NewTileEvent>,
    mut game: ResMut<Game>,
) {
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
                    column += 1;
                } else {
                    // merge
                    // despawn the next tile, and
                    // merge it with the current
                    // tile.
                    let real_next_tile = it.next()
                                    .expect("A peeked tile should always exist when we .next here");
                    tile.2.value += real_next_tile.2.value;

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
                            column += 1;
                        }
                    }
                }
            }
        }
        tile_writer.send(NewTileEvent);
    }
    if game.score_best < game.score {
        game.score_best = game.score;
    }
}

fn render_tiles(
    mut commands: Commands,
    tiles: Query<
        (Entity, &Transform, &Position),
        Changed<Position>,
    >,
    board: Res<Board>,
) {
    for (entity, transform, pos) in tiles.iter() {
        let x = board.cell_position_to_physical(pos.x);
        let y = board.cell_position_to_physical(pos.y);

        commands.entity(entity).insert(transform.ease_to(
            Transform::from_xyz(
                x,
                y,
                transform.translation.z,
            ),
            EaseFunction::QuadraticInOut,
            EasingType::Once {
                duration: std::time::Duration::from_millis(
                    100,
                ),
            },
        ));
    }
}

fn new_tile_handler(
    mut tile_reader: EventReader<NewTileEvent>,
    mut commands: Commands,
    board: Res<Board>,
    tiles: Query<&Position>,
) {
    for _event in tile_reader.read() {
        // insert new tile
        let mut rng = rand::thread_rng();
        let possible_position: Option<Position> = board
            .tiles()
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
                &board,
                pos,
                #[cfg(test)]
                {
                    Points { value: 2 }
                },
            );
        }
    }
}

fn spawn_tile(
    commands: &mut Commands,
    board: &Board,
    pos: Position,
    #[cfg(test)] points: Points,
) {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: colors::palette::TILE,
                    custom_size: Some(Vec2::splat(
                        TILE_SIZE,
                    )),
                    ..default()
                },
                transform: Transform::from_xyz(
                    board.cell_position_to_physical(pos.x),
                    board.cell_position_to_physical(pos.y),
                    2.0,
                ),
                ..default()
            },
            #[cfg(test)]
            {
                points
            },
            #[cfg(not(test))]
            {
                Points { value: 2 }
            },
            pos,
        ))
        .with_children(|child_builder| {
            child_builder
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        "2",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    )
                    .with_justify(JustifyText::Center),
                    transform: Transform::from_xyz(
                        0.0, 0.0, 1.0,
                    ),
                    ..default()
                })
                .insert(TileText);
        });
}

fn end_game(
    tiles: Query<(&Position, &Points)>,
    board: Res<Board>,
    mut next_state: ResMut<NextState<RunState>>,
) {
    if tiles.iter().len() == 16 {
        let map: HashMap<&Position, &Points> =
            tiles.iter().collect();

        let neighbor_points =
            [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let board_range: Range<i8> = 0..(board.size as i8);

        let has_move = tiles.iter().any(
            |(Position { x, y }, value)| {
                neighbor_points
                    .iter()
                    .filter_map(|(x2, y2)| {
                        let new_x = *x as i8 - x2;
                        let new_y = *y as i8 - y2;

                        if !board_range.contains(&new_x)
                            || !board_range.contains(&new_y)
                        {
                            return None;
                        };

                        map.get(&Position {
                            x: new_x.try_into().unwrap(),
                            y: new_y.try_into().unwrap(),
                        })
                    })
                    .any(|&v| v == value)
            },
        );

        if !has_move {
            next_state.set(RunState::GameOver);
        }
    };
}

fn game_reset(
    mut commands: Commands,
    tiles: Query<Entity, With<Position>>,
    mut game: ResMut<Game>,
) {
    for entity in tiles.iter() {
        commands.entity(entity).despawn_recursive();
    }
    game.score = 0;
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::CommandQueue;

    use super::*;

    #[test]
    fn gameover_triggers_when_16_tiles_exist() {
        let mut app = App::new();
        let board = Board::new(4);
        app.insert_resource(Board::new(4))
            .init_state::<RunState>()
            .add_systems(Startup, (spawn_board).chain())
            .add_systems(
                Update,
                (
                    end_game,
                    // apply_state_transition here is
                    // optional, but would require an
                    // additional
                    // app.update() cycle to run if we
                    // didn't include it here.
                    apply_state_transition::<RunState>,
                )
                    .chain(),
            );

        // insert tiles to set up a game
        let mut command_queue = CommandQueue::default();

        let mut commands =
            Commands::new(&mut command_queue, &app.world);
        for (i, (x, y)) in board.tiles().enumerate() {
            spawn_tile(
                &mut commands,
                &board,
                Position { x, y },
                Points {
                    value: 2_u32.pow(i as u32),
                },
            );
        }

        command_queue.apply(&mut app.world);

        // Run systems to insert tiles.
        // Game over is also detected immediately, but the
        // NextState resource must be used after
        // it is inserted by `apply_state_transition`
        // system
        app.update();

        let state = app
            .world
            .get_resource::<State<RunState>>()
            .expect("state to be inserted");

        assert_eq!(&RunState::GameOver, state.get());
    }
}
