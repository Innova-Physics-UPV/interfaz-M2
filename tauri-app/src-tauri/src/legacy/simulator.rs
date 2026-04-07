use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::mpsc::Sender;
use rand::Rng; // Importante: Necesitas 'rand' en Cargo.toml

use crate::telemetry::{Telemetry, SystemStatus};
use crate::codec::{Codec, Protocol};

pub struct Simulator {
    tx: Sender<Vec<u8>>,
}

impl Simulator {
    pub fn new(tx: Sender<Vec<u8>>) -> Self {
        Self { tx }
    }

    pub fn run(&self) {
        let mut state = Telemetry::new();
        state.pressure_mbar = 1013.25; 
        loop {
            self.update_physics(&mut state);
            let raw_bytes = Codec::encode(&state, Protocol::Postcard);
            let noisy_bytes = self.inject_noise(raw_bytes);

            // ENVÍO
            if !noisy_bytes.is_empty() {
                if let Err(e) = self.tx.send(noisy_bytes) {
                    eprintln!("[SIMULADOR] Error fatal: Canal cerrado. Apagando hilo. ({})", e);
                    break;
                }
            }
            // ENVIO A DESTIEMPO
            let mut rng = rand::thread_rng();
            let jitter_ms = rng.gen_range(200..350);
            thread::sleep(Duration::from_millis(jitter_ms));
        }
    }

    fn update_physics(&self, state: &mut Telemetry) {
        let mut rng = rand::thread_rng();
        state.timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // Simula el ciclo de encendido
        match state.status {
            SystemStatus::Idle => {
            
                state.pressure_mbar = 1013.25 + rng.gen_range(-0.5..0.5); //ruido 
                state.voltage_kv = 0.0;
                state.heater_current_a = 0.0;
                
                // Bbombeo aleatorio
                if rng.gen_bool(0.01) { 
                    println!("[FISICA] Iniciando Bomba de Vacío...");
                    state.status = SystemStatus::Pumping; 
                }
            },
            SystemStatus::Pumping => {
                // Descenso logarítmico de la presión
                state.pressure_mbar *= 0.95; 
                // Si llegamos a vacío profundo (< 0.01 mBar), pasamos a precalentar
                if state.pressure_mbar < 0.01 {
                    println!("[FISICA] Vacío alcanzado ({:.4} mBar). Precalentando...", state.pressure_mbar);
                    state.status = SystemStatus::Preheat;
                }
            },
            SystemStatus::Preheat => {
                // Subir corriente del filamento suavemente (Inercia térmica)
                if state.heater_current_a < 0.3 {
                    state.heater_current_a += 0.01;
                }
                // Simular pequeña fluctuación
                state.heater_current_a += rng.gen_range(-0.001..0.001);
                // Tras un tiempo (simulado por probabilidad), encender HV
                if state.heater_current_a >= 0.28 && rng.gen_bool(0.02) {
                    println!("[FISICA] Cátodo caliente. Encendiendo Alta Tensión...");
                    state.status = SystemStatus::HvOn;
                }
            },
            SystemStatus::HvOn => {
                // Subir voltaje hasta 15kV
                if state.voltage_kv < 15.0 {
                    state.voltage_kv += 0.5;
                }
                // Ruido en la alta tensión (común en flybacks)
                state.voltage_kv += rng.gen_range(-0.2..0.2);
                // 1% de probabilidad de que se abra sistema de seguridad (Interlock)
                if rng.gen_bool(0.01) {
                    println!("[FISICA] ¡ALERTA! Interlock abierto durante operación.");
                    state.interlock_engaged = false;
                    state.status = SystemStatus::Error;
                }
            },
            SystemStatus::Error => {
                state.voltage_kv = 0.0; // Corte inmediato
                state.interlock_engaged = false;
                
                // Recuperación automática tras un rato
                if rng.gen_bool(0.05) {
                    println!("[FISICA] Sistema reiniciado. Interlock cerrado.");
                    state.interlock_engaged = true;
                    state.status = SystemStatus::Idle;
                    state.pressure_mbar = 1013.25; // Se perdió el vacío al abrir
                }
            }
        }
    }
    fn inject_noise(&self, mut data: Vec<u8>) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let chance = rng.gen_range(0..100);

        // Paquete Perfecto (85% de las veces)
        if chance < 85 {
            //Comentar para ocultar el flujo de datos
            println!("[DATOS]{:?}",data);
            return data;
        }
        // Añadimos bytes aleatorios al final o al principio (Ruido EMI)
        if chance < 95 {
            println!("[CAOS] Inyectando ruido EMI en la línea...");
            let noise: Vec<u8> = (0..5).map(|_| rng.gen()).collect();
            data.extend(noise); // El buffer y COBS deberían manejar esto, pero a veces fallará
            return data;
        }
        // (Bit Flip) Checksum falla
        if chance < 99 {
            if let Some(byte) = data.get_mut(2) { // Corromper el 3er byte
                println!("[CAOS] Bit flip! Byte corrompido.");
                *byte ^= 0xFF; 
            }
            return data;
        }
        //Pérdida total
        println!("[CAOS] Paquete perdido en el vacío...");
        Vec::new() // Devolvemos vector vacío
    }
}