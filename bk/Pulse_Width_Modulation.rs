#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use arduino_hal::simple_pwm::*;
use panic_halt as _;



#[arduino_hal::entry]
fn main()->!
{
    let dp = arduino_hal::Peripherals::take().unwrap();
    
    let pins = arduino_hal::pins!(dp);
    
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
    ufmt::uwriteln!(&mut serial, "Hello From Arduino\r").void_unwrap();

    //Chip arduno Uno 328p run at 8Mhz (default in hardware), divider prescaler 64 --> 8,000Khz/64 = 125Khz
    //Chip arduino uno 328p has 3 time counter as the datasheet
    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

    // Use digital pin 5,6 is connected to a LED and a resistor in series for only timer0
    let mut pwm_led = pins.d5.into_output().into_pwm(&timer0);
    pwm_led.enable();// Enable it to run
    // Use digital pin 9,10 is connected to a LED and a resistor in series for only timer1
    let mut pwm_led1 = pins.d9.into_output().into_pwm(&timer1);
    pwm_led1.enable();
    // Use digital pin 3,11 is connected to a LED and a resistor in series for only timer1
    let mut pwm_led2 = pins.d3.into_output().into_pwm(&mut timer2);
    pwm_led2.enable();

    loop 
    {
        for x in (0..=255).chain((0..=254).rev()) {
            pwm_led.set_duty(x);
            pwm_led1.set_duty(x);
            pwm_led2.set_duty(x);
            arduino_hal::delay_ms(5);
        }
    }
}
