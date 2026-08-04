use bevy::{ecs::component::Component, math::IVec2};

#[derive(Debug, Component)]
pub struct ArenaBounds {
    size: IVec2,
}
