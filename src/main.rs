use bevy::{
    color::palettes::tailwind::*,
    dev_tools::states::log_transitions, math::U16Vec2,
    prelude::*,
};
use itertools::Itertools;
use rand::prelude::*;
use std::{
    cmp::Ordering,
    collections::HashMap,
    convert::{TryFrom, TryInto},
};

mod ui;
use ui::*;

#[derive(Event)]
struct NewTileEvent;

#[derive(Resource, Clone)]
struct Board {
    size: u16,
    world_size: f32,
    tile_size: f32,
    tile_spacer: f32,
}

impl Board {
    fn new(size: u16) -> Self {
        let tile_size: f32 = 80.0;
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

    fn tiles(&self) -> impl Iterator<Item = (u16, u16)> {
        (0..self.size).cartesian_product(0..self.size)
    }
}

#[derive(Debug, PartialEq, Component)]
struct Points {
    value: u32,
}

#[derive(
    Debug,
    PartialEq,
    Copy,
    Clone,
    Eq,
    Hash,
    Component,
    Deref,
    DerefMut,
)]
struct Position(U16Vec2);

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
        board_size: u16,
        position: &mut Position,
        index: u16,
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
    fn get_row_position(&self, position: &Position) -> u16 {
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
    Startup,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Srgba::hex("#1f2638").unwrap().into(),
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
            GameUiPlugin,
        ))
        .init_resource::<Game>()
        .init_state::<RunState>()
        .add_systems(OnEnter(RunState::Startup), setup)
        .add_systems(
            Update,
            (
                board_shift,
                render_tile_points,
                render_tiles,
                end_game,
            )
                .chain()
                .run_if(in_state(RunState::Playing)),
        )
        .add_systems(OnEnter(RunState::Playing), game_reset)
        .add_observer(new_tile_handler)
        // optional logging to view state transitions
        .add_systems(Update, log_transitions::<RunState>)
        .run();
}

/// Spawn a 2d camera with a specific vertical size
/// in world units. The Transform moves the camera
/// into a position where the vertical size fits the
/// board near the bottom of the screen, with some space
/// at the top for the scoreboard
fn setup(
    mut commands: Commands,
    board: Res<Board>,
    mut next_state: ResMut<NextState<RunState>>,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode:
                bevy::camera::ScalingMode::FixedVertical {
                    viewport_height: 600.,
                },
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(0., 100., 1.),
    ));

    let tiles: Vec<_> = board
        .tiles()
        .map(|tile| {
            (
                Sprite {
                    color: SLATE_500.into(),
                    custom_size: Some(Vec2::splat(
                        board.tile_size,
                    )),
                    ..default()
                },
                Transform::from_xyz(
                    board.grid_to_world_position(tile.0),
                    board.grid_to_world_position(tile.1),
                    1.0,
                ),
            )
        })
        .collect();

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::splat(
                board.world_size,
            )),
            color: SLATE_600.into(),
            ..default()
        },
        Children::spawn(tiles),
    ));

    next_state.set(RunState::Playing);
}

/// Keep the TileText values up to date with the
/// Points value. The Points value lives on the root
/// of the entity so we query for the Text2d we want
/// to mutate, then iterate up the Parent chain to
/// find the relevant Points component.
fn render_tile_points(
    mut texts: Query<
        (Entity, &mut Text2d, &mut TextFont),
        With<TileText>,
    >,
    points: Query<&Points>,
    entities_with_parents: Query<&ChildOf>,
) {
    for (entity, mut text2d, mut text_font) in &mut texts {
        let Some(points) = entities_with_parents
            .iter_ancestors(entity)
            .find_map(|entity| points.get(entity).ok())
        else {
            warn!(
                "Entity {entity:?} text2d with TileText doesn't have a Points Component in its ancestor tree"
            );
            continue;
        };

        let points_string = points.value.to_string();
        let points_length = points_string.len();
        text2d.0 = points.value.to_string();

        text_font.font_size = match points_length {
            1 => 40.,
            2 => 35.,
            3 => 30.,
            _ => 25.,
        };
    }
}

/// Shift the tiles on the board in a NSEW direction
fn board_shift(
    board: Res<Board>,
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut tiles: Query<(Entity, &mut Position, &mut Points)>,
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
        let mut column: u16 = 0;

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
                        .despawn();

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

        // spawn a new tile
        commands.trigger(NewTileEvent);
    }

    // update the high score if our current score is higher
    if game.score_best < game.score {
        game.score_best = game.score;
    }
}

/// Move a tile to its new Position if its Position has changed
fn render_tiles(
    mut commands: Commands,
    tiles: Query<
        (Entity, &Transform, &Position),
        Changed<Position>,
    >,
    board: Res<Board>,
) {
    for (entity, transform, pos) in tiles.iter() {
        commands.entity(entity).insert(
            Transform::from_xyz(
                board.grid_to_world_position(pos.x),
                board.grid_to_world_position(pos.y),
                transform.translation.z,
            ),
        );
    }
}

/// Find a single position on an active game board that can
/// accept a Tile
fn new_tile_handler(
    _: On<NewTileEvent>,
    mut commands: Commands,
    board: Res<Board>,
    tiles: Query<&Position>,
) {
    // insert new tile
    let mut rng = rand::rng();
    let possible_position: Option<Position> = board
        .tiles()
        .filter_map(|tile_pos| {
            let new_pos = Position(U16Vec2::from(tile_pos));
            match tiles.iter().find(|pos| pos == &&new_pos)
            {
                Some(_) => None,
                None => Some(new_pos),
            }
        })
        .choose(&mut rng);

    if let Some(pos) = possible_position {
        commands.queue(SpawnTile {
            pos,
            points: Points { value: 2 },
        });
    }
}

/// A system that detects whether or not the game can continue
/// if it can't, the game state is set to GameOver
fn end_game(
    tiles: Query<(&Position, &Points)>,
    mut next_state: ResMut<NextState<RunState>>,
) {
    if tiles.iter().len() != 16 {
        // if the board isn't full, we by definition have more
        // moves, so continue playing.
        return;
    }

    let map: HashMap<&Position, &Points> =
        tiles.iter().collect();

    let neighbor_offsets =
        [IVec2::NEG_X, IVec2::X, IVec2::Y, IVec2::NEG_Y];

    // if any tile is next to a tile with the same point
    // value, then there is a valid move available
    let has_move =
        tiles.iter().any(|(Position(current), value)| {
            neighbor_offsets
                .into_iter()
                .filter_map(|neighbor_offset| {
                    let new = current.as_ivec2()
                        - neighbor_offset;
                    map.get(&Position(new.try_into().ok()?))
                })
                .any(|&v| v == value)
        });

    if !has_move {
        next_state.set(RunState::GameOver);
    }
}

fn game_reset(
    mut commands: Commands,
    mut game: ResMut<Game>,
) {
    game.score = 0;

    commands.trigger(NewTileEvent);
    commands.trigger(NewTileEvent);
}

struct SpawnTile {
    pos: Position,
    points: Points,
}

impl Command for SpawnTile {
    fn apply(self, world: &mut World) {
        let board = {
            let Some(board) = world.get_resource::<Board>()
            else {
                warn!(
                    "SpawnTile command requires a Res<Board> to exist"
                );
                return;
            };
            board.clone()
        };

        let Some(asset_server) =
            world.get_resource::<AssetServer>()
        else {
            warn!(
                "Spawning a tile requires an AssetServer to exist"
            );
            return;
        };

        let font = asset_server.load("Outfit-Black.ttf");

        world.commands().spawn((
            Sprite {
                color: SLATE_400.into(),
                custom_size: Some(Vec2::splat(
                    board.tile_size,
                )),
                ..default()
            },
            Transform::from_xyz(
                board.grid_to_world_position(self.pos.x),
                board.grid_to_world_position(self.pos.y),
                2.0,
            ),
            self.points,
            self.pos,
            DespawnOnExit(RunState::GameOver),
            children![(
                Text2d("2".to_string()),
                TextFont {
                    font,
                    font_size: 80.,
                    ..default()
                },
                TextColor(Color::BLACK),
                TextLayout {
                    justify: Justify::Center,
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 1.0),
                TileText,
            )],
        ));
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::world::CommandQueue;

    use super::*;

    #[test]
    fn gameover_triggers_when_16_tiles_exist() {
        let mut app = App::new();
        let board = Board::new(4);
        app.add_plugins((
            MinimalPlugins,
            bevy::state::app::StatesPlugin,
            bevy::asset::AssetPlugin::default(),
            bevy::image::ImagePlugin::default(),
        ))
        .init_asset::<Font>()
        .insert_resource(Board::new(4))
        .init_state::<RunState>()
        .add_systems(OnEnter(RunState::Startup), setup)
        .add_systems(
            Update,
            end_game.run_if(in_state(RunState::Playing)),
        );

        // insert tiles to set up a game
        let mut command_queue = CommandQueue::default();

        let mut commands =
            Commands::new(&mut command_queue, app.world());

        for (i, (x, y)) in board.tiles().enumerate() {
            commands.queue(SpawnTile {
                pos: Position(U16Vec2::new(x, y)),
                points: Points {
                    value: 2_u32.pow(i as u32),
                },
            });
        }

        command_queue.apply(app.world_mut());

        // Run systems to insert tiles.
        // Game over is also detected immediately, but the
        // NextState resource must be used after
        // it is inserted by `apply_state_transition`
        // system
        app.update();
        app.world_mut().run_schedule(StateTransition);

        let state = app
            .world()
            .get_resource::<State<RunState>>()
            .expect("state to be inserted");

        assert_eq!(&RunState::GameOver, state.get());
    }
}
