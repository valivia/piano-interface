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
    pub fn draw_instrument_page(&mut self, state: &State) {
        Text::with_alignment(
            state.active_instrument(),
            Point::new(120, 120),
            CHAR_LARGE,
            Alignment::Center,
        )
        .draw(&mut self.display)
        .ok();

        self.draw_status_icons(state);
    }
}
