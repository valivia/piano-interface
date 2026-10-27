use core::fmt::Write;
use embedded_graphics::{
    Drawable,
    geometry::Point,
    text::{Alignment, Text},
};
use heapless::String;

use crate::modules::{
    display::{CHAR_LARGE, DisplayRenderer},
    state::State,
};

impl DisplayRenderer {
    pub fn draw_metronome_page(&mut self, state: &State) {
        let mut text = String::<8>::new();

        Text::with_alignment(
            "Metronome",
            Point::new(120, 90),
            CHAR_LARGE,
            Alignment::Center,
        )
        .draw(&mut self.display)
        .ok();

        write!(text, "{}", state.metronome.bpm).ok();
        Text::with_alignment(&text, Point::new(120, 120), CHAR_LARGE, Alignment::Center)
            .draw(&mut self.display)
            .ok();

        text.clear();
        write!(text, "{}", state.metronome.time_signature).ok();
        Text::with_alignment(&text, Point::new(120, 140), CHAR_LARGE, Alignment::Center)
            .draw(&mut self.display)
            .ok();
    }
}
