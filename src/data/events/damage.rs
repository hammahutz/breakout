use bevy::prelude::*;

use crate::data::components::{DamageComponent, HealthComponent};

#[derive(Event)]
pub struct DamageEvent {
    pub damage: DamageComponent,
    pub health: HealthComponent,
}
