#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32f4xx_hal::{prelude::*, stm32};
use stm32f4xx_hal::usb::{Peripheral, UsbBus};
use usb_device::prelude::*;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let _clocks = rcc
        .cfgr
        .use_hse(25.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .require_pll48clk()
        .freeze();

    let gpioa = dp.GPIOA.split();

    let usb = Peripheral {
        usb_global: dp.OTG_FS_GLOBAL,
        usb_device: dp.OTG_FS_DEVICE,
        usb_pwrclk: dp.OTG_FS_PWRCLK,
        pin_dm: gpioa.pa11.into_alternate_af10(),
        pin_dp: gpioa.pa12.into_alternate_af10(),
    };

    let usb_bus = UsbBus::new(usb, unsafe { &mut EP_MEMORY });

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Enumeration test")
        .serial_number("TEST")
        .device_class(0)
        .build();

    loop {
        if usb_dev.poll(&mut []) {
        }
    }
}
