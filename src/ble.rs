use crate::devices::DEVICES;
use core::sync::atomic::{AtomicBool, Ordering};
use esp32_nimble::enums::{ConnMode, DiscMode, OwnAddrType};
use esp_idf_svc::sys::{esp_fill_random, random};

pub static START: AtomicBool = AtomicBool::new(true);

pub fn start() {
    START.store(true, Ordering::Relaxed)
}

pub fn stop() {
    START.store(false, Ordering::Relaxed)
}

fn random_addr() -> [u8; 6] {
    let mut addr = [0u8; 6];
    addr[0] = unsafe { random() as u8 } % 64 + 192; // 192-255
    unsafe { esp_fill_random(addr[1..6].as_mut_ptr() as *mut core::ffi::c_void, 5) };
    log::info!("{addr:?}");
    addr
}

fn random_mode() -> (ConnMode, DiscMode) {
    match unsafe { random() } % 3 {
        0 => (ConnMode::Non, DiscMode::Non),
        1 => (ConnMode::Non, DiscMode::Gen),
        _ => (ConnMode::Und, DiscMode::Non),
    }
}

pub async fn ble_loop(timer: &mut esp_idf_hal::timer::TimerDriver<'_>) {
    let ble_device = esp32_nimble::BLEDevice::take();
    ble_device.set_own_addr_type(OwnAddrType::Random);
    let ble_advertising = ble_device.get_advertising();

    for (name, data) in DEVICES.iter().cycle() {
        if START.load(Ordering::Relaxed) {
            log::info!("{name}");
            let _ = ble_device.set_rnd_addr(random_addr());
            let (conn_mode, disc_mode) = random_mode();
            ble_advertising.advertisement_type(conn_mode);
            ble_advertising.disc_mode(disc_mode);
            ble_advertising.custom_adv_data(data).unwrap();
        }
        timer.delay(timer.tick_hz()).await.unwrap();
        ble_advertising.stop().unwrap();
    }
}
