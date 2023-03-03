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
    
    let mut serial = default_serial!(dp, pins, 115200);

    loop 
    {
        delay_ms(1000);
        ufmt::uwriteln!(&mut serial, "Hello From Arduino\r").void_unwrap();
    }
}