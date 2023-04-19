#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;
use defmt as _;

use cortex_m_rt::entry;

use microbit::{
    hal::{
        prelude::*,
    },
};

use core::fmt::Write;

use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

use core::fmt;
use embedded_hal::blocking::serial as bserial;
use embedded_hal::serial;
use microbit::hal::uarte::{Error, Instance, Uarte, UarteRx, UarteTx};

static mut TX_BUF: [u8; 1] = [0; 1];
static mut RX_BUF: [u8; 1] = [0; 1];

pub struct UartePort<T: Instance>(UarteTx<T>, UarteRx<T>);

impl<T: Instance> UartePort<T> {
    pub fn new(serial: Uarte<T>) -> UartePort<T> {
        let (tx, rx) = serial
            .split(unsafe { &mut TX_BUF }, unsafe { &mut RX_BUF })
            .unwrap();
        UartePort(tx, rx)
    }
}

impl<T: Instance> fmt::Write for UartePort<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }
}

impl<T: Instance> serial::Write<u8> for UartePort<T> {
    type Error = Error;

    fn write(&mut self, b: u8) -> nb::Result<(), Self::Error> {
        self.0.write(b)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.0.flush()
    }
}

impl<T: Instance> bserial::write::Default<u8> for UartePort<T> {}

impl<T: Instance> serial::Read<u8> for UartePort<T> {
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.1.read()
    }
}


#[entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };
    defmt::info!("configuring serial");

    loop {
        write!(serial, "Hello World:\r\n").unwrap();
        let input = nb::block!(serial.read()).unwrap();
        write!(serial, "You said: {}\r\n", input as char).unwrap();
        defmt::info!("You said: {}", input as char);
    }
}



/*
#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        p.GPIO.pin_cnf[24].write(|w| w.pull().pullup().dir().output());
        p.GPIO.pin_cnf[25].write(|w| w.pull().disabled().dir().input());

        p.UART0.pseltxd.write(|w| unsafe { w.bits(24) });
        p.UART0.pselrxd.write(|w| unsafe { w.bits(25) });

        p.UART0.baudrate.write(|w| w.baudrate().baud115200());
        p.UART0.enable.write(|w| w.enable().enabled());

        let _ = write_uart0(&p.UART0, "Hello World!\n");
    }

    loop {
        continue;
    }
}

fn write_uart0(uart0: &microbit::pac::UART0, s: &str) -> core::fmt::Result {
    uart0.tasks_starttx.write(|w| unsafe { w.bits(1) });
    for c in s.as_bytes() {
        /* Write the current character to the output register */
        uart0.txd.write(|w| unsafe { w.bits(u32::from(*c)) });

        /* Wait until the UART is clear to send */
        while uart0.events_txdrdy.read().bits() == 0 {}

        /* And then set it back to 0 again, just because ?!? */
        uart0.events_txdrdy.write(|w| unsafe { w.bits(0) });
    }
    uart0.tasks_stoptx.write(|w| unsafe { w.bits(1) });
    Ok(())
}
use core::str;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        defmt::info!("Pin config"); 
        p.P0.pin_cnf[6].write(|w| w.pull().pullup().dir().output());
        p.P1.pin_cnf[8].write(|w| w.pull().disabled().dir().input());

        let uart0 = p.UART0;
        /* Tell UART which pins to use for sending and receiving */
        uart0.psel.txd.write(|w| unsafe { w.bits(6) });
        uart0.psel.rxd.write(|w| unsafe { w.bits(8) });

        /* Set a typical baud rate of 115200 */
        uart0.baudrate.write(|w| w.baudrate().baud115200());

        /* Enable UART function */
        uart0.enable.write(|w| w.enable().enabled());

        /* Print a nice hello message */
        let _ = write_uart0(&uart0, "Please type characters to echo:\r\n");

        /* Fire up receiving task */
        uart0.tasks_startrx.write(|w| unsafe { w.bits(1) });

        /* Endless loop */
        defmt::info!("Starting loop"); 
        loop {
            /* Busy wait for reception of data */
            while uart0.events_rxdrdy.read().bits() == 0 {}

            /* We're going to pick up the data soon, let's signal the buffer is already waiting for
             * more data */
            uart0.events_rxdrdy.write(|w| unsafe { w.bits(0) });

            /* Read one 8bit value */
            let c = uart0.rxd.read().bits() as u8;

            /* What comes in must go out, we don't care what it is */
            let _ = write_uart0(&uart0, unsafe { str::from_utf8_unchecked(&[c; 1]) });
        }
    }

    panic!("End");
}

fn write_uart0(uart0: &microbit::pac::UART0, s: &str) -> core::fmt::Result {
    /* Start UART sender */
    uart0.tasks_starttx.write(|w| unsafe { w.bits(1) });

    for c in s.as_bytes() {
        /* Write the current character to the output register */
        uart0.txd.write(|w| unsafe { w.bits(u32::from(*c)) });

        /* Wait until the UART is clear to send */
        while uart0.events_txdrdy.read().bits() == 0 {}

        /* And then reset it for the next round */
        uart0.events_txdrdy.write(|w| unsafe { w.bits(0) });
    }

    /* Stop UART sender */
    uart0.tasks_stoptx.write(|w| unsafe { w.bits(1) });
    Ok(())
}
*/
