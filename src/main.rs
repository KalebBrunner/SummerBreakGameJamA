#![allow(unused)]

mod arena;
mod fruit;
mod grid_location;

use bevy::{
    app::*,
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query},
    },
};
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};

use crate::fruit::FruitPlugin;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn hello_world() {
    println!("Hello world!");
}

fn main() {
    println!("Hello, world!");

    App::new()
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(FruitPlugin)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}
