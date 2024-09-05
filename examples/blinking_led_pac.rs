#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use nrf51_pac::Peripherals;
use panic_halt as _;

#[entry]
fn main() -> ! {
    /* COL1 pin is 04, ROW1 pin is 13, source: https://github.com/microbit-foundation/microbit-reference-design/blob/master/PDF/Schematic%20Print/Schematic%20Prints.PDF
     */
    let p = Peripherals::take().unwrap();

    p.GPIO.pin_cnf[13].write(|w| w.dir().output());
    p.GPIO.pin_cnf[04].write(|w| w.dir().output());

    let mut is_on = false;
    loop {
        p.GPIO.out.write(|w| w.pin13().bit(is_on));
        for _ in 0..50_000 {
            nop();
        }
        is_on = !is_on;
    }
}
