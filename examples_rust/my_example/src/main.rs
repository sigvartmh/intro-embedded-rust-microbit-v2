#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m::asm;
use rtt_target::{rprintln, rtt_init_print};

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // abort instruction: triggers a HardFault exception which causes probe-run to exit
    asm::udf()
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");
    loop { continue; };
}
