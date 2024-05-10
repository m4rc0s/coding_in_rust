use std::time::Duration;

use bevy::app::App;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::DefaultPlugins;
use rand::Rng;

const TIME_STEP: f64 = 1. / 9.;
const WINDOW_WIDTH: f32 = 700.;
const WINDOW_HEIGHT: f32 = 700.;
const GRID_SIZE: i32 = 28;

#[derive(Resource, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Resource)]
struct Orientation {
    direction: Direction,
}

#[derive(Component, Debug)]
struct SnakeHead;

#[derive(Component, Debug)]
struct SnakeSegment;

#[derive(Component, Debug)]
struct SnakeTail;

#[derive(Component, Debug)]
struct Apple;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake Game".to_string(),
                transparent: true,
                resolution: (WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32).into(),
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Orientation {
            direction: Direction::Right,
        })
        .add_systems(Startup, (camera, snake, fruit))
        .add_systems(
            Update,
            (
                direction,
                (
                    (movement).after(direction),
                    (position_translation).after(movement),
                    (eating.before(position_translation)),
                )
                    .run_if(on_timer(Duration::from_secs_f32(TIME_STEP as f32))),
            ),
        )        
        .insert_resource(ClearColor(Color::rgba(0.0, 0.0, 0.0, 0.9)))
        .run();
}

fn camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn fruit(mut commands: Commands) {
    let color = Color::YELLOW_GREEN;
    let mut rng = rand::thread_rng();

    commands.spawn((
        Apple,
        SpriteBundle {
            sprite: Sprite { color, ..default() },
            transform: Transform {
                translation: Vec3::new(
                    rng.gen_range(-145.0..145.0),
                    rng.gen_range(-300.0..300.0),
                    0.0,
                ),
                scale: Vec3::splat(32.0),
                ..default()
            },
            ..default()
        },
    ));
}

fn snake(mut commands: Commands) {
    commands.spawn((
        SnakeHead,
        Position { x: 1, y: 1 },
        SpriteBundle {
            sprite: Sprite {
                color: Color::GOLD,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1.0, 1.0, 0.0),
                scale: Vec3::new(32.0, 32.0, 0.0),
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn((
        SnakeSegment,
        Position { x: -1, y: 1 },
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1.0, 1.0, 0.0),
                scale: Vec3::new(32.0, 32.0, 0.0),
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn((
        SnakeTail,
        Position { x: -2, y: 1 },
        SpriteBundle {
            sprite: Sprite {
                color: Color::LIME_GREEN,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1.0, 1.0, 0.0),
                scale: Vec3::new(32.0, 32.0, 0.0),
                ..default()
            },
            ..default()
        },
    ));
}

fn direction(keyboard_input: Res<ButtonInput<KeyCode>>, mut orientation: ResMut<Orientation>) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) && orientation.direction != Direction::Down {
        orientation.direction = Direction::Up;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowDown) && orientation.direction != Direction::Up {
        orientation.direction = Direction::Down;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowRight) && orientation.direction != Direction::Left
    {
        orientation.direction = Direction::Right;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowLeft) && orientation.direction != Direction::Right
    {
        orientation.direction = Direction::Left;
    }
}

fn movement(
    orientation: Res<Orientation>,
    mut head: Query<
        (&mut SnakeHead, &mut Position),
        (With<SnakeHead>, Without<SnakeSegment>, Without<SnakeTail>),
    >,
    mut segments: Query<
        (&mut SnakeSegment, &mut Position),
        (With<SnakeSegment>, Without<SnakeHead>, Without<SnakeTail>),
    >,
    mut tail: Query<
        (&mut SnakeTail, &mut Position),
        (With<SnakeTail>, Without<SnakeSegment>, Without<SnakeHead>),
    >,
) {
    let (head, mut head_position) = head.iter_mut().next().unwrap();
    let (tail, mut tail_position) = tail.iter_mut().next().unwrap();

    let mut prev_pos = head_position.clone();

    match orientation.direction {
        Direction::Up => {
            head_position.y += 1;
        }
        Direction::Down => {
            head_position.y -= 1;
        }
        Direction::Right => {
            head_position.x += 1;
        }
        Direction::Left => {
            head_position.x -= 1;
        }
    }

    for (_, mut position) in segments.iter_mut() {
        let prev_seg_pos = position.clone();

        position.x = prev_pos.x;
        position.y = prev_pos.y;

        prev_pos = prev_seg_pos;
    }

    tail_position.x = prev_pos.x;
    tail_position.y = prev_pos.y;
}

fn eating(
    mut commands: Commands,
    seg_trans_query: Query<&Transform, With<SnakeHead>>,
    fruit_trans_query: Query<(Entity, &Transform), With<Apple>>,
) {
    const COLLISION_SPREAD: f32 = 16.0;
    let segment_transform = seg_trans_query.iter().next().unwrap();
    let (fruit, fruit_transform) = fruit_trans_query.iter().next().unwrap();
    let color = Color::YELLOW_GREEN;
    let mut rng = rand::thread_rng();

    if (segment_transform.translation.x <= fruit_transform.translation.x + COLLISION_SPREAD
        && segment_transform.translation.x >= fruit_transform.translation.x - COLLISION_SPREAD)
        && (segment_transform.translation.y <= fruit_transform.translation.y + COLLISION_SPREAD
            && segment_transform.translation.y >= fruit_transform.translation.y - COLLISION_SPREAD)
    {
        println!(
            "well colided with snake pos ({:?}, {:?}) and fruit pos ({:?}, {:?})",
            segment_transform.translation.x,
            segment_transform.translation.y,
            fruit_transform.translation.x,
            fruit_transform.translation.y
        );

        commands.entity(fruit).despawn();
        commands.spawn((
            Apple,
            SpriteBundle {
                sprite: Sprite { color, ..default() },
                transform: Transform {
                    translation: Vec3::new(
                        rng.gen_range(-345.0..345.0),
                        rng.gen_range(-300.0..300.0),
                        0.0,
                    ),
                    scale: Vec3::new(32.0, 32.0, 0.0),
                    ..default()
                },
                ..default()
            },
        ));
        commands.spawn((
            SnakeSegment,
            Position {
                x: segment_transform.translation.x as i32,
                y: segment_transform.translation.y as i32,
            },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(-1.0, -1.0, 0.0),
                    scale: Vec3::new(32.0, 32.0, 0.0),
                    ..default()
                },
                ..default()
            },
        ));
    }
}

fn position_translation(mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, WINDOW_WIDTH as f32, GRID_SIZE as f32),
            convert(pos.y as f32, WINDOW_HEIGHT as f32, GRID_SIZE as f32),
            0.0,
        );
    }
}
