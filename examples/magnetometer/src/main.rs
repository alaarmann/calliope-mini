#![no_std]
#![no_main]

use bmx055::Bmx055;
use calliope_mini::hal::{
    pac::twi0::frequency::FREQUENCY_A,
    prelude::*,
    twi::{self},
    Timer,
};
use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let board = calliope_mini::Board::take().unwrap();

    let i2c = { twi::Twi::new(board.TWI0, board.i2c.into(), FREQUENCY_A::K100) };

    let mut timer = Timer::new(board.TIMER0);

    let mut sensor = Bmx055::new_with_i2c(i2c);
    defmt::println!("Initializing...");
    sensor.init(&mut timer).unwrap();

    sensor.set_accel_bw(bmx055::AccelBandwidth::Hz125).unwrap();
    defmt::println!("Initialized.");

    loop {
        let magnetic_field_data = sensor.magnetic_field_data().unwrap();
        if magnetic_field_data.data_ready() {
            defmt::println!(
                "Magnetic Field Data (in microTesla): x {} y {} z {}",
                magnetic_field_data.x_compensated_ut(),
                magnetic_field_data.y_compensated_ut(),
                magnetic_field_data.z_compensated_ut(),
            );
        }
        timer.delay_ms(100_u32);
    }
}
