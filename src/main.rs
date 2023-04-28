use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    for (i, (position_type, message)) in [
        (PositionType::Relative, "Relative Positioning"),
        (PositionType::Absolute, "Absolute Positioning"),
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
