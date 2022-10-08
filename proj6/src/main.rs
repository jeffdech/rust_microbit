// #![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
// use panic_halt as _;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::{
    board::Board,
    hal::{
        pac,
        prelude::*, pwm, 
        gpio::{Pin, Input, PullUp, Level}, 
        timer::Timer
    },
};

type LEDPin = Pin<Input<PullUp>>;

struct RGBLed {
    pwm: pwm::Pwm<pac::PWM0>,
    red: LEDPin,
    green: LEDPin,
    blue: LEDPin,
    duty_cycle: u16,
}

impl RGBLed {
    pub fn new(pwm: pwm::Pwm<pac::PWM0>, red: LEDPin, green: LEDPin, blue: LEDPin) -> Self {
        let duty_cycle = 256_u16;

        pwm.set_prescaler(pwm::Prescaler::Div128);

        pwm.set_output_pin(pwm::Channel::C0, red);
        pwm.set_output_pin(pwm::Channel::C1, green);
        pwm.set_output_pin(pwm::Channel::C2, blue);

        Self {
            pwm,
            red,
            blue,
            green,
            duty_cycle
        }
    }

    pub fn set_color(&mut self, red: u8, green: u8, blue: u8) {
        self.pwm.set_duty_on(pwm::Channel::C0, red as u16);
        self.pwm.set_duty_on(pwm::Channel::C1, green as u16);
        self.pwm.set_duty_on(pwm::Channel::C2, blue as u16);
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    
    rprintln!("Starting...");

    if let Some(mut board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut pwm0 = pwm::Pwm::new(board.PWM0);

        rprintln!("Timer and PWM set up...");

        let mut rbg_led = RBGLed::new(
            pwm0,
            board.pins.p0_02.into_pulldown_input().degrade(),
            board.pins.p0_03.into_pulldown_input().degrade(),
            board.pins.p0_04.into_pulldown_input().degrade()
        );

        rprintln!("RGB LED set up...");

        let colors = [
            [255, 0, 0],
            [0, 255, 0],
            [0, 0, 255]
        ];

        rprintln!("Entering loop...");

        loop {
            for i in (0..3) {
                rprintln!("{} {} {}", colors[i][0], colors[i][1], colors[i][2]);
                rgb_led.set_color(colors[i][0], colors[i][1], colors[i][2]);
                timer.delay_ms(250u16);
            }
        }
    }

    panic!("Could not take peripherals");
}