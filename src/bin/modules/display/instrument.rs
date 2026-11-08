use embedded_graphics::{
    Drawable,
    geometry::Point,
    text::{Alignment, Text},
};

use crate::modules::{
    display::{
        DisplayRenderer, PageRender,
        style::{STYLE_ACCENT, STYLE_ACTIVE, Style},
    },
    state::instrument::Instrument,
};

impl PageRender for Instrument {
    fn render(&self, renderer: &mut DisplayRenderer) {
        Text::with_alignment(
            self.active_instrument(),
            Point::new(120, 120),
            STYLE_ACCENT.text_large(),
            Alignment::Center,
        )
        .draw(&mut renderer.display)
        .ok();

        if self.is_selecting() {
            Text::with_alignment(
                self.selected_instrument(),
                Point::new(120, 140),
                Style::generic(DisplayRenderer::should_blink()).text_default(),
                Alignment::Center,
            )
            .draw(&mut renderer.display)
            .ok();
        }

        renderer.draw_selector_indicator(self.is_selecting());
    }
}
