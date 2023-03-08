use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_bad_sdr_bloom::{BloomPlugin, BloomSettings};

use common::settings::BloomSettingsPlugin; // Only needed for the demo

mod common;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(BloomPlugin)
        .add_plugin(BloomSettingsPlugin) // Only needed for the demo
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2dBundle::default(),
        BloomSettings {
            threshold: 0.11, // Just above the clear color
            intensity: 0.5,
            scale: 1.0,
            ..default()
        },
    ));

    let (saturation, lightness, alpha) = (0.8, 0.7, 1.0);

    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::Hsla {
                hue: 300.,
                saturation,
                lightness,
                alpha,
            },
            custom_size: Some(Vec2::new(100.0, 200.0)),
            ..default()
        },
        ..default()
    });

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(100.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::Hsla {
            hue: 180.,
            saturation,
            lightness,
            alpha,
        })),
        transform: Transform::from_translation(Vec3::new(-250., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(100., 6).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::Hsla {
            hue: 120.,
            saturation,
            lightness,
            alpha,
        })),
        transform: Transform::from_translation(Vec3::new(250., 0., 0.)),
        ..default()
    });
}
