//! Bevy systems for the atmosphere subsystem.
//!
//! Bridges the pure ISA functions in [`super::isa`] to ECS data on aircraft
//! entities. Currently exposes a single system, [`update_atmosphere`], which
//! samples [`AtmosphereState`] from the world-space altitude.

use super::isa::atmosphere_at;
use crate::_bevy::*;
use crate::components::AtmosphereState;
use avian3d::{math::Scalar, prelude::Position};

const EARTH_RADIUS: Scalar = 6_360_000.0;

/// Updates [`AtmosphereState`] on each aircraft from its world-space altitude.
///
/// Reads `Position.length() - EARTH_RADIUS` as geometric altitude above sea level.
#[allow(clippy::unnecessary_cast)]
pub fn update_atmosphere(mut query: Query<(&Position, &mut AtmosphereState)>) {
    for (position, mut atm) in &mut query {
        let altitude_m = position.length() as Scalar - EARTH_RADIUS;
        *atm = atmosphere_at(altitude_m);
    }
}
