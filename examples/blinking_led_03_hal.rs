#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::digital::{OutputPin, PinState};
use hal::{gpio::Level, pac};
use nrf51_hal::{self as hal, gpio::p0::Parts};
use panic_halt as _;

#[entry]
fn main() -> ! {
    /* COL1 pin is 04, ROW1 pin is 13, source: https://github.com/microbit-foundation/microbit-reference-design/blob/master/PDF/Schematic%20Print/Schematic%20Prints.PDF
     */
    let p = pac::Peripherals::take().unwrap();
    let port0 = Parts::new(p.GPIO);

    let _col1 = port0.p0_04.into_push_pull_output(Level::Low);
    let mut row1 = port0.p0_13.into_push_pull_output(Level::Low);

    let mut is_on = false;
    loop {
        let _ = row1.set_state(PinState::from(is_on));
        for _ in 0..10_000 {
            nop();
        }
        is_on = !is_on;
    }
}
