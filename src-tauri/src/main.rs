// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod telemetry;
mod codec;
mod simulator;
mod ingestion;

use std::sync::mpsc;
use std::thread;
use tauri::Manager;
use crate::codec::{Codec, Protocol};
use crate::simulator::Simulator;
use crate::ingestion::start_ingestion_pipeline;

fn main() {
    println!("Hola, soy Rust y estoy vivo!");
    let (tx, rx) = mpsc::channel::<Vec<u8>>();

    thread::spawn(move || {
        let sim = Simulator::new(tx);
        sim.run();
    });

    tauri::Builder::default()
        .setup(move |app| {
            //start_ingestion_pipeline(app.handle().clone(), rx);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}