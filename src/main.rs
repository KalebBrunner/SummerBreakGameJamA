use bevy::{prelude::*, window::WindowResolution};

fn main() {
    let res_settings = ResolutionSettings {
        large: Vec2::new(1080.0, 1080.0),
        medium: Vec2::new(600.0, 600.0),
        small: Vec2::new(360.0, 360.0),
    };

    let initial_size = res_settings.small.as_uvec2();

    let window_plugin_with_initial_settings = WindowPlugin {
        primary_window: Some(Window {
            title: "Snek".into(),
            resolution: WindowResolution::new(initial_size.x, initial_size.y),
            resizable: false,
            ..default()
        }),
        ..default()
    };
    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin_with_initial_settings))
        .insert_resource(res_settings)
        .add_systems(Startup, setup_window)
        .add_systems(Update, (toggle_resolution, draw_grid))
        .run();
}

/// Stores the various window-resolutions we can select between.
#[derive(Resource)]
struct ResolutionSettings {
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}

fn draw_grid(window: Single<&mut Window>, mut gizmos: Gizmos) {
    let height = window.height() - 2.0;
    let width = window.width() - 2.0;

    let square_count = 10;

    let perspective_shift2 = |coord: Vec2| {
        return vec2(coord.x - width / 2.0, coord.y - height / 2.0);
    };

    for i in 0..square_count + 1 {
        let spacing = width / (square_count as f32);

        let i2 = i as f32;
        gizmos.line_2d(
            perspective_shift2(Vec2::new(spacing * i2, height)),
            perspective_shift2(Vec2::new(spacing * i2, 0.0)),
            Color::srgb(1.0, 0.2, 0.2),
        );
    }

    for i in 0..square_count + 1 {
        let spacing = height / (square_count as f32);

        let i2 = i as f32;
        gizmos.line_2d(
            perspective_shift2(Vec2::new(width, spacing * i2)),
            perspective_shift2(Vec2::new(0.0, spacing * i2)),
            Color::srgb(1.0, 0.2, 0.2),
        );
    }
}

fn setup_window(mut commands: Commands, mut window: Single<&mut Window>) {
    window.resizable = false;
    commands.spawn(Camera2d);
}

/// This system shows how to request the window to a new resolution
fn toggle_resolution(
    keys: Res<ButtonInput<KeyCode>>,
    mut window: Single<&mut Window>,
    resolution: Res<ResolutionSettings>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        let res = resolution.small;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Digit2) {
        let res = resolution.medium;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Digit3) {
        let res = resolution.large;
        window.resolution.set(res.x, res.y);
    }
}
