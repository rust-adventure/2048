use crate::components::RunState;
use bevy::prelude::*;

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.75, 0.75, 0.9).into()),
            hovered: materials.add(Color::rgb(0.7, 0.7, 0.9).into()),
            pressed: materials.add(Color::rgb(0.6, 0.6, 1.0).into()),
        }
    }
}

pub fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut run_state: ResMut<State<RunState>>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query
            .get_mut(
                *children
                    .first()
                    .expect("expect button to have a first child"),
            )
            .unwrap();
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();

                match run_state.current() {
                    RunState::Playing => {
                        run_state.set(RunState::GameOver).unwrap();
                    }
                    RunState::GameOver => {
                        run_state.set(RunState::Playing).unwrap();
                    }
                }
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                match run_state.current() {
                    RunState::Playing => {
                        text.sections[0].value = "End Game".to_string();
                    }
                    RunState::GameOver => {
                        text.sections[0].value = "New Game".to_string();
                    }
                }

                *material = button_materials.normal.clone();
            }
        }
    }
}
