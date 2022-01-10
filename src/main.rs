#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use heapless::Deque;

//use cortex_m::asm;
use cortex_m_rt::entry;
//use cortex_m_semihosting::*;

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::InputPin;
use nrf52810_hal as hal;
use nrf52810_hal::gpio::Level;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;

// {port0.p0_06, port0.p0_09, port0.p0_10, port0.p0_12, port0.p0_14, port0.p0_15, port0.p0_16, port0.p0_18, port0.p0_20, port0.p0_25} // gpio-only pins
// {port0.p0_04, port0.p0_05, port0.p0_28, port0.p0_30}// analogin GPIO pins

#[entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();
    let core = hal::pac::CorePeripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);

    //let timer = hal::Timer::new(p.TIMER0);
    let mut delay = hal::Delay::new(core.SYST);

    // GPIOs
    let mut digital_pins = Deque::<_, 10>::new();
    digital_pins.push_back(port0.p0_06.degrade()).ok();
    digital_pins.push_back(port0.p0_09.degrade()).ok();
    digital_pins.push_back(port0.p0_10.degrade()).ok();
    digital_pins.push_back(port0.p0_12.degrade()).ok();
    digital_pins.push_back(port0.p0_14.degrade()).ok();
    digital_pins.push_back(port0.p0_15.degrade()).ok();
    digital_pins.push_back(port0.p0_16.degrade()).ok();
    digital_pins.push_back(port0.p0_18.degrade()).ok();
    digital_pins.push_back(port0.p0_20.degrade()).ok();
    digital_pins.push_back(port0.p0_25.degrade()).ok();    

    // let mut analog_pins = [port0.p0_04.degrade(), //AIN2
    //                        port0.p0_05.degrade(), //AIN3
    //                        port0.p0_28.degrade(), //AIN4
    //                        port0.p0_30.degrade()]; //AIN6

    for _ in 0..digital_pins.len() {
        let mut inpin;
        match digital_pins.pop_front() {
            Some(p)  => { inpin = p.into_floating_input(); },
            None => { break; },
        }
        let mut samples : [bool; 5] = [false; 5];
        for i in 0..5 { samples[i] = inpin.is_high().unwrap(); }
        let info = (inpin.pin(), samples);

        delay.delay_ms(50_u16);
        digital_pins.push_back(inpin.into_disconnected()).ok();

    }
    //results:
    // 6: false
    // 9: true
    // 10: true
    // 12: false
    // 14: false
    // 15: false
    // 16: true
    // 18: true
    // 20: true
    // 25: true default, false when button SW1 is pushed
    
    for _ in 0..digital_pins.len() {
        let mut outpin;
        match digital_pins.pop_front() {
            Some(p)  => { outpin = p.into_push_pull_output(Level::Low); },
            None => { break; },
        }

        let info = outpin.pin();

        delay.delay_ms(50_u16);

        for _ in 0..3 {
            outpin.set_high().unwrap();
            delay.delay_ms(150_u16);
            outpin.set_low().unwrap();
            delay.delay_ms(150_u16); 
        }

        // 15: flashes two pairs of red illumination LEDs on top - active high
        //18 : blinks main green LED on back near where switch was - active low
        //20 : blinks main red LED on back near where switch was - active low
        // all others do nothing

        digital_pins.push_back(outpin.into_disconnected()).ok();

    }

    loop {
        delay.delay_ms(500_u16);
    }

    // loop {
    //     for pin in digital_pins.drain() {
    //         let mut inpin = pin.into_floating_input();
    //         let high = inpin.is_high();
    //         delay.delay_ms(50_u16);
    //     }
    //     // for pin in digital_pins.iter_mut() {
    //     //     let mut inpin = pin.into_floating_input();
    //     //     let high = inpin.digital_pinsis_high();
    //     //     delay.delay_ms(50_u16);
    //     // }
    //     // for pin in digital_pins.iter() {
    //     //     let mut led = pin.into_push_pull_output(Level::Low);
    //     //     led.set_high().unwrap();
    //     //     delay.delay_ms(100_u16);
    //     //     led.set_low().unwrap();
    //     //     delay.delay_ms(100_u16);
    //     // }
    //     delay.delay_ms(500_u16);
    //}
}