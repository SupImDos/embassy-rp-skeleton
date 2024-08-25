#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialise Peripherals
    let p = embassy_rp::init(Default::default());

    // Create LED
    let mut led = Output::new(p.PIN_25, Level::Low);

    // Loop
    loop {
        // Log
        info!("LED On!");

        // Turn LED On
        led.set_high();

        // Wait 100ms
        Timer::after(Duration::from_millis(100)).await;

        // Log
        info!("LED Off!");

        // Turn Led Off
        led.set_low();

        // Wait 100ms
        Timer::after(Duration::from_millis(100)).await;
    }
}
