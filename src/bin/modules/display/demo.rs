use embedded_graphics::{
    Drawable,
    geometry::Point,
    text::{Alignment, Text},
};

use crate::modules::{
    display::{DisplayRenderer, PageRender, TEXT_LARGE},
    state::demo::Demo,
};

impl PageRender for Demo {
    fn render(&self, renderer: &mut DisplayRenderer) {
        Text::with_alignment("Demo", Point::new(120, 90), TEXT_LARGE, Alignment::Center)
            .draw(&mut renderer.display)
            .ok();

        Text::with_alignment(
            self.active_song(),
            Point::new(120, 120),
            TEXT_LARGE,
            Alignment::Center,
        )
        .draw(&mut renderer.display)
        .ok();
    }
}
