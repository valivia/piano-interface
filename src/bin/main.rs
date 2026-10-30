#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_println::{self as _, println};

use crate::task::button::{ButtonId, button_task};
use crate::task::display::DisplayConfig;
use crate::task::metronome::MetronomeConfig;
use crate::task::power_state::PowerStateConfig;
use crate::task::rotary_encoder::RotaryEncoderConfig;
use esp_hal::gpio::Pin;

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    println!("{:#?}", panic_info.message());
    error!("{}", panic_info);
    loop {}
}

extern crate alloc;

mod modules;
mod task;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.4.0
    // generator parameters: -o esp32s3 -o unstable-hal -o alloc -o embassy -o defmt -o zed -o vscode -o esp32s3-mini-1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // The following pins are used to bootstrap the chip. They are available
    // for use, but check the datasheet of the module for more information on them.
    // - GPIO0
    // - GPIO3
    // - GPIO45
    // - GPIO46
    // These GPIO pins are in use by some feature of the module and should not be used.
    let _gpio27 = peripherals.GPIO27;
    let _gpio28 = peripherals.GPIO28;
    let _gpio29 = peripherals.GPIO29;
    let _gpio30 = peripherals.GPIO30;
    let _gpio31 = peripherals.GPIO31;
    let _gpio32 = peripherals.GPIO32;

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);

    info!("Embassy initialized!");

    // TODO: Spawn some tasks
    let _ = spawner;

    // Power state
    spawner.spawn(
        task::power_state::task(PowerStateConfig {
            led_pin: peripherals.GPIO8.into(),
        })
        .unwrap(),
    );
    // State
    spawner.spawn(task::state::task().unwrap());

    // Display
    spawner.spawn(
        task::display::display_task(DisplayConfig {
            spi: peripherals.SPI2,
            clock: peripherals.GPIO34.into(),
            data_out: peripherals.GPIO33.into(),
            reset: peripherals.GPIO18.into(),
            dc: peripherals.GPIO17.into(),
            cs: peripherals.GPIO16.into(),
            backlight: peripherals.GPIO15.into(),
        })
        .unwrap(),
    );

    // Metronome
    spawner.spawn(
        task::metronome::metronome_task(MetronomeConfig {
            buzzer_pin: peripherals.GPIO12.into(),
            ledc: peripherals.LEDC,
        })
        .unwrap(),
    );

    // Rotary encoder
    spawner.spawn(
        task::rotary_encoder::rotary_encoder_task(RotaryEncoderConfig {
            pcnt: peripherals.PCNT,
            s1_pin: peripherals.GPIO38.into(),
            s2_pin: peripherals.GPIO37.into(),
        })
        .unwrap(),
    );

    // Buttons
    spawner.spawn(button_task(ButtonId::OnOff, peripherals.GPIO7.degrade()).unwrap());
    spawner.spawn(button_task(ButtonId::Click, peripherals.GPIO36.degrade()).unwrap());
    spawner.spawn(button_task(ButtonId::Demo, peripherals.GPIO2.degrade()).unwrap());
    spawner.spawn(button_task(ButtonId::Reverb, peripherals.GPIO4.degrade()).unwrap());
    spawner.spawn(button_task(ButtonId::Metronome, peripherals.GPIO5.degrade()).unwrap());

    // Volume
    spawner.spawn(
        task::volume::task(task::volume::VolumeConfig {
            volume_pin: peripherals.GPIO1.into(),
            adc: peripherals.ADC1,
        })
        .unwrap(),
    );

    loop {
        Timer::after(Duration::from_millis(100)).await;
    }
}
