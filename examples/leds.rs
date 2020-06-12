// Toggles LEDs on PB0 and PB5

#![no_std]
#![no_main]

use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;

use embedded_hal::blocking::delay::DelayMs;
use gd32vf103xx_hal as hal;
use gd32vf103xx_hal::delay::McycleDelay;
use gd32vf103xx_hal::gpio::GpioExt;
use gd32vf103xx_hal::rcu::RcuExt;
use hal::pac;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.configure().freeze();

    let mut delay = McycleDelay::new(&rcu.clocks);
    let mut gpiob = dp.GPIOB.split(&mut rcu);
    let mut output_pb5 = gpiob.pb5.into_push_pull_output();
    let mut output_pb0 = gpiob.pb0.into_push_pull_output();
    loop {
        output_pb5.set_high().unwrap();
        output_pb0.set_high().unwrap();
        delay.delay_ms(500);
        output_pb5.set_low().unwrap();
        output_pb0.set_low().unwrap();
        delay.delay_ms(500);
    }
}
