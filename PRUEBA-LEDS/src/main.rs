use esp_idf_sys as _; // Necesario para los parches de runtime

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    // Parches necesarios para que esp-idf-sys enlace correctamente
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_svc::sys::link_patches();
    
    let dp = Peripherals::take().unwrap();
    
    // Configuración del botón
    let mut button = PinDriver::input(dp.pins.gpio21)?;
    button.set_pull(Pull::Up)?;

    // Configuración de los 10 LEDs usando 'downgrade_output' para homogeneizar tipos
    let mut leds = [
        PinDriver::output(dp.pins.gpio22.downgrade_output())?,
        PinDriver::output(dp.pins.gpio2.downgrade_output())?,
        PinDriver::output(dp.pins.gpio19.downgrade_output())?,
        PinDriver::output(dp.pins.gpio18.downgrade_output())?,
        PinDriver::output(dp.pins.gpio4.downgrade_output())?,
        PinDriver::output(dp.pins.gpio5.downgrade_output())?,
        PinDriver::output(dp.pins.gpio12.downgrade_output())?,
        PinDriver::output(dp.pins.gpio13.downgrade_output())?,
        PinDriver::output(dp.pins.gpio14.downgrade_output())?,
        PinDriver::output(dp.pins.gpio15.downgrade_output())?,
    ];

    let mut blinkdelay = 200_u32;

    loop {
        for led in &mut leds {
            led.set_high()?;
            
            // Verificamos el botón y actualizamos el delay
            blinkdelay = check_button(&button, blinkdelay);
            
            FreeRtos::delay_ms(blinkdelay);
            led.set_low()?;
            FreeRtos::delay_ms(100_u32);
        }
    }
}

// La función debe ir FUERA del main
fn check_button(but: &PinDriver<'_, Gpio21, Input>, current_delay: u32) -> u32 {
    if but.is_low() {
        let new_delay = if current_delay <= 50 {
            200 // Reset a valor inicial
        } else {
            current_delay - 50 // Aumentar velocidad
        };
        log::info!("Botón presionado. Delay:{}ms",new_delay);
        esp_idf_hal::delay::FreeRtos::delay_ms(150);
        new_delay
    } else {
        current_delay
    }
}