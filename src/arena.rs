use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct ArenaBounds {
    pub size: IVec2,
}

pub fn spawn_arena<const X: i32, const Y: i32>(mut commands: Commands) {
    let size = IVec2::new(X, Y);
    let arena_bounds = ArenaBounds { size };

    commands.spawn(arena_bounds);
}
