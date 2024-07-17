use crate::game::prelude::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct DamageEvent {
    pub damage: DamageComponent,
    pub health: HealthComponent,
}
