use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Fruit {}

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {}
}
