use embedded_graphics::{
    Drawable,
    geometry::Point,
    text::{Alignment, Text},
};

use crate::modules::{
    display::{DisplayRenderer, PageRender, TEXT_DEFAULT, TEXT_LARGE},
    state::instrument::Instrument,
};

impl PageRender for Instrument {
    fn render(&self, renderer: &mut DisplayRenderer) {
        Text::with_alignment(
            self.active_instrument(),
            Point::new(120, 120),
            TEXT_LARGE,
            Alignment::Center,
        )
        .draw(&mut renderer.display)
        .ok();

        if self.is_selecting() {
            Text::with_alignment(
                self.selected_instrument(),
                Point::new(120, 130),
                TEXT_DEFAULT,
                Alignment::Center,
            )
            .draw(&mut renderer.display)
            .ok();
        }
    }
}
