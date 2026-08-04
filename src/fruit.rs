use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Fruit {}

#[derive(Resource, Default)]
pub struct FruitConfig {
    fruit_limit: u32,
}

#[derive(Resource, Default)]
pub struct FruitStats {
    fruits_eaten: u32,
}

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_resource::<FruitConfig>()
            .init_resource::<FruitStats>();
    }
}
