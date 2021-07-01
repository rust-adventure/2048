use bevy::prelude::*;

const TILE_SIZE: f32 = 40.0;

struct Board {
    size: u8,
}

struct Materials {
    board: Handle<ColorMaterial>,
}

impl FromWorld for Materials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .unwrap();
        Materials {
            board: materials
                .add(Color::rgb(0.7, 0.7, 0.8).into()),
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
    let physical_board_size =
        f32::from(board.size) * TILE_SIZE;

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.board.clone(),
            sprite: Sprite::new(Vec2::new(
                physical_board_size,
                physical_board_size,
            )),
            ..Default::default()
        })
        .insert(board);
}
