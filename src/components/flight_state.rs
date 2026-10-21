//! Derived flight state and atmosphere state components.

use crate::_bevy::*;
use avian3d::math::Real;
use serde::{Deserialize, Serialize};

/// Derived aerodynamic state quantities. Written each frame by
/// `update_flight_state`. Read-only for consumers (HUD, autopilot, debug).
///
/// Lives on the **aircraft root entity**.
#[derive(Component, Reflect, Serialize, Deserialize, Clone, Debug, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct FlightState {
    /// Angle of attack, alpha (rad). Positive = nose above relative wind.
    pub alpha_rad: Real,
    /// Sideslip angle, beta (rad). Positive = nose left of relative wind.
    pub beta_rad: Real,
    /// True airspeed V (m/s).
    pub airspeed_ms: Real,
    /// Mach number = airspeed / speed of sound (dimensionless).
    pub mach: Real,
    /// Dynamic pressure, q-bar = half * density * airspeed^2 (Pa).
    pub dynamic_pressure_pa: Real,
    /// Geometric altitude above sea level (m).
    pub altitude_m: Real,
    /// Roll rate p in body frame (rad/s). Written by `update_flight_state`.
    pub p_rads: Real,
    /// Pitch rate q in body frame (rad/s). Written by `update_flight_state`.
    pub q_rads: Real,
    /// Yaw rate r in body frame (rad/s). Written by `update_flight_state`.
    pub r_rads: Real,
}

/// ISA atmosphere conditions at this entity's altitude.
/// Written each frame by `update_atmosphere`.
///
/// Lives on the **aircraft root entity**.
#[derive(Component, Reflect, Serialize, Deserialize, Clone, Debug, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct AtmosphereState {
    /// Air density ρ (kg/m³).
    pub density_kgm3: Real,
    /// Static pressure p (Pa).
    pub pressure_pa: Real,
    /// Temperature T (K).
    pub temperature_k: Real,
    /// Speed of sound a (m/s).
    pub speed_of_sound_ms: Real,
}
