use bevy::prelude::*;
use bevy_easings::*;
use itertools::Itertools;
use rand::{thread_rng, Rng};

mod buttons;
mod events;

use buttons::*;
use events::*;

const TILE_SPACER: f32 = 10.0;
const TILE_SIZE: f32 = 40.0;

#[derive(PartialEq, Eq)]
struct Position {
    x: u8,
    y: u8,
}
struct Block {
    value: u32,
}
struct BlockText;

struct Board {
    size: u8,
}
struct ScoreDisplay;

#[derive(Default)]
struct Game {
    score: u32,
}

struct Materials {
    board: Handle<ColorMaterial>,
    tile_placeholder: Handle<ColorMaterial>,
    block: Handle<ColorMaterial>,
}
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "2048".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(
            0.04, 0.04, 0.1,
        )))
        .init_resource::<Game>()
        .add_startup_system(setup.system())
        .add_startup_system(setup_ui.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_easings::EasingsPlugin)
        .add_startup_stage(
            "game_setup",
            SystemStage::single(spawn_board.system()),
        )
        .add_startup_stage(
            "tile_start",
            SystemStage::single(spawn_tiles.system()),
        )
        .add_system(board_shift.system())
        .add_system(render_blocks.system())
        .add_system(new_tile_handler.system())
        .add_event::<NewTileEvent>()
        .init_resource::<ButtonMaterials>()
        .add_system(button_system.system())
        .add_system(game_reset.system())
        .add_event::<GameResetEvent>()
        .add_system(scoreboard.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands.insert_resource(Materials {
        board: materials
            .add(Color::rgb(0.7, 0.7, 0.8).into()),
        tile_placeholder: materials
            .add(Color::rgb(0.75, 0.75, 0.9).into()),
        block: materials
            .add(Color::rgb(0.9, 0.9, 1.0).into()),
    });
}

fn setup_ui(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    commands
    .spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            border: Rect::all(Val::Px(50.0)),
            // flex_grow: 1.0,
            ..Default::default()
        },
        material: materials.add(Color::NONE.into()),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "2048",
                TextStyle {
                    font: asset_server
                        .load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        });
    
        parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                border: Rect::all(Val::Px(50.0)),
                // flex_grow: 1.0,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Scoreboard",
                    TextStyle {
                        font: asset_server
                            .load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            }).insert(ScoreDisplay);
            parent.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(
                        Val::Px(100.0),
                        Val::Px(30.0),
                    ),
                    // center button
                    // margin: Rect::all(Val::Auto),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    margin: Rect {
                        left: Val::Px(20.0),
                        right: Val::Px(20.0),
                        top: Val::Px(20.0),
                        bottom: Val::Px(20.0),
                    },
                    ..Default::default()
                },
                material: button_materials.normal.clone(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load(
                                "fonts/FiraSans-Bold.ttf",
                            ),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
            });
        });
            
    });
}
fn spawn_board(
    mut commands: Commands,
    materials: Res<Materials>,
) {
    let board = Board { size: 4 };
    let physical_board_size = {
        // size of all tiles
        f32::from(board.size) * TILE_SIZE
        // size of all spacers
        + f32::from(board.size) * TILE_SPACER
        // extra spacer on the off side to round out the board
        + TILE_SPACER
    };

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.board.clone(),
            sprite: Sprite::new(Vec2::new(
                physical_board_size,
                physical_board_size,
            )),
            ..Default::default()
        })
        .with_children(|child_builder| {
            for tile in (0..board.size)
                .cartesian_product(0..board.size)
            {
                child_builder.spawn_bundle(SpriteBundle {
                    material: materials
                        .tile_placeholder
                        .clone(),
                    sprite: Sprite::new(Vec2::new(
                        f32::from(board.size) * 10.0,
                        f32::from(board.size) * 10.0,
                    )),
                    transform: Transform::from_xyz(
                        // true position
                        f32::from(tile.0) * TILE_SIZE
                        // moved left because it is at board center
                            - (f32::from(board.size)
                                * TILE_SIZE
                                / 2.0)
                                // moved right because it's even numbered
                                // (odd would be centered)
                            + (0.5 * TILE_SIZE)
                            // account for in-between spacing by applying N
                            // spacers
                            + f32::from(tile.0)
                                * TILE_SPACER
                            - TILE_SPACER * 1.5,
                        f32::from(tile.1) * TILE_SIZE
                            - (f32::from(board.size)
                                * TILE_SIZE
                                / 2.0)
                            + (0.5 * TILE_SIZE)
                            + f32::from(tile.1)
                                * TILE_SPACER
                            - TILE_SPACER * 1.5,
                        1.0,
                    ),
                    ..Default::default()
                });
            }
        })
        .insert(board);
}
fn game_reset(
    mut commands: Commands,
    materials: Res<Materials>,
    query_board: Query<&Board>,
    asset_server: Res<AssetServer>,
    mut game_reset_reader: EventReader<GameResetEvent>,
    mut blocks: Query<Entity, With<Block>>,
    mut game: ResMut<Game>,
) {
    if game_reset_reader.iter().next().is_some() {
        for entity in blocks.iter() {
            commands.entity(entity).despawn_recursive();
        }
        game.score = 0;
        spawn_tiles(
            commands,
            materials,
            query_board,
            asset_server,
        );
    }
}

// update the score displayed during the game
fn scoreboard(
    game: Res<Game>,
    mut query: Query<&mut Text, With<ScoreDisplay>>,
) {
    let mut text = query.single_mut().unwrap();
    text.sections[0].value =
        format!("Score: {}", game.score);
}
fn spawn_tiles(
    mut commands: Commands,
    materials: Res<Materials>,
    query_board: Query<&Board>,
    asset_server: Res<AssetServer>,
) {
    let board = query_board
        .single()
        .expect("always expect a board");
    // insert new tile
    let mut rng = rand::thread_rng();
    let mut starting_tiles: Vec<Position> = vec![];
    loop {
        if starting_tiles.len() >= 2 {
            break;
        }

        let pos = Position {
            x: rng.gen_range(0..board.size),
            y: rng.gen_range(0..board.size),
        };
        match starting_tiles.iter().find(|block_pos| {
            block_pos.x == pos.x && block_pos.y == pos.y
        }) {
            Some(_) => continue,
            None => {
                starting_tiles.push(pos);
            }
        };
    }
    for Position { x, y } in starting_tiles.iter() {
        let pos = Position { x: *x, y: *y };
        let tile: (u8, u8) = (pos.x, pos.y);
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.block.clone(),
                sprite: Sprite::new(Vec2::new(
                    f32::from(board.size) * 10.0,
                    f32::from(board.size) * 10.0,
                )),
                transform: Transform::from_xyz(
                    block_pos_to_transform(
                        board.size.into(),
                        tile.0.into(),
                    ),
                    block_pos_to_transform(
                        board.size.into(),
                        tile.1.into(),
                    ),
                    1.0,
                ),
                ..Default::default()
            })
            .with_children(|child_builder| {
                child_builder.spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        "2",
                        TextStyle {
                            font: asset_server.load(
                                "fonts/FiraSans-Bold.ttf",
                            ),
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
                }).insert(BlockText);
            })
            .insert(Block { value: 2 })
            .insert(pos);
    }
}

fn block_pos_to_transform(
    board_size: f32,
    pos: f32,
) -> f32 {
    f32::from(pos) * TILE_SIZE
        // moved left because it is at board center
            - (f32::from(board_size)
                * TILE_SIZE
                / 2.0)
                // moved right because it's even numbered
                // (odd would be centered)
            + (0.5 * TILE_SIZE)
            // account for in-between spacing by applying N
            // spacers
            + f32::from(pos)
                * TILE_SPACER
        - TILE_SPACER * 1.5
}
fn render_blocks(
    mut commands: Commands,
    mut blocks: Query<
        (
            Entity,
            &mut Transform,
            &Position,
            Changed<Position>,
        ),
        With<Block>,
    >,
    query_board: Query<&Board>,
) {
    let board = query_board
        .single()
        .expect("expect there to be a board");
    for (entity, mut transform, pos, pos_changed) in
        blocks.iter_mut()
    {
        if pos_changed {
            let x = block_pos_to_transform(
                board.size.into(),
                pos.x.into(),
            );
            let y = block_pos_to_transform(
                board.size.into(),
                pos.y.into(),
            );
            let mut ent = commands.entity(entity);
            ent.insert(transform.ease_to(
                Transform::from_xyz(
                    x,
                    y,
                    transform.translation.z,
                ),
                EaseFunction::QuadraticInOut,
                EasingType::Once {
                    duration:
                        std::time::Duration::from_millis(
                            100,
                        ),
                },
            ));
        }
    }
}

#[derive(Debug)]
enum MergeStatus {
    Merge,
    DifferentRows,
    DifferentValues,
}
// u32 is the block value
// u8 is the "vertical" y position,
// which is different for each direction
fn should_merge(
    block: (u32, u8),
    block_next: (u32, u8),
) -> MergeStatus {
    if block.1 != block_next.1 {
        // if the blocks aren't on the same level
        // they can't collide, so skip to next loop
        // iteration
        MergeStatus::DifferentRows
    } else if block.0 != block_next.0 {
        // if the block values don't match,
        // they can't merge, so skip to the
        // next loop iteration
        MergeStatus::DifferentValues
    } else {
        MergeStatus::Merge
    }
}
fn board_shift(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    // mut query_world: Query<&mut World>,
    mut texts: Query<(Entity, &mut Text), With<BlockText>>,
    mut blocks: Query<(
        Entity,
        &mut Position,
        &mut Block,
        &Children,
    )>,
    query_board: Query<&Board>,
    mut tile_writer: EventWriter<NewTileEvent>,
    mut game: ResMut<Game>,
) {

    // EndGameCheck
    // TODO: 
    // 1. End game if 16 blocks are on field
    // 2. end game with proper checks
    if blocks.iter_mut().len() == 16 {
        // get all four sorts for each board shift direction
        // check each sort for *any* merge possibility
        // if a merge is possible anywhere, break out immediately
        // if no merge possible anywhere, end game.
    };

    // Normal Processing
    let board = query_board
        .single()
        .expect("expect there to be a board");

    if keyboard_input.just_pressed(KeyCode::Left) {
        let mut it = blocks
            .iter_mut()
            .sorted_by(|a, b| {
                match Ord::cmp(&a.1.y, &b.1.y) {
                    std::cmp::Ordering::Equal => {
                        Ord::cmp(&a.1.x, &b.1.x)
                    }
                    ordering => ordering,
                }
            })
            // .sorted_by_key(|v| v.1.y)
            .peekable();
        let mut x: u8 = 0;

        loop {
            match (it.next(), it.peek()) {
                (None, _) => {
                    // nothing left to process
                    break;
                }
                (Some(mut block), None) => {
                    block.1.x = x;
                }
                (Some(mut block), Some(block_next)) => {
                    match should_merge(
                        (block.2.value, block.1.y),
                        (
                            block_next.2.value,
                            block_next.1.y,
                        ),
                    ) {
                        MergeStatus::Merge => {
                            // despawn the next block, and
                            // merge it with the current
                            // block.
                            let real_next_block = it.next().expect("A peeked block should always exist when we .next here");
                            block.2.value = block.2.value
                                + real_next_block.2.value;
                            block.1.x = x;

                            // update score
                            game.score += block.2.value;
                            // update text
                            for child in block.3.iter() {
                                let mut text = texts
                                    .get_mut(*child)
                                    .expect(
                                        "text to exist",
                                    );
                                let mut section = text.1.sections.first_mut().expect("expect a single section in text");
                                section.value = block
                                    .2
                                    .value
                                    .to_string();
                            }
                            // if the next, next block
                            // (block #3 of 3)
                            // isn't in the same row, reset
                            // x
                            // otherwise increment by one
                            if let Some(future) = it.peek()
                            {
                                if block.1.y != future.1.y {
                                    x = 0;
                                } else {
                                    x = x + 1;
                                }
                            }

                            commands
                                .entity(real_next_block.0)
                                .despawn_recursive();
                            continue;
                        }
                        MergeStatus::DifferentRows => {
                            block.1.x = x;
                            x = 0;
                            continue;
                        }
                        MergeStatus::DifferentValues => {
                            block.1.x = x;
                            x = x + 1;
                            continue;
                        }
                    }
                }
            }
            break;
        }
        // insert new block
        tile_writer.send(NewTileEvent);
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        let mut it = blocks
            .iter_mut()
            // we want our sorting to first sort by x,
            // then break x ties with y's comparison
            .sorted_by(|a, b| {
                match Ord::cmp(&b.1.y, &a.1.y) {
                    std::cmp::Ordering::Equal => {
                        Ord::cmp(&b.1.x, &a.1.x)
                    }
                    a => a,
                }
            })
            .peekable();
        let mut x: u8 = 0;
        loop {
            match (it.next(), it.peek()) {
                (None, _) => {
                    break;
                }
                (Some(mut block), None) => {
                    block.1.x = board.size - 1 - x;
                }
                (Some(mut block), Some(block_next)) => {
                    match should_merge(
                        (block.2.value, block.1.y),
                        (
                            block_next.2.value,
                            block_next.1.y,
                        ),
                    ) {
                        MergeStatus::Merge => {
                            let real_next_block = it.next().expect("A peeked block should always exist when we .next here");
                            block.2.value = block.2.value
                                + real_next_block.2.value;
                            block.1.x = board.size - 1 - x;

                            // update score
                            game.score += block.2.value;
                            // update text
                            for child in block.3.iter() {
                                let mut text = texts
                                    .get_mut(*child)
                                    .expect(
                                        "text to exist",
                                    );
                                let mut section = text.1.sections.first_mut().expect("expect a single section in text");
                                section.value = block
                                    .2
                                    .value
                                    .to_string();
                            }

                            if let Some(future) = it.peek()
                            {
                                if block.1.y != future.1.y {
                                    x = 0;
                                } else {
                                    x = x + 1;
                                }
                            }
                            commands
                                .entity(real_next_block.0)
                                .despawn_recursive();
                            continue;
                        }
                        MergeStatus::DifferentRows => {
                            block.1.x = board.size - 1 - x;
                            x = 0;
                            continue;
                        }
                        MergeStatus::DifferentValues => {
                            block.1.x = board.size - 1 - x;
                            x = x + 1;
                            continue;
                        }
                    }
                }
            }

            break;
        }
        // insert new block
        tile_writer.send(NewTileEvent);
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        let mut it = blocks
            .iter_mut()
            .sorted_by(|a, b| {
                match Ord::cmp(&a.1.x, &b.1.x) {
                    std::cmp::Ordering::Equal => {
                        Ord::cmp(&a.1.y, &b.1.y)
                    }
                    ordering => ordering,
                }
            })
            .peekable();
        let mut y: u8 = 0;

        loop {
            match (it.next(), it.peek()) {
                (None, _) => {
                    // nothing left to process
                    break;
                }
                (Some(mut block), None) => {
                    block.1.y = y;
                }
                (Some(mut block), Some(block_next)) => {
                    match should_merge(
                        (block.2.value, block.1.x),
                        (
                            block_next.2.value,
                            block_next.1.x,
                        ),
                    ) {
                        MergeStatus::Merge => {
                            // _recursive the next block,
                            // and
                            // merge it with the current
                            // block.
                            let real_next_block = it.next().expect("A peeked block should always exist when we .next here");
                            block.2.value = block.2.value
                                + real_next_block.2.value;
                            block.1.y = y;

                            // update score
                            game.score += block.2.value;
                            // update text
                            for child in block.3.iter() {
                                let mut text = texts
                                    .get_mut(*child)
                                    .expect(
                                        "text to exist",
                                    );
                                let mut section = text.1.sections.first_mut().expect("expect a single section in text");
                                section.value = block
                                    .2
                                    .value
                                    .to_string();
                            }
                            // if the next, next block
                            // (block #3 of 3)
                            // isn't in the same row, reset
                            // x
                            // otherwise increment by one
                            if let Some(future) = it.peek()
                            {
                                if block.1.x != future.1.x {
                                    y = 0;
                                } else {
                                    y = y + 1;
                                }
                            }
                            commands
                                .entity(real_next_block.0)
                                .despawn_recursive();
                            continue;
                        }
                        MergeStatus::DifferentRows => {
                            block.1.y = y;
                            y = 0;
                            continue;
                        }
                        MergeStatus::DifferentValues => {
                            block.1.y = y;
                            y = y + 1;
                            continue;
                        }
                    }
                }
            }
            break;
        }
        // insert new block
        tile_writer.send(NewTileEvent);
    } else if keyboard_input.just_pressed(KeyCode::Up) {
        let mut it = blocks
            .iter_mut()
            .sorted_by(|a, b| {
                match Ord::cmp(&b.1.x, &a.1.x) {
                    std::cmp::Ordering::Equal => {
                        Ord::cmp(&b.1.y, &a.1.y)
                    }
                    ordering => ordering,
                }
            })
            .peekable();
        let mut y: u8 = 0;

        loop {
            match (it.next(), it.peek()) {
                (None, _) => {
                    // nothing left to process
                    break;
                }
                (Some(mut block), None) => {
                    block.1.y = board.size - 1 - y;
                }
                (Some(mut block), Some(block_next)) => {
                    match should_merge(
                        (block.2.value, block.1.x),
                        (
                            block_next.2.value,
                            block_next.1.x,
                        ),
                    ) {
                        MergeStatus::Merge => {
                            // despawn the next block, and
                            // merge it with the current
                            // block.
                            let real_next_block = it.next().expect("A peeked block should always exist when we .next here");
                            block.2.value = block.2.value
                                + real_next_block.2.value;
                            block.1.y = board.size - 1 - y;

                            // update score
                            game.score += block.2.value;
                            // update text
                            for child in block.3.iter() {
                                let mut text = texts
                                    .get_mut(*child)
                                    .expect(
                                        "text to exist",
                                    );
                                let mut section = text.1.sections.first_mut().expect("expect a single section in text");
                                section.value = block
                                    .2
                                    .value
                                    .to_string();
                            }
                            // if the next, next block
                            // (block #3 of 3)
                            // isn't in the same row, reset
                            // x
                            // otherwise increment by one
                            if let Some(future) = it.peek()
                            {
                                if block.1.x != future.1.x {
                                    y = 0;
                                } else {
                                    y = y + 1;
                                }
                            }
                            commands
                                .entity(real_next_block.0)
                                .despawn_recursive();
                            continue;
                        }
                        MergeStatus::DifferentRows => {
                            block.1.y = board.size - 1 - y;
                            y = 0;
                            continue;
                        }
                        MergeStatus::DifferentValues => {
                            block.1.y = board.size - 1 - y;
                            y = y + 1;
                            continue;
                        }
                    }
                }
            }
            break;
        }
        // insert new block
        tile_writer.send(NewTileEvent);
    }
}

fn new_tile_handler(
    mut tile_reader: EventReader<NewTileEvent>,
    mut commands: Commands,
    query_board: Query<&Board>,
    asset_server: Res<AssetServer>,
    materials: Res<Materials>,
    mut blocks: Query<(&mut Position, &mut Block)>,
) {
    let board = query_board
        .single()
        .expect("expect there to always be a board");
    if tile_reader.iter().next().is_some() {
        // insert new tile
        let mut rng = rand::thread_rng();
        let pos = loop {
            let pos = Position {
                x: rng.gen_range(0..board.size),
                y: rng.gen_range(0..board.size),
            };
            match blocks.iter_mut().find(
                |(block_pos, _block)| {
                    block_pos.x == pos.x
                        && block_pos.y == pos.y
                },
            ) {
                Some(_) => continue,
                None => {
                    break pos;
                }
            }
        };
        let tile: (u8, u8) = (pos.x, pos.y);
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.block.clone(),
                sprite: Sprite::new(Vec2::new(
                    f32::from(board.size) * 10.0,
                    f32::from(board.size) * 10.0,
                )),
                transform: Transform::from_xyz(
                    block_pos_to_transform(
                        board.size.into(),
                        tile.0.into(),
                    ),
                    block_pos_to_transform(
                        board.size.into(),
                        tile.1.into(),
                    ),
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
                                font: asset_server.load(
                                    "fonts/FiraSans-Bold.ttf",
                                ),
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
                    .insert(BlockText);
            })
            .insert(Block { value: 2 })
            .insert(pos);
    }
}
