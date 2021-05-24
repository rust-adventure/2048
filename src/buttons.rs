use bevy::prelude::*;

use crate::events::GameResetEvent;

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .unwrap();
        ButtonMaterials {
            normal: materials
                .add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials
                .add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials
                .add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

pub fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut Handle<ColorMaterial>,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut game_reset_writer: EventWriter<GameResetEvent>,
) {
    for (interaction, mut material, children) in
        interaction_query.iter_mut()
    {
        let mut text =
            text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value =
                    "Press".to_string();
                *material =
                    button_materials.pressed.clone();
                game_reset_writer.send(GameResetEvent);
            }
            Interaction::Hovered => {
                text.sections[0].value =
                    "Hover".to_string();
                *material =
                    button_materials.hovered.clone();
            }
            Interaction::None => {
                text.sections[0].value =
                    "Button".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}
