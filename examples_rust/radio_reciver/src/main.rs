#![no_main]
#![no_std]

use cortex_m as _;
use core::str;


use rtt_target::{rprintln, rtt_init_print};
use microbit::{
    board::Board,
    hal::{prelude::*, Timer, Clocks},
    hal::ieee802154::{Radio, Channel, Packet, TxPower, Error},
};

use panic_rtt_target as _;

const TEN_MS: u32 = 10_000;

/*
#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        rprintln!("PANIC!");
        cortex_m::asm::bkpt();
    }
}
*/

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Radio example!");
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let clocks = Clocks::new(board.CLOCK).enable_ext_hfosc();
    let mut radio = {
        let mut radio = Radio::init(board.RADIO, &clocks);
        // set TX power to its maximum value
        radio.set_channel(Channel::_20); // <- must match the Dongle's listening channel
        radio.set_txpower(TxPower::Pos8dBm);
        rprintln!(
            "Radio initialized and configured with TX power set to the maximum value"
        );
        radio
    };
    rprintln!("Radio is up");


    // these three are equivalent
    loop {
        let mut packet = Packet::new();
        if let Ok(true) = board.buttons.button_a.is_low() {
            let msg: &[u8; 5] = b"Hello";
            packet.copy_from_slice(msg);
            let res = radio.try_send(&mut packet);
            match res {
                Ok(_) => rprintln!("Sending: {} which is {} bytes", str::from_utf8(&packet).expect("invalid UTF-8"), packet.len()),
                Err(_) => rprintln!("error sending: {}", str::from_utf8(msg).expect("Something went wrong"))
            }
            timer.delay_ms(1000_u32);
        }

        let res = radio.recv_timeout(&mut packet, &mut timer, TEN_MS);
        match res {
            Ok(crc) => {
                let message = str::from_utf8(&*packet);
                match  message {
                    Ok(message) => rprintln!("received: {} (CRC = {:X})", message, crc),
                    Err(_) => rprintln!("Unable to decode message")
                }
            }
            Err(Error::Crc(crc)) => rprintln!("CRC error: {:X}", crc),
            _ => ()
        }
    }

    panic!("End");
}
