use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemStatus {
    Idle, Pumping, Preheat, HvOn, Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Telemetry {
    pub timestamp: u64,
    pub voltage_kv: f32,
    pub pressure_mbar: f32,
    pub anode_temp_c: f32,
    pub status: SystemStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    StartPwm, StopSafe, Calibrate, SetSamplingFreqHz(u32), ResetState,
}