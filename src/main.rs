#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use arduino_hal::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main()->!
{
    let dp = arduino_hal::Peripherals::take().unwrap();
    
    let pins = arduino_hal::pins!(dp);
    
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
    
    let mut led = pins.d9.into_output();

    loop {
        led.set_high();
        delay_ms(200);
        ufmt::uwriteln!(&mut serial, "Hello From Arduino\r").void_unwrap();
        led.set_low();
        delay_ms(200);
    }
}
