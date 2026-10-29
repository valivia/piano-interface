use defmt::Format;
use embassy_futures::select::{Either, select};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, signal::Signal,
};
use embassy_time::{Duration, Timer};
use esp_hal::gpio::{AnyPin, Input, InputConfig, Pull};

#[derive(Clone, Copy, PartialEq, Eq, Format)]
pub enum ButtonId {
    OnOff,
    Demo,
    Metronome,
    Reverb,
    Click,
}

#[derive(Clone, Copy, PartialEq, Eq, Format)]
pub enum ButtonPressType {
    Short,
    Long,
}

#[derive(Clone, Copy, Format)]
pub struct ButtonEvent {
    pub id: ButtonId,
    pub press_type: ButtonPressType,
}

pub static BUTTON_EVENTS: Channel<CriticalSectionRawMutex, ButtonEvent, 8> = Channel::new();
pub static POWER_BUTTON: Signal<CriticalSectionRawMutex, ButtonPressType> = Signal::new();

const DEBOUNCE: Duration = Duration::from_millis(20);
const LONG_PRESS: Duration = Duration::from_millis(600);

#[embassy_executor::task(pool_size = 5)]
pub async fn button_task(id: ButtonId, pin: AnyPin<'static>) {
    let mut input = Input::new(pin, InputConfig::default().with_pull(Pull::Up));

    loop {
        input.wait_for_low().await;

        // Debounce
        Timer::after(DEBOUNCE).await;
        if input.is_high() {
            continue;
        }

        // Short or long press
        let press_type = match select(input.wait_for_high(), Timer::after(LONG_PRESS)).await {
            Either::First(_) => ButtonPressType::Short,
            Either::Second(_) => ButtonPressType::Long,
        };

        // Long presses fire while still held, so the UI can react immediately
        BUTTON_EVENTS.send(ButtonEvent { id, press_type }).await;
        if id == ButtonId::OnOff {
            POWER_BUTTON.signal(press_type);
        }

        // Wait for release (returns instantly if already released), then debounce it
        input.wait_for_high().await;
        Timer::after(DEBOUNCE).await;
    }
}
