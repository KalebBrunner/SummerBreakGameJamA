use bevy::{ecs::component::Component, math::IVec2};

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct GridLocation {
    pub location: IVec2,
}
