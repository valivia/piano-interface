use defmt::{debug, info};
use embassy_time::Timer;

use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{
    Async,
    delay::Delay,
    gpio::{AnyPin, Level, Output, OutputConfig},
    peripherals::SPI2,
    spi::master::{Config, Spi},
    time::Rate,
};
use gc9a01::{
    Gc9a01, SPIDisplayInterface,
    display::DisplayResolution240x240,
    mode::{BufferedGraphics, DisplayConfiguration},
    prelude::SPIInterface,
    rotation::DisplayRotation,
};

use crate::{modules::display::DisplayRenderer, task::state::STATE};

pub type Display<'a> = Gc9a01<
    SPIInterface<ExclusiveDevice<Spi<'a, Async>, Output<'a>, Delay>, Output<'a>>,
    DisplayResolution240x240,
    BufferedGraphics<DisplayResolution240x240>,
>;

pub struct DisplayConfig<'a> {
    pub spi: SPI2<'a>,
    pub clock: AnyPin<'a>,
    pub data_out: AnyPin<'a>,
    pub reset: AnyPin<'a>,
    pub dc: AnyPin<'a>,
    pub cs: AnyPin<'a>,
    pub backlight: AnyPin<'a>,
}

const TAG: &'static str = "[display]";

#[embassy_executor::task]
pub async fn display_task(config: DisplayConfig<'static>) {
    debug!("{} initializing", TAG);

    let cs = Output::new(config.cs, Level::High, OutputConfig::default());
    let dc = Output::new(config.dc, Level::High, OutputConfig::default());
    let mut reset = Output::new(config.reset, Level::High, OutputConfig::default());
    let mut backlight = Output::new(config.backlight, Level::High, OutputConfig::default());

    let mut delay = Delay::new();
    let spi_config = Config::default().with_frequency(Rate::from_mhz(40));
    let spi = Spi::new(config.spi, spi_config)
        .unwrap()
        .with_mosi(config.data_out)
        .with_sck(config.clock)
        .into_async();

    let spi_device = ExclusiveDevice::new(spi, cs, Delay::new()).unwrap();
    let interface = SPIDisplayInterface::new(spi_device, dc);

    let mut display = Gc9a01::new(
        interface,
        DisplayResolution240x240,
        DisplayRotation::Rotate0,
    )
    .into_buffered_graphics();

    display.reset(&mut reset, &mut delay).ok();
    display.init(&mut delay).unwrap();
    backlight.set_high();
    display.clear();
    display.flush().ok();

    let mut renderer = DisplayRenderer {
        display,
        previous_state: None,
    };

    let mut state_receiver = STATE.receiver().unwrap();

    info!("{} initialized", TAG);

    loop {
        let state = state_receiver.try_changed();
        renderer.render(&state);
        Timer::after_millis(40).await;
    }
}
