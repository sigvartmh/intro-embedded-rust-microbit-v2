#![no_main]
#![no_std]

use cortex_m_rt::entry;

use nrf52833_pac as pac;

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[entry]
fn main() -> ! {
    // Take the peripheral structure
    let p = pac::Peripherals::take().unwrap();
    p.P0.pin_cnf[28].write(|w| w.dir().output());
    p.P0.pin_cnf[11].write(|w| w.dir().output());
    p.P0.pin_cnf[21].write(|w| w.dir().output());
    p.P0.pin_cnf[22].write(|w| w.dir().output());

    let mut count: u8 = 0;
    loop {
        count += 1;

        if count & 1 == 1 {
            // Set the register value of port0 output register to 1 for bit 21 og 22 which are
            // connected to the diods in the LED matrix on the board.  
            p.P0.out.write(|w| unsafe { w.bits((1 << 21) | (1 << 22)) });
        } else {
            // sets all the registers to low
            p.P0.out.write(|w| unsafe { w.bits(0) });
        }

        for _ in 0..50_000 {
            cortex_m::asm::nop();
        }
    }
}
