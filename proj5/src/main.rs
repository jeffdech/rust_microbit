// #![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
// use panic_halt as _;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::{
    pac,
    hal::gpio::{p0, p1, Level, Pin, Output, PushPull},
    hal::prelude::*,
    hal::timer::Timer,
};

struct LEDBar {
    pins: [Pin<Output<PushPull>>; 10]
}

impl LEDBar {
    pub fn new(p0per: pac::P0, p1per: pac::P1) -> Self {
        let p0parts = p0::Parts::new(p0per);
        let p1parts = p1::Parts::new(p1per);

        Self {
            pins: [
                p0parts.p0_02.into_push_pull_output(Level::Low).degrade(),
                p0parts.p0_03.into_push_pull_output(Level::Low).degrade(),
                p0parts.p0_04.into_push_pull_output(Level::Low).degrade(),
                p0parts.p0_31.into_push_pull_output(Level::Low).degrade(),
                p0parts.p0_28.into_push_pull_output(Level::Low).degrade(),
                p1parts.p1_05.into_push_pull_output(Level::Low).degrade(),
                p0parts.p0_11.into_push_pull_output(Level::Low).degrade(),
                p0parts.p0_09.into_push_pull_output(Level::Low).degrade(),
                p0parts.p0_10.into_push_pull_output(Level::Low).degrade(),
                p0parts.p0_12.into_push_pull_output(Level::Low).degrade(),
            ]
        }
    }

    pub fn set_value(&mut self, value: usize) {
        let mut sval = value;
        if sval > 10 {
            sval = 10;
        } else if sval < 1 {
            sval = 1;
        }

        for i in 0..sval {
            self.pins[i].set_high();
        }

        for i in sval..10 {
            self.pins[i].set_low();
        }
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    if let Some(peripherals) = microbit::pac::Peripherals::take()  {

        let mut led_bar = LEDBar::new(peripherals.P0, peripherals.P1);
        let mut timer = Timer::new(peripherals.TIMER0);

        loop {
            for i in 1..=10 {
                rprintln!("{}", i);
                led_bar.set_value(i);
                timer.delay_ms(500u16);
            }
        }
    }

    panic!("Could not take peripherals");
}