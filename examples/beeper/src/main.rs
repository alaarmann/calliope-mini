#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use calliope_mini::{
    hal::{
        gpio::Level,
        gpiote::*,
        ppi::{self, ConfigurablePpi, Ppi},
    },
    Board,
};
use cortex_m_rt::entry;

const MIN_FREQUENCY_HZ_NO_PRESCALER: u32 = 245;
const PRESCALER_LOW_FREQUENCY: u32 = 4;

const _MIN_FREQUENCY_HZ: u32 = 20; //min human audible frequency
const _MAX_FREQUENCY_HZ: u32 = 20000; //max human audible frequency
const BOARD_FREQUENCY_HZ: u32 = 16000000;

const DEFAULT_DUTY: u32 = 100;

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        // stop controller
        let motor_nsleep = board
            .beeper_motor_pins
            .motor_nsleep
            .into_push_pull_output(Level::Low);

        // PWM init

        // output pins
        let outpin1 = board
            .beeper_motor_pins
            .motor_in1
            .into_push_pull_output(Level::Low)
            .degrade();
        let outpin2 = board
            .beeper_motor_pins
            .motor_in2
            .into_push_pull_output(Level::Low)
            .degrade();

        let gpiote = Gpiote::new(board.GPIOTE);
        // Output channel for 1
        gpiote
            .channel0()
            .output_pin(outpin1)
            .task_out_polarity(TaskOutPolarity::Toggle)
            .init_low();
        // enable task
        gpiote.channel0().task_out().write(|w| unsafe { w.bits(1) });
        // Output channel for 2
        gpiote
            .channel1()
            .output_pin(outpin2)
            .task_out_polarity(TaskOutPolarity::Toggle)
            .init_high();
        // enable task
        gpiote.channel1().task_out().write(|w| unsafe { w.bits(1) });

        let ppi_channels = ppi::Parts::new(board.PPI);
        // Set both outputs high form Timer0 CC[0]
        // Set each output low from the respective Timer0 CC[1] and CC[2]
        // Each timer can run 3
        let mut ppi0 = ppi_channels.ppi0;
        ppi0.set_task_endpoint(gpiote.channel0().task_out());
        ppi0.set_event_endpoint(&board.TIMER0.events_compare[0]);
        ppi0.enable(); // CHENSET

        let mut ppi1 = ppi_channels.ppi1;
        ppi1.set_task_endpoint(gpiote.channel1().task_out());
        ppi1.set_event_endpoint(&board.TIMER0.events_compare[1]);
        ppi1.enable(); // CHENSET

        let mut ppi2 = ppi_channels.ppi2;
        ppi2.set_task_endpoint(gpiote.channel0().task_out());
        ppi2.set_event_endpoint(&board.TIMER0.events_compare[2]);
        ppi2.enable(); // CHENSET

        let mut ppi3 = ppi_channels.ppi3;
        ppi3.set_task_endpoint(gpiote.channel1().task_out());
        ppi3.set_event_endpoint(&board.TIMER0.events_compare[3]);
        ppi3.enable(); // CHENSET

        // The Timer PAC is used directly as the HAL does not give full access to all registers
        board.TIMER0.mode.write(|w| unsafe { w.bits(0) }); // select timer mode
        board.TIMER0.bitmode.write(|w| unsafe { w.bits(0) }); // 16bit timer bit width
                                                              // CC[0] every 20 ms (50 Hz)
        board.TIMER0.prescaler.write(|w| unsafe { w.bits(2) }); // prescaler: source clock frequency is divided by 2^SCALE
        board.TIMER0.tasks_clear.write(|w| unsafe { w.bits(1) }); // trigger tasks clear

        board.TIMER0.shorts.write(|w| unsafe { w.bits(1 << 3) }); // shortcut between COMPARE[3] and
                                                                  // CLEAR

        // set up sound
        let frequency_hz = 220;

        //stop & clear timer
        board.TIMER0.tasks_stop.write(|w| unsafe { w.bits(1) });
        board.TIMER0.tasks_clear.write(|w| unsafe { w.bits(1) });

        //set prescaler for sound use
        let prescaler = if frequency_hz < MIN_FREQUENCY_HZ_NO_PRESCALER {
            PRESCALER_LOW_FREQUENCY
        } else {
            0
        };

        board
            .TIMER0
            .prescaler
            .write(|w| unsafe { w.bits(prescaler) }); // prescaler: source clock frequency is divided by 2^SCALE

        //disable GPIOTE control of the pins
        gpiote.channel0().task_out().write(|w| unsafe { w.bits(0) });
        gpiote.channel1().task_out().write(|w| unsafe { w.bits(0) });

        //max 50% duty per pwm just like in dual motor use
        let duty = DEFAULT_DUTY / 2;

        //calculate period corresponding to the desired frequency and the currently used prescaler
        let period = if frequency_hz < MIN_FREQUENCY_HZ_NO_PRESCALER {
            BOARD_FREQUENCY_HZ / (frequency_hz << PRESCALER_LOW_FREQUENCY)
        } else {
            BOARD_FREQUENCY_HZ / frequency_hz
        };

        //set compare register 2 and 3 according to the given frequency (this sets the PWM period)
        board.TIMER0.cc[2].write(|w| unsafe { w.bits(period - 1) });
        board.TIMER0.cc[3].write(|w| unsafe { w.bits(period) });

        //set duty cycle
        board.TIMER0.cc[0].write(|w| unsafe { w.bits(period - (period * duty) / 100) });
        board.TIMER0.cc[1].write(|w| unsafe { w.bits((period * duty) / 100 - 1) });

        //enable task & restart PWM
        gpiote.channel0().task_out().write(|w| unsafe { w.bits(1) });
        gpiote.channel1().task_out().write(|w| unsafe { w.bits(1) });

        board.TIMER0.tasks_start.write(|w| unsafe { w.bits(1) });

        //activate controller
        motor_nsleep.into_push_pull_output(Level::High);

        loop {}
    }
    panic!("End");
}
