use bevy::prelude::*;
use itertools::Itertools;

const TILE_SPACER: f32 = 10.0;
const TILE_SIZE: f32 = 40.0;
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
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
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
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.insert_resource(Materials {
        board: materials
            .add(Color::rgb(0.7, 0.7, 0.8).into()),
        tile_placeholder: materials
            .add(Color::rgb(0.75, 0.75, 0.9).into()),
        block: materials
            .add(Color::rgb(0.9, 0.9, 1.0).into()),
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

fn spawn_tiles(
    mut commands: Commands,
    materials: Res<Materials>,
    query_board: Query<&Board>,
    asset_server: Res<AssetServer>,
) {
    let board = query_board
        .single()
        .expect("always expect a board");
    let starting_tiles = vec![
        (0, 0),
        (0, 1),
        (3, 1),
        (0, 2),
        (2, 2),
        (3, 2),
        (0, 3),
        (1, 3),
        (2, 3),
        (3, 3),
    ];
    for (x, y) in starting_tiles.iter() {
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
        (&mut Transform, &Position),
        With<Block>,
    >,
    query_board: Query<&Board>,
) {
    let board = query_board
        .single()
        .expect("expect there to be a board");
    for (mut transform, pos) in blocks.iter_mut() {
        transform.translation.x = block_pos_to_transform(
            board.size.into(),
            pos.x.into(),
        );
        transform.translation.y = block_pos_to_transform(
            board.size.into(),
            pos.y.into(),
        );
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
) {
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
                    dbg!((block.1.x, block.1.y));
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
                            // merge it with the current block.
                            let real_next_block = it.next().expect("A peeked block should always exist when we .next here");
                            block.2.value = block.2.value
                                + real_next_block.2.value;
                            block.1.x = x;

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
                            // if the next, next block (block #3 of 3)
                            // isn't in the same row, reset x
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
            // dbg!(board.size - 1 - x);
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
                            // _recursive the next block, and
                            // merge it with the current block.
                            let real_next_block = it.next().expect("A peeked block should always exist when we .next here");
                            block.2.value = block.2.value
                                + real_next_block.2.value;
                            block.1.y = y;

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
                            // if the next, next block (block #3 of 3)
                            // isn't in the same row, reset x
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
            dbg!(y);
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
                            dbg!("Merge");
                            // despawn the next block, and
                            // merge it with the current block.
                            let real_next_block = it.next().expect("A peeked block should always exist when we .next here");
                            block.2.value = block.2.value
                                + real_next_block.2.value;
                            block.1.y = board.size - 1 - y;

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
                            // if the next, next block (block #3 of 3)
                            // isn't in the same row, reset x
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
                            dbg!("DifferentRows");
                            block.1.y = board.size - 1 - y;
                            y = 0;
                            continue;
                        }
                        MergeStatus::DifferentValues => {
                            println!(
                                "DifferentValues: {:?}",
                                board.size - 1 - y,
                            );
                            block.1.y = board.size - 1 - y;
                            y = y + 1;
                            continue;
                        }
                    }
                }
            }
            break;
        }
    }
}