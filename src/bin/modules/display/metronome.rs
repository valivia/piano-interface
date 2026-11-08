use core::fmt::Write;
use embedded_graphics::{
    Drawable,
    geometry::Point,
    text::{Alignment, Text},
};
use heapless::String;

use crate::modules::{
    display::{DisplayRenderer, PageRender, style::STYLE_ACCENT},
    state::metronome::Metronome,
};

impl PageRender for Metronome {
    fn render(&self, renderer: &mut DisplayRenderer) {
        // Graphic

        // Title
        Text::with_alignment(
            "Metronome",
            Point::new(120, 120),
            STYLE_ACCENT.text_title(),
            Alignment::Center,
        )
        .draw(&mut renderer.display)
        .ok();

        // Options
        let value = if self.enabled { "On" } else { "Off" };
        renderer.draw_option("State", value, 150, self.cursor.option_state(0));

        let mut buffer = String::<5>::new();
        write!(buffer, "{}/4", &self.time_signature).ok();
        renderer.draw_option("Measure", &buffer, 175, self.cursor.option_state(1));

        renderer.draw_option("Tempo", &self.bpm, 200, self.cursor.option_state(2));
    }
}
