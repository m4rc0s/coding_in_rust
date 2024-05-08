use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Snake;

fn main() {
    App::new()
        .add_systems(Startup, add_snake)
        .add_systems(Update, hello_snake)
        .run();
}

fn hello_snake() {
    println!("Hello from bevy!");
}

fn add_snake(mut commands: Commands) {
    commands.spawn((Snake, Position { y: 0.0, x: 0.0 }));
}
