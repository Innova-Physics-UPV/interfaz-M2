use std::thread;
use std::sync::mpsc;
use tauri::Manager;
use tauri::Emitter;
// Importamos las cosas que definiste en codec.rs
use crate::codec::{Codec, Protocol, SerialBuffer};

pub fn start_ingestion_pipeline(app_handle: tauri::AppHandle, rx: mpsc::Receiver<Vec<u8>>) {
    thread::spawn(move || {
        let mut buffer = crate::codec::SerialBuffer::new();

        // El bucle limpio
        for raw_chunk in rx {
            //Ingesta
            buffer.push_bytes(&raw_chunk);
            // Procesamiento (mientras haya paquetes completos en el buffer)
            while let Some(frame) = buffer.try_pop_frame() {
                match Codec::decode(&frame, Protocol::Postcard) {
                    Ok(telemetry) => {
                        // El _  ignora errores si la ventana está cerrada
                        let _ = app_handle.emit("telemetry-update", telemetry);
                    }
                    Err(e) => {
                        eprintln!("Paquete corrupto descartado: {}", e);
                    }
                }
            }
        }
    });
}