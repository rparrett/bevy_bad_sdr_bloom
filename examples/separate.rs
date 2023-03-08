use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::CameraOutputMode,
        render_resource::{BlendState, LoadOp},
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
};
use bevy_bad_sdr_bloom::{BloomPlugin, BloomSettings};

use common::settings::BloomSettingsPlugin; // Only needed for controlling the demo

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
        .add_plugin(BloomSettingsPlugin) // Only needed for controlling the demo
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let no_bloom_layer = RenderLayers::layer(0);
    let bloom_layer = RenderLayers::layer(1);

    commands.spawn((Camera2dBundle::default(), no_bloom_layer));

    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                // Transparent clear color to ensure this layers on top of the output correctly
                clear_color: ClearColorConfig::Custom(Color::NONE),
                ..default()
            },
            camera: Camera {
                order: 1,
                // Disable MSAA writeback as we don't want the last camera's outputs in our
                // intermediate multisampled textures
                msaa_writeback: false,
                // Write to the output texture, but load the previous state + blend
                output_mode: CameraOutputMode::Write {
                    blend_state: Some(BlendState::ALPHA_BLENDING),
                    color_attachment_load_op: LoadOp::Load,
                },
                ..default()
            },
            ..default()
        },
        BloomSettings {
            threshold: 0.01,
            intensity: 0.5,
            scale: 1.0,
            ..default()
        },
        UiCameraConfig { show_ui: false },
        bloom_layer,
    ));

    let (saturation, lightness, alpha) = (0.8, 0.7, 1.0);

    // Circle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(100.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::Hsla {
                hue: 180.,
                saturation,
                lightness,
                alpha,
            })),
            transform: Transform::from_translation(Vec3::new(-125., 0., 0.)),
            ..default()
        },
        no_bloom_layer,
    ));

    // Hexagon
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::RegularPolygon::new(100., 6).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::Hsla {
                hue: 60.,
                saturation,
                lightness,
                alpha,
            })),
            transform: Transform::from_translation(Vec3::new(125., 0., 0.)),
            ..default()
        },
        bloom_layer,
    ));
}
