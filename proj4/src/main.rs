// #![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
// use panic_halt as _;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{
        prelude::*, 
        Timer
    },
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    
    let mut led_out = board.pins.p0_02.into_push_pull_output(microbit::hal::gpio::Level::Low);
    let button_in = board.pins.p0_03.into_pulldown_input();

    loop {
        if button_in.is_low().unwrap() {
            led_out.set_high();
            rprintln!("LED High");
        } else {
            led_out.set_low();
            rprintln!("LED Low");
        }
    }
}