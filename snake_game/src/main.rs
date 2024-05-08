use bevy::prelude::*;

fn hello_snake() {
    println!("Hello from bevy!");
}

fn main() {
    App::new().add_systems(Update, hello_snake).run();
}
