#![feature(ip_in_core)]
#![no_std]
#![no_main]
extern crate alloc;

mod ble;
mod devices;
mod http;

use crate::ble::ble_loop;
use crate::http::start_http_server;
use embassy_futures::select::select;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::sys::EspError;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::timer::EspTaskTimerService;
use esp_idf_svc::wifi::{AsyncWifi, AuthMethod, EspWifi};

#[no_mangle]
fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
    esp_idf_hal::task::block_on(run()).unwrap();
}

async fn run() -> Result<(), EspError> {
    let peripherals = Peripherals::take()?;

    let sys_loop = EspSystemEventLoop::take()?;
    let timer_service = EspTaskTimerService::new()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let mut wifi = AsyncWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
        timer_service,
    )?;

    let wifi_configuration = esp_idf_svc::wifi::Configuration::AccessPoint(
        esp_idf_svc::wifi::AccessPointConfiguration {
            ssid: "esp32".into(),
            password: "".into(),
            auth_method: AuthMethod::None,
            channel: 1,
            max_connections: 255,
            ..Default::default()
        },
    );

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start().await?;
    log::info!("Wifi started");

    wifi.wait_netif_up().await?;
    log::info!("Wifi netif up");
    log::info!("{:?}", wifi.wifi().ap_netif().get_ip_info());

    let mut timer = esp_idf_hal::timer::TimerDriver::new(
        peripherals.timer00,
        &esp_idf_hal::timer::TimerConfig::new(),
    )?;
    timer.delay(timer.tick_hz()).await.unwrap();
    select(start_http_server(), ble_loop(&mut timer)).await;
    Ok(())
}
