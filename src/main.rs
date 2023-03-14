

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported


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

    log::info!("Connect to {}", CONFIG.wifi_ssid);

    // let _wifi = wifi(CONFIG.wifi_ssid, CONFIG.wifi_psk)?;

    // TODO your code here
    //get(...)?;

    Ok(())
}
