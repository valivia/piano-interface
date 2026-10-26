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

use crate::task::display::DisplayConfig;
use crate::task::metronome::MetronomeConfig;
use crate::task::rotary_encoder::RotaryEncoderConfig;
use crate::task::state::StateConfig;
use crate::task::volume::VolumeConfig;

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

    // State
    spawner.spawn(
        task::state::task(StateConfig {
            on_off_pin: peripherals.GPIO4.into(),
            demo_pin: peripherals.GPIO39.into(),
            metronome_pin: peripherals.GPIO40.into(),
            reverb_pin: peripherals.GPIO41.into(),
            click_pin: peripherals.GPIO9.into(),
        })
        .unwrap(),
    );

    // Display
    spawner.spawn(
        task::display::display_task(DisplayConfig {
            spi: peripherals.SPI2,
            clock: peripherals.GPIO35.into(),
            data_out: peripherals.GPIO36.into(),
            reset: peripherals.GPIO37.into(),
            dc: peripherals.GPIO13.into(),
            cs: peripherals.GPIO14.into(),
            backlight: peripherals.GPIO15.into(),
        })
        .unwrap(),
    );

    // Metronome
    spawner.spawn(
        task::metronome::metronome_task(MetronomeConfig {
            buzzer_pin: peripherals.GPIO17.into(),
            ledc: peripherals.LEDC,
        })
        .unwrap(),
    );

    // Rotary encoder
    spawner.spawn(
        task::rotary_encoder::rotary_encoder_task(RotaryEncoderConfig {
            pcnt: peripherals.PCNT,
            s1_pin: peripherals.GPIO7.into(),
            s2_pin: peripherals.GPIO8.into(),
        })
        .unwrap(),
    );

    spawner.spawn(
        task::volume::task(VolumeConfig {
            volume_pin: peripherals.GPIO5.into(),
            adc: peripherals.ADC1,
        })
        .unwrap(),
    );

    loop {
        Timer::after(Duration::from_millis(100)).await;
    }
}
