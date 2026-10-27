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
    pub fn draw_reverb_page(&mut self, state: &State) {
        let mut text = String::<8>::new();

        Text::with_alignment("Reverb", Point::new(120, 90), CHAR_LARGE, Alignment::Center)
            .draw(&mut self.display)
            .ok();

        Text::with_alignment(
            state.reverb.reverb_type.name(),
            Point::new(120, 120),
            CHAR_LARGE,
            Alignment::Center,
        )
        .draw(&mut self.display)
        .ok();

        write!(text, "{}", state.reverb.reverb_depth).ok();
        Text::with_alignment(&text, Point::new(120, 140), CHAR_LARGE, Alignment::Center)
            .draw(&mut self.display)
            .ok();
    }
}
