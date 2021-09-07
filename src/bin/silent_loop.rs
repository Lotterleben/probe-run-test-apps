#![no_main]
#![no_std]

use my_app as _; // global logger + panicking-behavior + memory layout

// does nothing (used to test if ctrl+c termionations are recognized properly)
#[cortex_m_rt::entry]
fn main() -> ! {
    loop {
        cortex_m::asm::nop();
    }
}
