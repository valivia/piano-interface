use embedded_graphics::{
    Drawable,
    geometry::Point,
    text::{Alignment, Text},
};

use crate::modules::{
    display::{DisplayRenderer, PageRender, style::STYLE_ACCENT},
    state::metronome::Metronome,
};

impl PageRender for Metronome {
    fn render(&self, renderer: &mut DisplayRenderer) {
        Text::with_alignment(
            "Metronome",
            Point::new(120, 90),
            STYLE_ACCENT.text_title(),
            Alignment::Center,
        )
        .draw(&mut renderer.display)
        .ok();

        renderer.draw_option(&self.bpm, 120, self.cursor.option_state(0));
        renderer.draw_option(&self.time_signature, 140, self.cursor.option_state(1));
    }
}
