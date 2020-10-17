#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_itm;

use cortex_m;
use cortex_m_rt::entry;
use stm32h7xx_hal::hal::digital::v2::ToggleableOutputPin;
use stm32h7xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let pwr = dp.PWR.constrain();
    let vos = pwr.freeze();

    let rcc = dp.RCC.constrain();
    let mut ccdr = rcc.sys_ck(100.mhz()).freeze(vos, &dp.SYSCFG);

    let gpiob = dp.GPIOB.split(&mut ccdr.ahb4);
    let mut delay = cp.SYST.delay(ccdr.clocks);

    let mut red = gpiob.pb14.into_push_pull_output();
    let mut blue = gpiob.pb7.into_push_pull_output();
    let mut yellow = gpiob.pb0.into_push_pull_output();

    loop {
        red.toggle().unwrap();
        delay.delay_ms(500_u16);
        red.toggle().unwrap();

        blue.toggle().unwrap();
        delay.delay_ms(500_u16);
        blue.toggle().unwrap();

        yellow.toggle().unwrap();
        delay.delay_ms(500_u16);
        yellow.toggle().unwrap();
    }
}
