#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::spi::{Config, Spi};
use embassy_rp_skeleton::patterns;
use embassy_time::{Duration, Timer};
use smart_leds::SmartLedsWrite;
use ws2812_blocking_spi::Ws2812BlockingWriter;
use {defmt_rtt as _, panic_probe as _};

const SLEEP: u64 = 25;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Log
    info!("Starting...");

    // Initialise Peripherals
    let p = embassy_rp::init(Default::default());

    // Create NeoPixel
    let mut pixels =
        Ws2812BlockingWriter::new(Spi::new_blocking_txonly(p.SPI1, p.PIN_10, p.PIN_11, {
            let mut config = Config::default();
            config.frequency = 3_000_000;
            config
        }));

    // Initialise Patterns
    let mut p1 = patterns::pattern1::<13>(16);
    let mut p2 = patterns::pattern2::<13>(64);
    let mut p3 = patterns::pattern3::<13>(32);
    let mut p4 = patterns::pattern4v1::<13>(32);

    // Rainbows
    loop {
        // Log
        info!("Pattern 1");

        // Pattern 1
        for colours in p1.by_ref().take(192) {
            // Write
            pixels.write(colours).unwrap();

            // Sleep
            Timer::after(Duration::from_millis(SLEEP)).await;
        }

        // Log
        info!("Pattern 2");

        // Pattern 2
        for colours in p2.by_ref().take(224) {
            // Write
            pixels.write(colours).unwrap();

            // Sleep
            Timer::after(Duration::from_millis(SLEEP)).await;
        }

        // Log
        info!("Pattern 3");

        // Pattern 3
        for colours in p3.by_ref().take(224) {
            // Write
            pixels.write(colours).unwrap();

            // Sleep
            Timer::after(Duration::from_millis(SLEEP)).await;
        }

        // Log
        info!("Pattern 4");

        // Pattern 4
        for colours in p4.by_ref().take(192) {
            // Write
            pixels.write(colours).unwrap();

            // Sleep
            Timer::after(Duration::from_millis(SLEEP)).await;
        }
    }
}
