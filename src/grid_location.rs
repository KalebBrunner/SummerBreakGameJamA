use bevy::{ecs::component::Component, math::IVec2};

#[derive(Debug, Component, PartialEq, Eq)]
pub struct GridLocation {
    pub location: IVec2,
}
