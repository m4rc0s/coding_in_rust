use bevy::{prelude::*, transform::commands};

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Resource)]
struct Orientation {
    direction: Direction,
    factor: i8,
}

#[derive(Component, Debug)]
struct SnakeSegment;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Orientation {
            direction: Direction::Right,
            factor: 1,
        })
        .add_systems(Startup, (setup_camera, spawn_snake))
        .add_systems(
            Update,
            (move_snake, watch_direction_change, define_snake_direction),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn watch_direction_change(resource: Res<Orientation>) {
    if resource.is_changed() {
        println!(
            "direction changed ({:?}, {:?})",
            resource.direction, resource.factor
        );
    }
}

fn spawn_snake(mut commands: Commands) {
    let color = Color::rgb(0.0, 0.7, 0.0);

    commands.spawn((
        Position { x: 0, y: 0 },
        SnakeSegment,
        SpriteBundle {
            sprite: Sprite { color, ..default() },
            transform: Transform {
                translation: Vec3::new(1.0, 1.0, 0.0),
                scale: Vec3::splat(16.0),
                ..default()
            },
            ..default()
        },
    ));
}

fn define_snake_direction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut orientation: ResMut<Orientation>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        orientation.direction = Direction::Up;
        orientation.factor = 1;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        orientation.direction = Direction::Down;
        orientation.factor = -1;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        orientation.direction = Direction::Right;
        orientation.factor = 1;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        orientation.direction = Direction::Left;
        orientation.factor = -1;
    }
}

fn move_snake(
    orientation: Res<Orientation>,
    mut segment_transform_query: Query<&mut Transform, With<SnakeSegment>>,
) {
    let mut transform = segment_transform_query.iter_mut().next().unwrap();

    match orientation.direction {
        Direction::Up => {
            transform.translation =
                Vec3::new(transform.translation.x, transform.translation.y + 1.0, 0.0);
        }
        Direction::Down => {
            transform.translation =
                Vec3::new(transform.translation.x, transform.translation.y - 1.0, 0.0);
        }
        Direction::Right => {
            transform.translation =
                Vec3::new(transform.translation.x + 1.0, transform.translation.y, 0.0);
        }
        Direction::Left => {
            transform.translation =
                Vec3::new(transform.translation.x - 1.0, transform.translation.y, 0.0);
        }
    }
}
