use bevy::{prelude::*, window::WindowResolution};

#[derive(Resource)]
struct ResolutionSettings {
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}

#[derive(Resource)]
struct GridSettings {
    offset: f32,
    size: u32,
    block_size: f32,
}

impl GridSettings {
    fn new(window_height: f32) -> GridSettings {
        let initial_offset = 1.0;
        let initial_size = 10;

        GridSettings {
            offset: initial_offset,
            size: initial_size,
            block_size: (window_height - (initial_offset * 2.0)) / (initial_size as f32),
        }
    }
}

struct GridPosition {
    x: u32,
    y: u32,
}

struct MyCoord {
    x: f32,
    y: f32,
}

fn main() {
    let res_settings = ResolutionSettings {
        large: Vec2::new(1080.0, 1080.0),
        medium: Vec2::new(600.0, 600.0),
        small: Vec2::new(360.0, 360.0),
    };

    let initial_size = res_settings.small;

    let window_plugin_with_initial_settings = WindowPlugin {
        primary_window: Some(Window {
            title: "Snek".into(),
            resolution: WindowResolution::new(initial_size.x as u32, initial_size.y as u32),
            resizable: false,
            ..default()
        }),
        ..default()
    };

    let grid = GridSettings::new(initial_size.x);

    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin_with_initial_settings))
        .insert_resource(res_settings)
        .insert_resource(grid)
        .add_systems(Startup, setup_window)
        .add_systems(Update, (toggle_resolution, draw_grid, draw_snake).chain())
        .run();
}

fn draw_snake(mut gizmos: Gizmos, window: Single<&mut Window>, grid: Res<GridSettings>) {
    fn draw_snake_square(
        gizmos: &mut Gizmos,
        window: &Single<&mut Window>,
        grid: &GridSettings,
        grid_pos: GridPosition,
    ) {
        let snake_color = Color::srgb(255.0, 251.0, 0.0);
        let coord = grid_to_myworld(&grid, &grid_pos);
        let size = grid.block_size - 5.0;

        let x1: f32 = coord.x - size / 2.0;
        let y1: f32 = coord.y - size / 2.0;
        let x2: f32 = coord.x + size / 2.0;
        let y2: f32 = coord.y + size / 2.0;

        gizmos.line_2d(
            myworld_to_screen(&window, Vec2::new(x1, y1)),
            myworld_to_screen(&window, Vec2::new(x2, y1)),
            snake_color,
        );
        gizmos.line_2d(
            myworld_to_screen(&window, Vec2::new(x2, y1)),
            myworld_to_screen(&window, Vec2::new(x2, y2)),
            snake_color,
        );
        gizmos.line_2d(
            myworld_to_screen(&window, Vec2::new(x2, y2)),
            myworld_to_screen(&window, Vec2::new(x1, y2)),
            snake_color,
        );
        gizmos.line_2d(
            myworld_to_screen(&window, Vec2::new(x1, y2)),
            myworld_to_screen(&window, Vec2::new(x1, y1)),
            snake_color,
        );
    }
    let pos = GridPosition { x: 1, y: 1 };
    draw_snake_square(&mut gizmos, &window, &grid, pos);
}

fn draw_grid(mut gizmos: Gizmos, window: Single<&mut Window>, grid: Res<GridSettings>) {
    fn draw_grid_square(
        gizmos: &mut Gizmos,
        window: &Single<&mut Window>,
        grid: &GridSettings,
        grid_pos: GridPosition,
    ) {
        let coord = grid_to_myworld(&grid, &grid_pos);

        let x1: f32 = coord.x - grid.block_size / 2.0;
        let y1: f32 = coord.y - grid.block_size / 2.0;
        let x2: f32 = coord.x + grid.block_size / 2.0;
        let y2: f32 = coord.y + grid.block_size / 2.0;

        gizmos.line_2d(
            myworld_to_screen(&window, Vec2::new(x1, y1)),
            myworld_to_screen(&window, Vec2::new(x2, y1)),
            Color::srgb(1.0, 0.2, 0.2),
        );
        gizmos.line_2d(
            myworld_to_screen(&window, Vec2::new(x2, y1)),
            myworld_to_screen(&window, Vec2::new(x2, y2)),
            Color::srgb(1.0, 0.2, 0.2),
        );
        gizmos.line_2d(
            myworld_to_screen(&window, Vec2::new(x2, y2)),
            myworld_to_screen(&window, Vec2::new(x1, y2)),
            Color::srgb(1.0, 0.2, 0.2),
        );
        gizmos.line_2d(
            myworld_to_screen(&window, Vec2::new(x1, y2)),
            myworld_to_screen(&window, Vec2::new(x1, y1)),
            Color::srgb(1.0, 0.2, 0.2),
        );
    }

    for i in 0..grid.size {
        for j in 0..grid.size {
            let pos = GridPosition { x: i, y: j };
            draw_grid_square(&mut gizmos, &window, &grid, pos);
        }
    }
}

fn grid_to_myworld(grid: &GridSettings, grid_pos: &GridPosition) -> MyCoord {
    let xcount = grid_pos.x as f32;
    let ycount = grid_pos.y as f32;

    let x_new = grid.offset + (grid.block_size * (xcount + 0.5));
    let y_new = grid.offset + (grid.block_size * (ycount + 0.5));

    MyCoord { x: x_new, y: y_new }
}

fn myworld_to_screen(window: &Single<&mut Window>, coord: Vec2) -> Vec2 {
    let width = window.width();
    let height = window.height();
    // let width = 0.0;
    // let height = 0.0;

    Vec2 {
        x: coord.x - (width / 2.0),
        y: coord.y - (height / 2.0),
    }
}

fn setup_window(mut commands: Commands, mut window: Single<&mut Window>) {
    window.resizable = false;
    commands.spawn(Camera2d);
}

fn toggle_resolution(
    keys: Res<ButtonInput<KeyCode>>,
    mut window: Single<&mut Window>,
    resolution: Res<ResolutionSettings>,
    mut grid: ResMut<GridSettings>,
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

    grid.block_size = (window.height() - (grid.offset * 2.0)) / (grid.size as f32);
}
