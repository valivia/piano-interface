use core::{cell::RefCell, cmp::min};

use critical_section::Mutex;
use defmt::{debug, info};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, watch::Watch};
use embassy_time::Timer;
use esp_hal::{
    gpio::{AnyPin, Input, InputConfig, Pull},
    handler,
    interrupt::Priority,
    pcnt::{Pcnt, channel, unit},
    peripherals::PCNT,
};

static UNIT0: Mutex<RefCell<Option<unit::Unit<'static, 1>>>> = Mutex::new(RefCell::new(None));

pub static ROTARY_STEPS: Channel<CriticalSectionRawMutex, i8, 8> = Channel::new();

const COUNTS_PER_DETENT: i16 = 4;

pub struct RotaryEncoderConfig<'a> {
    pub pcnt: PCNT<'a>,
    pub s1_pin: AnyPin<'a>,
    pub s2_pin: AnyPin<'a>,
}

const TAG: &'static str = "[encoder]";

#[embassy_executor::task]
pub async fn rotary_encoder_task(config: RotaryEncoderConfig<'static>) {
    debug!("{} initializing", TAG);

    let mut pcnt = Pcnt::new(config.pcnt);
    pcnt.set_interrupt_handler(interrupt_handler);

    let u0 = pcnt.unit1;
    u0.set_low_limit(None).unwrap();
    u0.set_high_limit(None).unwrap();
    u0.set_filter(Some(min(10u16 * 80, 1023u16))).unwrap();
    u0.clear();

    // Pins
    let input_cfg = InputConfig::default().with_pull(Pull::Up);
    let pin_a = Input::new(config.s1_pin, input_cfg);
    let pin_b = Input::new(config.s2_pin, input_cfg);
    let input_a = pin_a.peripheral_input();
    let input_b = pin_b.peripheral_input();

    // Set up channels with control and edge signals
    let ch0 = &u0.channel0;
    ch0.set_ctrl_signal(input_a.clone());
    ch0.set_edge_signal(input_b.clone());
    ch0.set_ctrl_mode(channel::CtrlMode::Reverse, channel::CtrlMode::Keep);
    ch0.set_input_mode(channel::EdgeMode::Increment, channel::EdgeMode::Decrement);

    let ch1 = &u0.channel1;
    ch1.set_ctrl_signal(input_b);
    ch1.set_edge_signal(input_a);
    ch1.set_ctrl_mode(channel::CtrlMode::Reverse, channel::CtrlMode::Keep);
    ch1.set_input_mode(channel::EdgeMode::Decrement, channel::EdgeMode::Increment);

    // Enable interrupts and resume pulse counter unit
    u0.listen();
    u0.resume();
    let counter = u0.counter.clone();

    critical_section::with(|cs| UNIT0.borrow_ref_mut(cs).replace(u0));

    let mut last_raw: i16 = 0;
    let mut remainder: i16 = 0;

    info!("{} initialized", TAG);

    loop {
        Timer::after_millis(5).await;

        let raw = counter.get();
        let d = raw.wrapping_sub(last_raw);
        if d == 0 {
            continue;
        }
        last_raw = raw;
        remainder += d;

        let steps = remainder / COUNTS_PER_DETENT;
        if steps == 0 {
            continue;
        }

        remainder -= steps * COUNTS_PER_DETENT;

        let dir = steps.signum() as i8;
        for _ in 0..steps.unsigned_abs() {
            let _ = ROTARY_STEPS.try_send(dir);
        }
    }
}

#[handler(priority = Priority::Priority2)]
fn interrupt_handler() {
    critical_section::with(|cs| {
        let mut u0 = UNIT0.borrow_ref_mut(cs);
        let u0 = u0.as_mut().unwrap();
        if u0.interrupt_is_set() {
            u0.reset_interrupt();
        }
    });
}
