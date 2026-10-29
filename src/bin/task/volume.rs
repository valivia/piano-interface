use defmt::info;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::Timer;
use esp_hal::{
    analog::adc::{Adc, AdcCalCurve, AdcConfig, Attenuation},
    peripherals::{ADC1, GPIO1, GPIO5},
};

pub struct VolumeConfig<'a> {
    pub volume_pin: GPIO1<'a>,
    pub adc: ADC1<'a>,
}

const VOLUME_CHANGED: Signal<CriticalSectionRawMutex, u8> = Signal::new();

const CALIBRATION_MAX: usize = 3154;

const TAG: &'static str = "[volume]";

#[embassy_executor::task]
pub async fn task(config: VolumeConfig<'static>) {
    let mut adc1_config = AdcConfig::new();
    let mut pin = adc1_config
        .enable_pin_with_cal::<_, AdcCalCurve<ADC1>>(config.volume_pin, Attenuation::_11dB);

    let mut adc1 = Adc::new(config.adc, adc1_config).into_async();

    // unreachable value, forces loop to send volume at boot
    let mut volume: u8 = 255;

    info!("{} initialized", TAG);

    loop {
        let pin_value: u16 = adc1.read_oneshot(&mut pin).await;
        let new_volume = (pin_value as f32 / CALIBRATION_MAX as f32 * 100.0) as u8;

        if new_volume != volume {
            volume = new_volume;
            VOLUME_CHANGED.signal(new_volume);
            // info!("volume changed: {} - {}", &pin_value, new_volume);
        }

        Timer::after_millis(250).await;
    }
}
