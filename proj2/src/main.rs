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
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let light_mat_a = [
        [0, 0, 0, 0, 1],
        [0, 0, 0, 1, 0],
        [1, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0]
    ];

    let light_mat_b = [
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 1, 0],
        [1, 0, 0, 0, 1]
    ];

    let mut mat_a_selected = true;

    display.clear();
    loop {
        mat_a_selected = if (board.buttons.button_a.is_low().unwrap()) {
            true
        } else if (board.buttons.button_b.is_low().unwrap()) {
            false
        } else {
            mat_a_selected
        };

        rprintln!("{} selected", if mat_a_selected { "A" } else { "B"} );
        
        display.show(
            &mut timer, 
            match mat_a_selected {
                true => light_mat_a,
                false => light_mat_b
            },
            250
        );
    }
}