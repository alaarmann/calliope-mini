//! Named GPIO pin types
//!
//! This module maps the GPIO pin names as described in the
//! [Pins and Signals section of the micro:bit site](https://tech.microbit.org/hardware/edgeconnector/#pins-and-signals)
//! Where appropriate the pins are restricted with the appropriate `MODE`
//! from `nrf-hal`.
#![allow(clippy::upper_case_acronyms, missing_docs)]
use crate::hal::gpio::{p0, Floating, Input, Output, Pin, PushPull};

/* GPIO pads */
pub type PAD0<MODE> = p0::P0_00<MODE>;
pub type PAD1<MODE> = p0::P0_01<MODE>;
pub type PAD2<MODE> = p0::P0_02<MODE>;
pub type PAD3<MODE> = p0::P0_22<MODE>;

/* LED display */
pub const NUM_COLS: usize = 9;
pub type COL1 = p0::P0_04<Output<PushPull>>;
pub type COL2 = p0::P0_05<Output<PushPull>>;
pub type COL3 = p0::P0_06<Output<PushPull>>;
pub type COL4 = p0::P0_07<Output<PushPull>>;
pub type COL5 = p0::P0_08<Output<PushPull>>;
pub type COL6 = p0::P0_09<Output<PushPull>>;
pub type COL7 = p0::P0_10<Output<PushPull>>;
pub type COL8 = p0::P0_11<Output<PushPull>>;
pub type COL9 = p0::P0_12<Output<PushPull>>;

pub const NUM_ROWS: usize = 3;
pub type ROW1 = p0::P0_13<Output<PushPull>>;
pub type ROW2 = p0::P0_14<Output<PushPull>>;
pub type ROW3 = p0::P0_15<Output<PushPull>>;

/// GPIO pins connected to the LED matrix
///
/// The pins are represented as a [3x9 matrix on the micro:bit
/// V1](https://tech.microbit.org/hardware/1-5-revision/#display).
/// This is mapped to the physical 5x5 LED matrix in the [crate::display]
/// modules.
///
/// Use the [display_pins] macro for easier construction.
pub struct DisplayPins {
    pub col1: COL1,
    pub col2: COL2,
    pub col3: COL3,
    pub col4: COL4,
    pub col5: COL5,
    pub col6: COL6,
    pub col7: COL7,
    pub col8: COL8,
    pub col9: COL9,
    pub row1: ROW1,
    pub row2: ROW2,
    pub row3: ROW3,
}

/// GPIO pins connected to the beeper / motor driver
pub struct BeeperMotorPins {
    pub motor_nsleep: MOTOR_NSLEEP,
    pub motor_in1: MOTOR_IN1,
    pub motor_in2: MOTOR_IN2,
}

/// GPIO pins connected to the microphone
pub struct MicrophonePins {
    pub mic_in: MIC,
}

type LED = Pin<Output<PushPull>>;

impl DisplayPins {
    pub fn degrade(self) -> ([LED; NUM_COLS], [LED; NUM_ROWS]) {
        (
            [
                self.col1.degrade(),
                self.col2.degrade(),
                self.col3.degrade(),
                self.col4.degrade(),
                self.col5.degrade(),
                self.col6.degrade(),
                self.col7.degrade(),
                self.col8.degrade(),
                self.col9.degrade(),
            ],
            [
                self.row1.degrade(),
                self.row2.degrade(),
                self.row3.degrade(),
            ],
        )
    }
}

/// Create [DisplayPins] from a [GPIO Parts](crate::hal::gpio::p0::Parts)
///
/// # Example
///
/// ```no_run
/// # use microbit_common as microbit;
/// use microbit::{
///     display_pins,
///     pac,
///     hal::gpio::p0::Parts as P0Parts,
/// };
///
/// // take the peripherals
/// let p = pac::Peripherals::take().unwrap();
/// // split off the P0 GPIO port
/// let p0parts = P0Parts::new(p.GPIO);
///
/// let pins = display_pins!(p0parts);
/// ```
#[macro_export]
macro_rules! display_pins {
    ( $p0parts:expr ) => {{
        use microbit::{gpio::DisplayPins, hal::gpio::Level};

        DisplayPins {
            row1: $p0parts.p0_13.into_push_pull_output(Level::Low),
            row2: $p0parts.p0_14.into_push_pull_output(Level::Low),
            row3: $p0parts.p0_15.into_push_pull_output(Level::Low),
            col1: $p0parts.p0_04.into_push_pull_output(Level::Low),
            col2: $p0parts.p0_05.into_push_pull_output(Level::Low),
            col3: $p0parts.p0_06.into_push_pull_output(Level::Low),
            col4: $p0parts.p0_07.into_push_pull_output(Level::Low),
            col5: $p0parts.p0_08.into_push_pull_output(Level::Low),
            col6: $p0parts.p0_09.into_push_pull_output(Level::Low),
            col7: $p0parts.p0_10.into_push_pull_output(Level::Low),
            col8: $p0parts.p0_11.into_push_pull_output(Level::Low),
            col9: $p0parts.p0_12.into_push_pull_output(Level::Low),
        }
    }};
}

/* buttons */
pub type BTN_A = p0::P0_17<Input<Floating>>;
pub type BTN_B = p0::P0_16<Input<Floating>>;

/* rgb led */
pub type RGB_LED<MODE> = p0::P0_18<MODE>;

/* motor */
pub type MOTOR_NSLEEP = p0::P0_28<Output<PushPull>>;
pub type MOTOR_IN1 = p0::P0_29<Output<PushPull>>;
pub type MOTOR_IN2 = p0::P0_30<Output<PushPull>>;

/* mic */
pub type MIC = p0::P0_03<Input<Floating>>;

/* spi */
pub type MOSI<MODE> = p0::P0_09<MODE>;
pub type MISO<MODE> = p0::P0_08<MODE>;
pub type SCK<MODE> = p0::P0_07<MODE>;

/* i2c - shared external and internal */
pub type SCL = p0::P0_19<Input<Floating>>;
pub type SDA = p0::P0_20<Input<Floating>>;

/* uart */
pub type UART_TX = p0::P0_24<Output<PushPull>>;
pub type UART_RX = p0::P0_25<Input<Floating>>;

/* accelerometer interrupt pin */
pub type ACCEL_INT<MODE> = p0::P0_21<MODE>;

/* edge connector */
// -> GND
// -> +V
pub type EDGE00<MODE> = PAD0<MODE>;
pub type EDGE01<MODE> = PAD1<MODE>;
pub type EDGE02<MODE> = PAD2<MODE>;
pub type EDGE03<MODE> = PAD3<MODE>;
pub type EDGE04 = COL1;
pub type EDGE05 = COL2;
pub type EDGE06 = COL3;
pub type EDGE07 = COL4;
pub type EDGE08 = COL5;
pub type EDGE09 = COL6;
// -> GND
// -> GND
pub type EDGE10 = COL7;
pub type EDGE11 = COL8;
pub type EDGE12 = COL9;
pub type EDGE13 = ROW1;
pub type EDGE14 = ROW2;
pub type EDGE15 = ROW3;
pub type EDGE16<MODE> = p0::P0_26<MODE>;
pub type EDGE17<MODE> = p0::P0_27<MODE>;
pub type EDGE18 = SDA;
pub type EDGE19 = SCL;
// -> +V
// -> GND
