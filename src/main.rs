#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

//use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use embedded_hal::digital::v2::OutputPin;
use nrf52810_hal as hal;
use nrf52810_hal::gpio::Level;
//use nrf52810_hal::delay::Delay;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;

// {port0.p0_6, port0.p0_9, port0.p0_10, port0.p0_12, port0.p0_14, port0.p0_15, port0.p0_16, port0.p0_18, port0.p0_20, port0.p0_25} // gpio-only pins
// {port0.p0_4, port0.p0_5, port0.p0_28, port0.p0_30}// analogin GPIO pins
// {AIN2,       AIN3,       AIN4,        AIN6}// analogin AINs

#[entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();
    let core = hal::pac::CorePeripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);

    //let timer = hal::Timer::new(p.TIMER0);
    let mut delay = hal::Delay::new(core.SYST);

    hprintln!("Hello, H  world!").unwrap();

    
    // UPDATE TO GO THROUGH ALL OF THE PINS
    let mut led = port0.p0_13.into_push_pull_output(Level::Low);

    loop {
        led.set_high().unwrap();
        delay.delay_ms(100_u16);
        led.set_low().unwrap();
        delay.delay_ms(100_u16);
    }
}