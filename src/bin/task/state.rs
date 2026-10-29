use defmt::info;
use embassy_futures::select::{Either3, select3};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};

use crate::{
    modules::state::State,
    task::{button::BUTTON_EVENTS, power_state::POWER_STATE, rotary_encoder::ROTARY_STEPS},
};

pub static STATE: Watch<CriticalSectionRawMutex, State, 2> = Watch::new();

const TAG: &'static str = "[state_task]";

#[embassy_executor::task]
pub async fn task() {
    let encoder = ROTARY_STEPS.receiver();
    let button = BUTTON_EVENTS.receiver();
    let mut power_state = POWER_STATE.receiver().unwrap();

    let mut state = State::default();
    let sender = STATE.sender();
    sender.send(state);

    info!("{} initialized", TAG);

    loop {
        match select3(encoder.receive(), button.receive(), power_state.changed()).await {
            Either3::First(delta) => state.encoder_event(delta),
            Either3::Second(event) => state.button_event(event),
            Either3::Third(power_state) => state.power_state_event(power_state),
        }

        sender.send(state);
    }
}
