use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::uart::{UartDriver, config::Config};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use shared::{Command, Telemetry, SystemStatus};

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities. This is so we can log
    esp_idf_svc::log::EspLogger::initialize_default();

    // Here we initialize the peripherals
    let peripherals = Peripherals::take().unwrap();
    
    // Comunication channel between acquisition and transport threads
    let (tx_telemetry, rx_telemetry) = mpsc::channel::<Telemetry>();

    // ---------------------------------------------------------
    // Thread 0: Data acquisition
    // ---------------------------------------------------------
    let tx_clone = tx_telemetry.clone();
    // Note: ESP-IDF std::thread wrapper asigns tasks to free cores in a round-robin way. 
    // For pinning to a core,we need to use esp_idf_hal::thread::EspThread, maybe we need it, we have to try. 
    thread::spawn(move || {
        loop {
            let data = Telemetry {
                timestamp: 0, 
                voltage_kv: 0.0,
                pressure_mbar: 1013.0,
                anode_temp_c: 25.0,
                status: SystemStatus::Idle,
            };
            // TODO: Here we interrogate the buses one by one, then we pack the data into the Telemetry struct and send it to the transport thread.
            // Send to memory queue for transport thread to pick up 
            let _ = tx_clone.send(data);

            // Frecuencia teórica (1 kHz , ajustable)
            thread::sleep(Duration::from_millis(1)); 
        }
    });
    // ---------------------------------------------------------
    // Thread 1: Data transport
    // ---------------------------------------------------------
    thread::spawn(move || {
        // UART is the interface we use to comunicate with the host, here we initialize it.
        // We are configuring it with a baudrate of 115200, which is a common choice for telemetry data, is important in the receptor to configure to this frequency.
        //This is an example, we should change pins and uart number so it works.
        let mut uart = UartDriver::new(
            peripherals.uart1,
            peripherals.pins.gpio1, // TX
            peripherals.pins.gpio3, // RX
            Option::<esp_idf_svc::hal::gpio::AnyIOPin>::None,
            Option::<esp_idf_svc::hal::gpio::AnyIOPin>::None,
            &Config::default().baudrate(115200),
        ).unwrap();
        let mut encode_buf = [0u8; 64];
        loop {
            //TODO: 
            // Empty queue handling: if there is telemetry data, we serialize and transmit it, otherwise we just wait for the next iteration.
            // We use postcard 
            //Correct way to do this is using rx_telemetry.recv() and hadle the error.
            //Then send it using uart.write()
        }
    });

    // When initializations are done, we put main thread to sleep
    loop {
        thread::sleep(Duration::from_secs(10));
    }
}