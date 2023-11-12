#![no_std]
#![no_main]

mod devices;

use devices::DEVICES;

#[no_mangle]
fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let ble_device = esp32_nimble::BLEDevice::take();
    let ble_advertising = ble_device.get_advertising();

    for (name, data) in DEVICES.iter().cycle() {
        log::info!("{name}");
        ble_advertising.custom_adv_data(data).unwrap();
        ble_advertising.start().unwrap();
        esp_idf_hal::delay::FreeRtos::delay_ms(1000);
        ble_advertising.stop().unwrap();
    }
}
