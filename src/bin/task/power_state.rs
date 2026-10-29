use defmt::info;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use esp_hal::gpio::{AnyPin, Level, Output, OutputConfig};

use crate::task::button::POWER_BUTTON;

pub struct PowerStateConfig<'a> {
    pub led_pin: AnyPin<'a>,
}

pub static POWER_STATE: Watch<CriticalSectionRawMutex, bool, 3> = Watch::new_with(false);

const TAG: &'static str = "[power_state]";

#[embassy_executor::task]
pub async fn task(config: PowerStateConfig<'static>) {
    let mut indicator_led = Output::new(config.led_pin, Level::Low, OutputConfig::default());

    let sender = POWER_STATE.sender();

    let mut is_on = false;

    info!("{} initialized", TAG);

    loop {
        let _press_type = POWER_BUTTON.wait().await;
        is_on = !is_on;
        info!("{} Device turned {}", TAG, if is_on { "on" } else { "off" });
        indicator_led.set_level(is_on.into());
        sender.send(is_on);
    }
}
