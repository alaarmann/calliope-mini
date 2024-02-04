#![no_main]
#![no_std]

use core::arch::asm;

use defmt_rtt as _;
use panic_halt as _;

use calliope_mini::{
    board::Board,
    hal::{gpio::Level, prelude::*},
    pac::GPIO,
};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    board.display_pins.row1.set_high().unwrap();
    let mut led1 = board.display_pins.col1;

    let _rgb_led_pin = board.rgb_led_pin.into_push_pull_output(Level::Low);

    let grb: [u8; 3] = [0, 0, 0];
    set_colour(grb);

    // order is green, red, blue
    let grb: [u8; 3] = [0, 64, 0];
    set_colour(grb);

    loop {
        if let Ok(true) = board.buttons.button_a.is_high() {
            let _ = led1.set_high();
        } else {
            let _ = led1.set_low();
        }
    }
}

#[allow(unused_assignments)]
fn set_colour(grb: [u8; 3]) {
    unsafe {
        (*GPIO::ptr()).outclr.write(|w| w.pin18().set_bit());
    }
    nrf_delay_us(60);
    let mut frame: [bool; 24] = [false; 24];
    let mut i: usize = 0;
    for c in 0..3 {
        for b in (0..8).rev() {
            frame[i] = (grb[c] & (1u8 << b)) != 0;
            i += 1;
        }
    }

    for bit in frame {
        // GPIO base address 0x50000000
        // OUTSET offset 0x508
        // OUTCLR offset 0x50C
        // at 16 MHz 1 cycle is 62.5 nanoseconds
        // subtract 2 cycles for STR instruction (needs verification)
        let mut rgb_led_pin = 1 << 18;
        let mut gpio_base = 0x50000000;
        let mut outset_offset = 0x508;
        let mut outclr_offset = 0x50c;
        if bit {
            unsafe {
                asm!(
                "str {0},[{1}, {2}]", // OUTSET
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "str {0},[{1}, {3}]", // OUTCLR
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                inout(reg) rgb_led_pin,
                inout(reg) gpio_base,
                inout(reg) outset_offset,
                inout(reg) outclr_offset,
                );
            }
        } else {
            unsafe {
                asm!(
                "str {0},[{1}, {2}]", // OUTSET
                "nop",
                "nop",
                "nop",
                "nop",
                "str {0},[{1}, {3}]", // OUTCLR
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                inout(reg) rgb_led_pin,
                inout(reg) gpio_base,
                inout(reg) outset_offset,
                inout(reg) outclr_offset,
                );
            }
        }
    }
    nrf_delay_us(60);
}

#[allow(unused_assignments)]
#[inline(always)]
fn nrf_delay_us(mut delay_us: u32) {
    unsafe {
        asm!(
        "2:",
        "subs {0}, {0}, #1",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "bne 2b",
        inout(reg) delay_us
        );
    }
}
