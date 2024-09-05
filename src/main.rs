#![no_std]
#![no_main]

use core::ptr::write_volatile;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    /* COL1 pin is 04, ROW1 pin is 13, source: https://github.com/microbit-foundation/microbit-reference-design/blob/master/PDF/Schematic%20Print/Schematic%20Prints.PDF
        Addresses can found in https://infocenter.nordicsemi.com/pdf/nRF51_RM_v3.0.pdf page 57
    */
    const GPIO0_PINCNF13_ROW1_ADDR: *mut u32 = 0x5000_0734 as *mut u32;
    const GPIO0_PINCNF04_COL1_ADDR: *mut u32 = 0x5000_0710 as *mut u32;
    const DIR_OUTPUT_POS: u32 = 0;
    const PINCNF_DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS;
    unsafe {
        write_volatile(GPIO0_PINCNF13_ROW1_ADDR, PINCNF_DRIVE_LED);
        write_volatile(GPIO0_PINCNF04_COL1_ADDR, PINCNF_DRIVE_LED);
    }
    const GPIO0_OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32;
    const GPIO0_OUT_ROW1_POS: u32 = 13;
    let mut is_on = false;
    loop {
        unsafe {
            write_volatile(GPIO0_OUT_ADDR, (is_on as u32) << GPIO0_OUT_ROW1_POS);
        }
        for _ in 0..400_000 {
            nop();
        }
        is_on = !is_on;
    }
}
