#![no_main]
#![no_std]

use hal::{
    gpio::Level,
    prelude::{OutputPin, StatefulOutputPin},
};
use my_app as _; // global logger + panicking-behavior + memory layout
use nrf52840_hal as hal;
use rtt_target::{self, rprintln};

const NUM_CYCLES: u32 = 1001337;

#[cortex_m_rt::entry]
fn main() -> ! {
    //defmt::info!("Hello, world!");
    rtt_target::rtt_init_print!();

    if let Some(periph) = hal::pac::Peripherals::take() {
        let mut timer = hal::Timer::new(periph.TIMER0);

        // turn one led on so we know flashing was successful
        let pins = hal::gpio::p0::Parts::new(periph.P0);
        let mut p13 = pins.p0_13.degrade().into_push_pull_output(Level::Low);

        loop {
            //defmt::info!("xoxo");
            rprintln!("xoxo");
            timer.delay(NUM_CYCLES);

            if p13.is_set_low().unwrap() {
                p13.set_high();
            } else {
                p13.set_low();
            }
        }
    }

    my_app::exit()
}
