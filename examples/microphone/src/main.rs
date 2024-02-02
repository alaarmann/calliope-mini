#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;

use calliope_mini::{
    board::Board,
    display::blocking::Display,
    hal::{adc::AdcConfig, prelude::*, Adc, Timer},
};

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut display = Display::new(board.display_pins);

        // initialize adc
        let adc_config = AdcConfig::default();
        let mut adc = Adc::new(board.ADC, adc_config);
        let mut mic_in = board.microphone_pins.mic_in.into_floating_input();

        let mut count: u64 = 0;
        let mut sum: u64 = 0;
        let mut max_value: u16 = 0;
        loop {
            let mic_value = adc
                .read(&mut mic_in)
                .expect("could not read value of microphone") as u16;

            // Smoothen the signal as audio comes in waves
            max_value = max_value.max(mic_value);
            sum += mic_value as u64;
            count += 1;

            if count % 100 == 0 {
                let avg = (sum / count) as u16;
                let image = [
                    [if max_value > avg + 100 { 1 } else { 0 }; 5],
                    [if max_value > avg + 80 { 1 } else { 0 }; 5],
                    [if max_value > avg + 60 { 1 } else { 0 }; 5],
                    [if max_value > avg + 40 { 1 } else { 0 }; 5],
                    [if max_value > avg + 20 { 1 } else { 0 }; 5],
                ];
                display.show(&mut timer, image, 10);
                max_value = 0;
            }
        }
    }

    panic!("End");
}
