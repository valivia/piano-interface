use embedded_graphics::{
    Drawable,
    geometry::Point,
    text::{Alignment, Text},
};

use crate::modules::{
    display::{CHAR_LARGE, DisplayRenderer},
    state::State,
};

impl DisplayRenderer {
    pub fn draw_demo_page(&mut self, state: &State) {
        Text::with_alignment(
            "Metronome",
            Point::new(120, 90),
            CHAR_LARGE,
            Alignment::Center,
        )
        .draw(&mut self.display)
        .ok();

        Text::with_alignment(
            state.demo.active_song,
            Point::new(120, 120),
            CHAR_LARGE,
            Alignment::Center,
        )
        .draw(&mut self.display)
        .ok();
    }
}
