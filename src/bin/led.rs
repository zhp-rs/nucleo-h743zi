#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_itm;
use cortex_m;
use cortex_m_rt::entry;
use stm32h7xx_hal::hal::digital::v2::OutputPin;
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

    let mut red = gpiob.pb14.into_push_pull_output();

    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        loop {
            red.set_high().unwrap();
            delay.delay_ms(500_u16);
            red.set_low().unwrap();
            delay.delay_ms(500_u16);
        }
    }
}
