use bevy::prelude::*;

pub const SCORE_CONTAINER: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::ColumnReverse;
    style.align_items = AlignItems::Center;
    style.padding = UiRect {
        left: Val::Px(20.0),
        right: Val::Px(20.0),
        top: Val::Px(20.0),
        bottom: Val::Px(20.0),
    };
    style.min_width = Val::Px(100.);
    style
};
