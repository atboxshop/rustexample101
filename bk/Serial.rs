#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;



#[arduino_hal::entry]
fn main()->!
{
    let dp = arduino_hal::Peripherals::take().unwrap();
    
    let pins = arduino_hal::pins!(dp);
    
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);

    loop 
    {
        arduino_hal::delay_ms(5000);
        ufmt::uwriteln!(&mut serial, "Hello From Arduino\r").void_unwrap();
    }
}