#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use nb::block;

use stm32f1xx_hal::{
    prelude::*,
    pac,
    timer::Timer
};
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
#[entry]
fn fn main() -> ! {
    let cp = cortex_m::Peripheralsls::take().unwarp();
    let dp = pac::Peripherals::take().unwrap();

}

