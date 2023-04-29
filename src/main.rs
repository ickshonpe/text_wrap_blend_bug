use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_unwrapped)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    for (i, (position_type, message, right)) in [
        (
            PositionType::Relative,
            "Relative Positioning\nRelative Positioning",
            Val::Auto,
        ),
        (
            PositionType::Absolute,
            "Absolute Positioning\nAbsolute Positioning",
            Val::Auto,
        ),
        (
            PositionType::Absolute,
            "Absolute Positioning\nAbsolute Positioning",
            Val::Px(0.),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let x = i as f32 * 150.;
        commands
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.),
                    left: Val::Px(10. + x),
                    right,
                    ..default()
                },
                background_color: Color::MAROON.into(),
                ..default()
            })
            .with_children(|builder| {
                builder.spawn(
                    TextBundle::from_section(message, TextStyle::default())
                        .with_style(Style {
                            position_type,
                            ..default()
                        })
                        .with_background_color(Color::DARK_GREEN),
                );
            });
    }
}


fn setup_unwrapped(mut commands: Commands) {
    for (i, (position_type, message, right)) in [
        (
            PositionType::Absolute,
            "Absolute Positioning\nAbsolute Positioning",
            Val::Auto,
        ),
        (
            PositionType::Absolute,
            "Absolute Positioning\nAbsolute Positioning",
            Val::Px(0.),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let x = i as f32 * 150.;
        commands.spawn(
            TextBundle::from_section(message, TextStyle::default())
                .with_style(Style {
                    top: Val::Px(200. + i as f32 * 100.),
                    left: Val::Px(10. + x),
                    right,
                    position_type,
                    ..default()
                })
                .with_background_color(Color::DARK_GREEN),
        );
    }
}
