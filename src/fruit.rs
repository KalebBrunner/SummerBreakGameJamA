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

fn spawn_fruits(
    //
    mut commands: Commands,
    fruit_config: Res<FruitConfig>,
    fruits: Query<&Fruit>,
) {
    let fruit_count = fruits.count();
    let fruit_limit = fruit_config.fruit_limit as usize;

    for _ in fruit_count..fruit_limit {
        commands.spawn((Fruit {}));
    }
}

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        let fruit_config = FruitConfig { fruit_limit: 3 };

        app //
            .insert_resource(fruit_config)
            .init_resource::<FruitStats>();
    }
}

#[cfg(test)]
mod tests {
    use bevy::{ecs::system::RunSystemOnce, prelude::*};

    use crate::fruit::{Fruit, FruitConfig, spawn_fruits};

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
            .insert_resource(fruit_config)
            .add_systems(Update, spawn_fruits);

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
            .insert_resource(fruit_config)
            .add_systems(Update, spawn_fruits);

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
}
