#![no_std]
#![no_main]

mod devices;

use esp32_nimble::enums::{ConnMode, OwnAddrType};
use esp_idf_svc::sys::{esp_fill_random, random};
use devices::DEVICES;

fn random_addr() -> [u8; 6] {
    let mut addr = [0u8, 0, 0, 0, 0, 0];
    unsafe { esp_fill_random(addr.as_mut_ptr() as *mut core::ffi::c_void, 6) };
    addr
}

fn random_mode() -> ConnMode {
    match unsafe { random() } % 2 {
        0 => ConnMode::Non,
        _ => ConnMode::Und,
    }
}

#[no_mangle]
fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let ble_device = esp32_nimble::BLEDevice::take();
    ble_device.set_own_addr_type(OwnAddrType::Random);
    let ble_advertising = ble_device.get_advertising();

    for (name, data) in DEVICES.iter().cycle() {
        log::info!("{name}");
        let _ = ble_device.set_rnd_addr(random_addr());
        ble_advertising.advertisement_type(random_mode());
        ble_advertising.custom_adv_data(data).unwrap();
        ble_advertising.start().unwrap();
        esp_idf_hal::delay::FreeRtos::delay_ms(1000);
        ble_advertising.stop().unwrap();
    }
}
