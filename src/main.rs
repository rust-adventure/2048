use bevy::{
    color::palettes::tailwind::*,
    dev_tools::states::log_transitions,
    ecs::world::Command, math::U16Vec2, prelude::*,
    render::camera::ScalingMode,
};
// use bevy_easings::*;
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
    physical_size: f32,
    tile_size: f32,
    tile_spacer: f32,
}

impl Board {
    fn new(size: u16) -> Self {
        let tile_size: f32 = 80.0;
        let tile_spacer: f32 = 10.0;

        let physical_size = f32::from(size) * tile_size
            + f32::from(size + 1) * tile_spacer;
        Board {
            size,
            physical_size,
            tile_size,
            tile_spacer,
        }
    }
    fn grid_to_world_position(&self, pos: u16) -> f32 {
        let offset = -self.physical_size / 2.0
            + 0.5 * self.tile_size;

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
            // EasingsPlugin,
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
        .add_systems(
            OnEnter(RunState::Playing),
            (game_reset, spawn_tiles),
        )
        .add_event::<NewTileEvent>()
        .add_observer(new_tile_handler)
        .add_systems(Update, log_transitions::<RunState>)
        .enable_state_scoped_entities::<RunState>()
        .run();
}

fn setup(
    mut commands: Commands,
    board: Res<Board>,
    mut next_state: ResMut<NextState<RunState>>,
) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 600.,
            },
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(0., 100., 1.),
    ));

    commands
        .spawn((Sprite {
            custom_size: Some(Vec2::splat(
                board.physical_size,
            )),
            color: SLATE_600.into(),
            ..default()
        },))
        .with_children(|builder| {
            for tile in board.tiles() {
                builder.spawn((
                    Sprite {
                        color: SLATE_500.into(),
                        custom_size: Some(Vec2::splat(
                            board.tile_size,
                        )),
                        ..default()
                    },
                    Transform::from_xyz(
                        board
                            .grid_to_world_position(tile.0),
                        board
                            .grid_to_world_position(tile.1),
                        1.0,
                    ),
                ));
            }
        });

    next_state.set(RunState::Playing);
}

fn spawn_tiles(mut commands: Commands, board: Res<Board>) {
    let mut rng = rand::thread_rng();
    let starting_tiles: Vec<(u16, u16)> =
        board.tiles().choose_multiple(&mut rng, 2);
    for (x, y) in starting_tiles.into_iter() {
        commands.queue(SpawnTile {
            pos: Position(U16Vec2::new(x, y)),
            points: Points { value: 2 },
        });
    }
}

// TODO: Find TileText on the right entity
// then iter_ancestors to find Points
fn render_tile_points(
    mut texts: Query<
        &mut Transform,
        (With<Text2d>, With<TileText>),
    >,
    mut writer: Text2dWriter,
    tiles: Query<(&Points, &Children)>,
) {
    for (points, children) in tiles.iter() {
        if let Some(entity) = children.first() {
            let mut transform = texts
                .get_mut(*entity)
                .expect("expected Text to exist");

            *writer.text(*entity, 0) =
                points.value.to_string();

            *transform = transform.with_scale(Vec3::splat(
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

        commands.trigger(NewTileEvent);
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
        let x = board.grid_to_world_position(pos.x);
        let y = board.grid_to_world_position(pos.y);

        commands.entity(entity).insert(
            Transform::from_xyz(
                x,
                y,
                transform.translation.z,
            ),
        );
        // commands.entity(entity).insert(transform.ease_to(
        //     Transform::from_xyz(
        //         x,
        //         y,
        //         transform.translation.z,
        //     ),
        //     EaseFunction::QuadraticInOut,
        //     EasingType::Once {
        //         duration: std::time::Duration::from_millis(
        //             100,
        //         ),
        //     },
        // ));
    }
}

fn new_tile_handler(
    _: Trigger<NewTileEvent>,
    mut commands: Commands,
    board: Res<Board>,
    tiles: Query<&Position>,
) {
    // insert new tile
    let mut rng = rand::thread_rng();
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

fn game_reset(mut game: ResMut<Game>) {
    game.score = 0;
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
                warn!("SpawnTile command requires a Res<Board> to exist");
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

        world
            .commands()
            .spawn((
                Sprite {
                    color: SLATE_400.into(),
                    custom_size: Some(Vec2::splat(
                        board.tile_size,
                    )),
                    ..default()
                },
                Transform::from_xyz(
                    board
                        .grid_to_world_position(self.pos.x),
                    board
                        .grid_to_world_position(self.pos.y),
                    2.0,
                ),
                self.points,
                self.pos,
                StateScoped(RunState::GameOver),
            ))
            .with_children(|child_builder| {
                child_builder.spawn((
                    Text2d("2".to_string()),
                    TextFont {
                        font,
                        font_size: 80.,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                    TextLayout {
                        justify: JustifyText::Center,
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, 1.0),
                    TileText,
                ));
            });
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
            bevy::render::texture::ImagePlugin::default(),
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
