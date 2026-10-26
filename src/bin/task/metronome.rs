use defmt::{error, info};
use esp_hal::clock;
use esp_hal::gpio::OutputPin;
use esp_hal::ledc::channel::{Channel, ChannelIFace};
use esp_hal::ledc::timer::{Timer, TimerIFace};
use esp_hal::{
    gpio::{AnyPin, DriveMode},
    ledc::{LSGlobalClkSource, Ledc, LowSpeed, channel, timer},
    peripherals::LEDC,
    time::Rate,
};

use crate::modules::state::Metronome;
use crate::task::state::STATE;

pub struct MetronomeConfig<'a> {
    pub buzzer_pin: AnyPin<'a>,
    pub ledc: LEDC<'a>,
}

// https://github.com/esp-rs/esp-hal-community/blob/main/esp-hal-buzzer/src/notes.rs

#[embassy_executor::task]
pub async fn metronome_task(config: MetronomeConfig<'static>) {
    let mut ledc = Ledc::new(config.ledc);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let frequency = Rate::from_hz(261);
    let mut result = 0;
    let mut value = Rate::from_hz(clock::ll::apb_clk_frequency()) / frequency;

    // Limit duty resolution to 14 bits
    while value > 1 && result < 14 {
        value >>= 1;
        result += 1;
    }

    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer1);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::try_from(result).unwrap(),
            clock_source: timer::LSClockSource::APBClk,
            frequency,
        })
        .unwrap();

    let mut channel0 = ledc.channel(channel::Number::Channel0, config.buzzer_pin);
    channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 0,
            drive_mode: DriveMode::PushPull,
        })
        .unwrap();

    let mut receiver = STATE.receiver().unwrap();

    let mut metronome = Metronome::default();

    let mut beat = 1;

    let buzzer = Buzzer::new(config.ledc, config.buzzer_pin);

    loop {
        beat += 1;
        if beat >= metronome.time_signature {
            beat = 1;
        }

        channel0.set_duty(50).ok();
        embassy_time::Timer::after_millis(50).await;

        channel0.set_duty(0).ok();
        let bpm = metronome.bpm as u64;
        let delay = (60 * 1000 - (bpm * 50)) / bpm;
        embassy_time::Timer::after_millis(delay).await;

        if let Some(state) = receiver.try_changed() {
            metronome = state.metronome;
            if !state.metronome.enabled {
                loop {
                    let state = receiver.changed().await;
                    metronome = state.metronome;
                    if state.metronome.enabled {
                        break;
                    }
                }
            }
        }
    }
}

struct Buzzer<'a> {
    timer: Timer<'a, LowSpeed>,
    output_pin: AnyPin<'a>,
}

impl<'a> Buzzer<'a> {
    pub fn new(ledc_peripheral: LEDC<'a>, output_pin: impl OutputPin + 'a) -> Self {
        let mut ledc = Ledc::new(ledc_peripheral);
        ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
        let timer = ledc.timer::<LowSpeed>(timer::Number::Timer1);
        Self {
            timer,
            output_pin: output_pin.degrade(),
        }
    }

    pub fn play(&mut self, frequency: u32) {
        // Mute if frequency is 0Hz
        if frequency == 0 {
            self.mute();
            return;
        }

        // Max duty resolution for a frequency:
        // Integer(log2(LEDC_APB_CKL / frequency))
        let mut result = 0;
        let mut value = Rate::from_hz(clock::ll::apb_clk_frequency()) / Rate::from_hz(frequency);

        // Limit duty resolution to 14 bits
        while value > 1 && result < 14 {
            value >>= 1;
            result += 1;
        }

        self.timer
            .configure(timer::config::Config {
                // Safety: This should never fail because resolution is limited to 14 bits
                duty: timer::config::Duty::try_from(result).unwrap(),
                clock_source: timer::LSClockSource::APBClk,
                frequency: Rate::from_hz(frequency),
            })
            .unwrap();

        let mut channel = Channel::new(channel::Number::Channel0, self.output_pin.reborrow());

        channel
            .configure(channel::config::Config {
                timer: &self.timer,
                // Use volume as duty if set since we use the same channel.
                duty_pct: 50,
                drive_mode: DriveMode::PushPull,
            })
            .unwrap();
    }

    pub fn mute(&mut self) {
        if !self.timer.is_configured() {
            return;
        }
        let mut channel = Channel::new(channel::Number::Channel0, self.output_pin.reborrow());
        channel
            .configure(channel::config::Config {
                timer: &self.timer,
                duty_pct: 0,
                drive_mode: DriveMode::PushPull,
            })
            .unwrap()
    }
}
