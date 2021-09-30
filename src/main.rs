#![no_main]
#![no_std]


use defmt_rtt as _; // global logger
use nrf52840_hal as _; // memory layout

use nrf52840_hal::pac::Peripherals;
use nrf52840_hal::temp::Temp;


pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    defmt::error!("panicked");
    exit()
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // Take ownership of the device peripherals
    let peripherals = Peripherals::take().unwrap();

    // Access to the temp sensor
    let mut temp_sensor = Temp::new(peripherals.TEMP);

    let die_temp_c: i32 = temp_sensor.measure().to_num();
    defmt::info!("processor temp is {:i32}°C", die_temp_c);

    exit();
}
