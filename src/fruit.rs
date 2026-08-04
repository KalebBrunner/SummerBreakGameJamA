use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Fruit {}

#[derive(Resource, Default)]
pub struct FruitConfig {
    fruit_limit: u32,
}

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FruitConfig>();
    }
}
