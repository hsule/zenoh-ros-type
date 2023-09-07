use crate::builtin_interfaces::Time;
use crate::service::ServiceHeader;
use crate::std_msgs::Header;
use serde_derive::{Deserialize, Serialize};

pub mod control_mode_command {
    pub const NO_COMMAND: u8 = 0;
    pub const AUTONOMOUS: u8 = 1;
    pub const MANUAL: u8 = 2;
}
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ControlModeCommand {
    pub stamp: Time,
    pub mode: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ControlModeReport {
    pub stamp: Time,
    pub mode: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Engage {
    pub stamp: Time,
    pub enable: bool,
}

pub mod gear_command {
    pub const DRIVE: u8 = 2;
    pub const REVERSE: u8 = 20;
    pub const PARK: u8 = 22;
    pub const LOW: u8 = 23;
}
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct GearCommand {
    pub stamp: Time,
    pub command: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct GearReport {
    pub stamp: Time,
    pub report: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct HandbrakeCommand {
    pub stamp: Time,
    pub active: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct HandbrakeReport {
    pub stamp: Time,
    pub report: bool,
}

pub mod hazard_lights_command {
    pub const NO_COMMAND: u8 = 0;
    pub const DISABLE: u8 = 1;
    pub const ENABLE: u8 = 2;
}
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct HazardLightsCommand {
    pub stamp: Time,
    pub command: u8,
}

pub mod hazard_lights_report {
    pub const DISABLE: u8 = 1;
    pub const ENABLE: u8 = 2;
}
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct HazardLightsReport {
    pub stamp: Time,
    pub report: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct SteeringReport {
    pub stamp: Time,
    pub steering_tire_angle: f32,
}

pub mod turn_indicators_command {
    pub const NO_COMMAND: u8 = 0;
    pub const DISABLE: u8 = 1;
    pub const ENABLE_LEFT: u8 = 2;
    pub const ENABLE_RIGHT: u8 = 3;
}
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct TurnIndicatorsCommand {
    pub stamp: Time,
    pub command: u8,
}

pub mod turn_indicators_report {
    pub const DISABLE: u8 = 1;
    pub const ENABLE_LEFT: u8 = 2;
    pub const ENABLE_RIGHT: u8 = 3;
}
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct TurnIndicatorsReport {
    pub stamp: Time,
    pub report: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct VelocityReport {
    pub header: Header,
    pub longitudinal_velocity: f32,
    pub lateral_velocity: f32,
    pub heading_rate: f32,
}

// -----service-----

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct EngageRequest {
    pub header: ServiceHeader,
    pub mode: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct EngageResponse {
    pub header: ServiceHeader,
    pub code: u32,
    pub message: String,
    pub success: bool,
}
