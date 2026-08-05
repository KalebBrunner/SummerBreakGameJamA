use bevy::prelude::*;
use bevy_rand::{global::GlobalRng, plugin::EntropyPlugin, prelude::WyRand};
use rand_core::Rng;

use crate::{arena::ArenaBounds, grid_location::GridLocation};

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

fn spawn_fruits(
    //
    mut commands: Commands,
    fruit_config: Res<FruitConfig>,
    fruits: Query<&Fruit>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    arena: Single<&ArenaBounds>,
) {
    let fruit_count = fruits.count();
    let fruit_limit = fruit_config.fruit_limit as usize;

    for _ in fruit_count..fruit_limit {
        let x = ((rng.next_u32() >> 1) as i32) % arena.size.x;
        let y = ((rng.next_u32() >> 1) as i32) % arena.size.y;
        let location = IVec2 { x, y };
        let grid_location = GridLocation { location };

        commands.spawn((Fruit {}, grid_location));
    }
}

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_resource::<FruitStats>();

        if !app.is_plugin_added::<EntropyPlugin<WyRand>>() {
            app.add_plugins(EntropyPlugin::<WyRand>::default());
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::{ecs::system::RunSystemOnce, prelude::*};
    use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};

    use crate::{
        arena::{ArenaBounds, spawn_arena},
        fruit::{Fruit, FruitConfig, FruitPlugin, spawn_fruits},
        grid_location::GridLocation,
    };

    fn assert_fruit_count(
        //
        app: &mut App,
        expected_fruit_count: u32,
    ) {
        let mut fruit_count = app //
            .world_mut()
            .query::<&Fruit>()
            .query(app.world())
            .count() as u32;

        assert_eq!(expected_fruit_count, fruit_count);
    }

    #[test]
    fn spawn_up_to_fruit_limit() {
        let fruit_limit = 3;
        let fruit_config = FruitConfig { fruit_limit };

        let mut app = App::new();
        app //
            .add_plugins(FruitPlugin)
            .insert_resource(fruit_config)
            .add_systems(Update, spawn_fruits)
            .add_systems(Startup, spawn_arena::<5, 5>);

        app.update();
        app.update();

        assert_fruit_count(&mut app, fruit_limit);
    }

    fn remove_one_fruit(
        //
        mut commands: Commands,
        fruits: Query<Entity, With<Fruit>>,
    ) {
        let fruit = fruits.iter().next().expect("no fruits in existence");

        commands.entity(fruit).despawn();
    }

    #[test]
    fn spawn_fruits_as_removed() {
        let fruit_limit = 2;
        let fruit_config = FruitConfig { fruit_limit };

        let mut app = App::new();
        app //
            .add_plugins(FruitPlugin)
            .insert_resource(fruit_config)
            .add_systems(Update, spawn_fruits);
        app.world_mut().run_system_once(spawn_arena::<5, 5>);

        app.update();

        assert_fruit_count(&mut app, fruit_limit);

        app.world_mut().run_system_once(remove_one_fruit);

        app.update();

        assert_fruit_count(&mut app, fruit_limit);

        app.update();

        assert_fruit_count(&mut app, fruit_limit);

        app.update();

        assert_fruit_count(&mut app, fruit_limit);

        app.world_mut().run_system_once(remove_one_fruit);

        app.update();

        assert_fruit_count(&mut app, fruit_limit);
    }

    #[test]
    fn spawn_fruits_at_random_locations() {
        let fruit_limit = 1;
        let fruit_config = FruitConfig { fruit_limit };

        let mut app = App::new();
        app //
            .add_plugins(FruitPlugin)
            .insert_resource(fruit_config)
            .add_systems(Update, spawn_fruits)
            .add_systems(Startup, spawn_arena::<5, 5>);

        let mut fruits = Vec::new();

        for _ in 0..10 {
            app.update();

            let world = app.world_mut();
            let fruit = world
                .query_filtered::<&GridLocation, With<Fruit>>()
                .iter(world)
                .next()
                .expect("fruit was not created");

            fruits.push(fruit.location);

            app.world_mut().run_system_once(remove_one_fruit);
        }

        let mut different_locations = false;
        'outer: for fruit_a_pos in fruits.iter() {
            for fruit_b_pos in fruits.iter() {
                if fruit_a_pos != fruit_b_pos {
                    different_locations = true;

                    break 'outer;
                }
            }
        }

        assert!(different_locations);
    }

    #[test]
    fn spawn_fruits_inside_arena() {
        let fruit_limit = 100;
        let fruit_config = FruitConfig { fruit_limit };

        const X: i32 = 5;
        const Y: i32 = 5;

        let mut app = App::new();
        app //
            .add_plugins(FruitPlugin)
            .insert_resource(fruit_config)
            .add_systems(Update, spawn_fruits)
            .add_systems(Startup, spawn_arena::<X, Y>);

        app.update();

        let world = app.world_mut();
        let mut fruits = world.query_filtered::<&GridLocation, With<Fruit>>();

        let mut none_outside_arena = true;
        for fruit_pos in fruits.iter(world) {
            assert!(fruit_pos.location.x >= 0);
            assert!(fruit_pos.location.y >= 0);
            assert!(fruit_pos.location.x < X);
            assert!(fruit_pos.location.y < Y);
        }
    }
}
