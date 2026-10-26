use defmt::info;
use embassy_futures::select::{Either6, select6};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use embassy_time::Timer;
use esp_hal::gpio::{AnyPin, Input, InputConfig, Pull};

use crate::{
    modules::state::{Page, State},
    task::rotary_encoder::ROTARY_DELTA,
};

pub static STATE: Watch<CriticalSectionRawMutex, State, 2> = Watch::new();

pub struct StateConfig<'a> {
    pub on_off_pin: AnyPin<'a>,
    pub click_pin: AnyPin<'a>,
    pub demo_pin: AnyPin<'a>,
    pub metronome_pin: AnyPin<'a>,
    pub reverb_pin: AnyPin<'a>,
}

#[embassy_executor::task]
pub async fn task(config: StateConfig<'static>) {
    let input_config = InputConfig::default().with_pull(Pull::Up);
    let mut click_input = Input::new(config.click_pin, input_config);
    let mut demo_input = Input::new(config.demo_pin, input_config);
    let mut metronome_input = Input::new(config.metronome_pin, input_config);
    let mut reverb_input = Input::new(config.reverb_pin, input_config);
    let mut on_off_input = Input::new(config.on_off_pin, input_config);

    let mut encoder = ROTARY_DELTA.receiver().unwrap();

    let mut state = State::default();
    let sender = STATE.sender();
    loop {
        match select6(
            on_off_input.wait_for_falling_edge(),
            demo_input.wait_for_falling_edge(),
            metronome_input.wait_for_falling_edge(),
            reverb_input.wait_for_falling_edge(),
            click_input.wait_for_falling_edge(),
            encoder.changed(),
        )
        .await
        {
            Either6::First(_) => info!("Clicked on/off"),
            Either6::Second(_) => {
                info!("Clicked demo");
                state.demo.enabled = !state.demo.enabled;
                sender.send(state);
            }
            Either6::Third(_) => {
                info!("Clicked metronome");
                state.metronome.enabled = !state.metronome.enabled;
                sender.send(state);
            }
            Either6::Fourth(_) => {
                info!("Clicked reverb");
                state.reverb.enabled = !state.reverb.enabled;
                sender.send(state);
            }
            Either6::Fifth(_) => info!("Clicked"),
            Either6::Sixth(delta) => info!("{}", delta),
        }

        Timer::after_millis(250).await;
    }
}
