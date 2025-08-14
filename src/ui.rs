use bevy::prelude::*;
use crate::core::GridDimensions;
use crate::GenerateWFCEvent;

// UI Components
#[derive(Component)]
pub enum DimensionButton {
    XIncrease,
    XDecrease,
    YIncrease,
    YDecrease,
    ZIncrease,
    ZDecrease,
}

#[derive(Component)]
pub struct GenerateButton;

// UI State Resource
#[derive(Resource)]
pub struct UIState {
    pub button_entity: Option<Entity>,
    pub x_text: Option<Entity>,
    pub y_text: Option<Entity>,
    pub z_text: Option<Entity>,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            button_entity: None,
            x_text: None,
            y_text: None,
            z_text: None,
        }
    }
}

pub fn setup_ui(
    mut commands: Commands,
    mut ui_state: ResMut<UIState>,
    dimensions: Res<GridDimensions>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "WFC Grid Controls",
                TextStyle {
                    font_size: 24.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ));

            create_dimension_control(
                parent, "X", dimensions.x, &mut ui_state,
                DimensionButton::XIncrease, DimensionButton::XDecrease,
                |state, entity| state.x_text = Some(entity)
            );

            create_dimension_control(
                parent, "Y", dimensions.y, &mut ui_state,
                DimensionButton::YIncrease, DimensionButton::YDecrease,
                |state, entity| state.y_text = Some(entity)
            );

            create_dimension_control(
                parent, "Z", dimensions.z, &mut ui_state,
                DimensionButton::ZIncrease, DimensionButton::ZDecrease,
                |state, entity| state.z_text = Some(entity)
            );

            let button = parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    border_color: BorderColor(Color::srgb(0.0, 0.0, 0.0)),
                    background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    ..default()
                },
                GenerateButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Generate WFC",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb(1.0, 1.0, 1.0),
                        ..default()
                    },
                ));
            })
            .id();

            ui_state.button_entity = Some(button);
        });
    });
}

fn create_dimension_control<F>(
    parent: &mut ChildBuilder,
    label: &str,
    value: usize,
    ui_state: &mut UIState,
    increase_button: DimensionButton,
    decrease_button: DimensionButton,
    set_text_entity: F,
) where
    F: FnOnce(&mut UIState, Entity),
{
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.0),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            format!("{}: ", label),
            TextStyle {
                font_size: 18.0,
                color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            },
        ));

        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    border: UiRect::all(Val::Px(1.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ..default()
            },
            decrease_button,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "-",
                TextStyle {
                    font_size: 20.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ));
        });

        let text_entity = parent.spawn(TextBundle::from_section(
            format!("{:3}", value),
            TextStyle {
                font_size: 18.0,
                color: Color::srgb(1.0, 1.0, 0.5),
                ..default()
            },
        )).id();

        set_text_entity(ui_state, text_entity);

        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    border: UiRect::all(Val::Px(1.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ..default()
            },
            increase_button,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "+",
                TextStyle {
                    font_size: 20.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ));
        });
    });
}

// Combine all UI interaction handlers into one public function
pub fn handle_ui_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&GenerateButton>, Option<&DimensionButton>, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut event_writer: EventWriter<GenerateWFCEvent>,
    mut dimensions: ResMut<GridDimensions>,
    ui_state: Res<UIState>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, gen_button, dim_button, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if gen_button.is_some() {
                    *color = BackgroundColor(Color::srgb(0.35, 0.75, 0.35));
                    border_color.0 = Color::srgb(0.0, 1.0, 0.0);
                    event_writer.send(GenerateWFCEvent);
                } else if let Some(dim_button) = dim_button {
                    *color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4));

                    match dim_button {
                        DimensionButton::XIncrease => dimensions.x = (dimensions.x + 1).min(20),
                        DimensionButton::XDecrease => dimensions.x = (dimensions.x.saturating_sub(1)).max(1),
                        DimensionButton::YIncrease => dimensions.y = (dimensions.y + 1).min(20),
                        DimensionButton::YDecrease => dimensions.y = (dimensions.y.saturating_sub(1)).max(1),
                        DimensionButton::ZIncrease => dimensions.z = (dimensions.z + 1).min(20),
                        DimensionButton::ZDecrease => dimensions.z = (dimensions.z.saturating_sub(1)).max(1),
                    }

                    // Update display text
                    if let Some(x_text) = ui_state.x_text {
                        if let Ok(mut text) = text_query.get_mut(x_text) {
                            text.sections[0].value = format!("{:3}", dimensions.x);
                        }
                    }
                    if let Some(y_text) = ui_state.y_text {
                        if let Ok(mut text) = text_query.get_mut(y_text) {
                            text.sections[0].value = format!("{:3}", dimensions.y);
                        }
                    }
                    if let Some(z_text) = ui_state.z_text {
                        if let Ok(mut text) = text_query.get_mut(z_text) {
                            text.sections[0].value = format!("{:3}", dimensions.z);
                        }
                    }
                }
            }
            Interaction::Hovered => {
                if gen_button.is_some() {
                    *color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
                    border_color.0 = Color::srgb(1.0, 1.0, 1.0);
                } else {
                    *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
                }
            }
            Interaction::None => {
                if gen_button.is_some() {
                    *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
                    border_color.0 = Color::srgb(0.0, 0.0, 0.0);
                } else {
                    *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
                }
            }
        }
    }
}