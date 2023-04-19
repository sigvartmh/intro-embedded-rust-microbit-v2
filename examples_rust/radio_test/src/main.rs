#![no_main]
#![no_std]

use cortex_m;

use rtt_target::{rprintln, rtt_init_print};
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer, Clocks},
    hal::clocks,
    hal::ieee802154::{Radio, Channel, Packet, TxPower},
};

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        rprintln!("PANIC!");
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Radio example!");
    if let Some(board) = Board::take() {
            board.display_pins.row1.set_high().unwrap();
            let mut led1 = board.display_pins.col1;
            let mut button_a = board.buttons.button_a;
            button_a.set_high();


        let mut timer = Timer::new(board.TIMER0);
        rprintln!("Timer!");
        let mut display = Display::new(board.display_pins);
        let clocks = Clocks::new(board.CLOCK);
        rprintln!("Clocks!");
        let _clocks = clocks.enable_ext_hfosc();
        let mut radio = {
            let mut radio = Radio::init(board.RADIO, &_clocks);
            // set TX power to its maximum value
            radio.set_txpower(TxPower::Pos8dBm);
            rprintln!(
                "Radio initialized and configured with TX power set to the maximum value"
            );
            radio
        };
        rprintln!("Radio is up");

         radio.set_channel(Channel::_20); // <- must match the Dongle's listening channel
    radio.set_txpower(TxPower::Pos8dBm);

    let mut packet = Packet::new();

    // these three are equivalent
    let msg: &[u8; 5] = &[72, 101, 108, 108, 111];
    // let msg: &[u8; 5] = &[b'H', b'e', b'l', b'l', b'o'];
    // let msg: &[u8; 5] = b"Hello";

    rprintln!(
        "sending: {:?}", msg
    );

    packet.copy_from_slice(msg);

    radio.send(&mut packet);

        #[allow(non_snake_case)]
        let letter_I = [
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        let heart = [
            [0, 1, 0, 1, 0],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_R = [
            [0, 1, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_u = [
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_s = [
            [0, 0, 1, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 1, 0, 0],
            [0, 0, 0, 1, 0],
            [0, 1, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_t = [
            [1, 1, 1, 1, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];
        
    rprintln!("Starting loop");
        loop {
            display.show(&mut timer, letter_I, 1000);
            display.show(&mut timer, heart, 1000);
            display.show(&mut timer, letter_R, 1000);
            display.show(&mut timer, letter_u, 1000);
            display.show(&mut timer, letter_s, 1000);
            display.show(&mut timer, letter_t, 1000);
            display.clear();
            timer.delay_ms(250_u32);
        }
    }

    panic!("End");
}
