use std::thread::sleep;
use std::time::Duration;
use embedded_svc::http::Method;
use embedded_svc::http::server::{Handler, HandlerResult};
use embedded_svc::wifi::{ClientConfiguration, Configuration, Wifi};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{EspWifi};
use esp_idf_sys as _;
use esp_idf_hal::{
    peripherals::Peripherals,
};
use esp_idf_svc::http::server;
use esp_idf_svc::http::server::{EspHttpConnection, EspHttpServer};
use esp_idf_sys::HttpStatus_Code_HttpStatus_Ok;
use indoc::indoc;
use log::info; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}


fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    log::info!("Connect to {}", CONFIG.wifi_ssid);


    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();


    // let mut storage = create_network_stack_storage!(3, 8, 1);
    let mut wifi_driver = EspWifi::new(
        peripherals.modem,
        sys_loop,
        Some(nvs)
    ).unwrap();


    wifi_driver.set_configuration(&Configuration::Client(ClientConfiguration{
        ssid: CONFIG.wifi_ssid.into(),
        password: CONFIG.wifi_psk.into(),
        ..Default::default()
    })).unwrap();

    wifi_driver.start().unwrap();
    wifi_driver.connect().unwrap();
    while !wifi_driver.is_connected().unwrap(){
        let config = wifi_driver.get_configuration().unwrap();
        info!("Waiting for station {:?}", config);
    }
    info!("Should be connected now");
    info!("IP info: {:?}", wifi_driver.sta_netif().get_ip_info().unwrap());
    sleep(Duration::from_secs(10));
    info!("IP info: {:?}", wifi_driver.sta_netif().get_ip_info().unwrap());
    info!("Starting server");

    let mut server = EspHttpServer::new(&server::Configuration {
        http_port: 80,
        ..Default::default()
    }).unwrap();

    server.handler("/", Method::Get, CrabbyHandler).unwrap();

    loop {}


    Ok(())
}

struct CrabbyHandler;

impl Handler<EspHttpConnection<'_>> for CrabbyHandler {
    fn handle(&self, connection: &mut EspHttpConnection<'_>) -> HandlerResult {
        info!("Hello ESP Rust 🦀 world!");
        connection.initiate_response(HttpStatus_Code_HttpStatus_Ok as u16, None, &[]).unwrap();
        connection.write(indoc! {"
            <!DOCTYPE html>
            <html lang=\"en\">
              <head>
                <meta charset=\"utf-8\">
                <title>🦀</title>
              </head>
              <body>
                Hello ESP Rust 🦀 world!
              </body>
            </html>
        "}.as_bytes()).unwrap();
        Ok(())
    }
}
