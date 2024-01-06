use crate::components::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub damage_value: f32,
    pub damage_type: DamageType,
}