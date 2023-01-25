use bevy::prelude::*;
use bevy_bad_sdr_bloom::BloomSettings;

pub struct BloomSettingsPlugin;

impl Plugin for BloomSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_system(update_settings);
        app.add_system(update_ui.after(update_settings));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let style = TextStyle {
        font: asset_server.load("FiraMono-Medium.ttf"),
        font_size: 18.0,
        color: Color::Hsla {
            hue: 207.,
            saturation: 0.80,
            lightness: 0.66,
            alpha: 1.,
        },
    };

    commands.spawn(
        TextBundle::from_sections([TextSection {
            value: "".to_string(),
            style,
        }])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            ..default()
        }),
    );
}

fn update_settings(
    mut settings: Query<&mut BloomSettings>,
    keycode: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(mut settings) = settings.get_single_mut() else { return };

    let dt = time.delta_seconds() / 5.;

    if keycode.pressed(KeyCode::Q) {
        settings.threshold -= dt;
    }
    if keycode.pressed(KeyCode::W) {
        settings.threshold += dt;
    }

    if keycode.pressed(KeyCode::E) {
        settings.knee -= dt;
    }
    if keycode.pressed(KeyCode::R) {
        settings.knee += dt;
    }

    if keycode.pressed(KeyCode::A) {
        settings.scale -= dt;
    }
    if keycode.pressed(KeyCode::S) {
        settings.scale += dt;
    }

    if keycode.pressed(KeyCode::D) {
        settings.intensity -= dt;
    }
    if keycode.pressed(KeyCode::F) {
        settings.intensity += dt;
    }
}

fn update_ui(settings: Query<&BloomSettings, Changed<BloomSettings>>, mut text: Query<&mut Text>) {
    let Ok(settings) = settings.get_single() else { return };
    let mut text = text.single_mut();

    text.sections[0].value = format!(
        "BloomSettings
-------------
[Q/W] Threshold: {:.3}
[E/R] Knee: {:.3}
[A/S] Scale: {:.3}
[D/F] Intensity: {:.3}",
        settings.threshold, settings.knee, settings.scale, settings.intensity
    );
}
