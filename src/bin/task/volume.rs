use defmt::info;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Instant, Timer};
use esp_hal::{
    analog::adc::{Adc, AdcCalCurve, AdcConfig, Attenuation},
    peripherals::{ADC1, GPIO1},
};

pub struct VolumeConfig<'a> {
    pub volume_pin: GPIO1<'a>,
    pub adc: ADC1<'a>,
}

pub static VOLUME: Signal<CriticalSectionRawMutex, u8> = Signal::new();

const CALIBRATION_MAX: usize = 2800; // 3154

const TAG: &'static str = "[volume]";

#[embassy_executor::task]
pub async fn task(config: VolumeConfig<'static>) {
    let mut adc1_config = AdcConfig::new();
    let mut pin = adc1_config
        .enable_pin_with_cal::<_, AdcCalCurve<ADC1>>(config.volume_pin, Attenuation::_11dB);

    let mut adc1 = Adc::new(config.adc, adc1_config).into_async();

    // unreachable value, forces loop to send volume at boot
    let mut volume: u8 = 255;
    let mut last_reading: u16 = u16::MAX;
    let mut last_big_change = Instant::now();
    info!("{} initialized", TAG);

    loop {
        let pin_value: u16 = adc1.read_oneshot(&mut pin).await;
        let difference = pin_value.abs_diff(last_reading);
        let new_volume = u8::clamp(
            (pin_value as f32 / CALIBRATION_MAX as f32 * 100.0) as u8,
            0,
            100,
        );

        if new_volume.abs_diff(volume) > 10 || volume == 255 {
            last_big_change = Instant::now();
        }

        if difference > 40 && Instant::now().duration_since(last_big_change).as_millis() < 1200 {
            VOLUME.signal(new_volume);
        }

        volume = new_volume;
        last_reading = pin_value;
        Timer::after_millis(100).await;
    }
}
