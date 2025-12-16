use serde::{Deserialize, Serialize};

// Enum para la máquina de estados
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemStatus {
    Idle,
    Pumping,
    Preheat,
    HvOn,
    Error,
}

// El struct principal que viaja por todo el sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Telemetry {
    pub timestamp: u64,
    pub pressure_mbar: f64,
    pub voltage_kv: f64,
    pub heater_current_a: f64,
    pub interlock_engaged: bool,
    pub status: SystemStatus,
}

impl Telemetry {
    // Un constructor vacío o por defecto para facilitar tests
    pub fn new() -> Self {
        Self {
            timestamp: 0,
            pressure_mbar: 1013.0,
            voltage_kv: 0.0,
            heater_current_a: 0.0,
            interlock_engaged: false,
            status: SystemStatus::Idle,
        }
    }
}
