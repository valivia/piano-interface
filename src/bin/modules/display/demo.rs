use embassy_time::Instant;
use embedded_graphics::{
    Drawable,
    geometry::{Point, Size},
    text::{Alignment, Text},
};
use micromath::F32Ext;

use crate::modules::{
    display::{
        DisplayRenderer, PageRender,
        sprite::IndexedImage,
        style::{STYLE_ACCENT, Style},
    },
    state::demo::Demo,
};

impl PageRender for Demo {
    fn render(&self, renderer: &mut DisplayRenderer) {
        const WIDTH: u32 = 32;
        const HEIGHT: u32 = 38;

        static METRONOME: IndexedImage = IndexedImage {
            size: Size::new(WIDTH, HEIGHT),
            data: include_bytes!("./assets/note.4bpp"),
        };

        let height_offset = if self.enabled {
            (f32::sin(Instant::now().as_millis() as f32 / 1000.0) * 12.0) as i32
        } else {
            8
        };

        const BASE_HEIGHT: i32 = 60 - (HEIGHT as i32 / 2);
        const X_OFFSET: i32 = 20;

        METRONOME
            .draw(
                &mut renderer.display,
                Point::new(
                    120 - X_OFFSET - (WIDTH as i32 / 2),
                    BASE_HEIGHT - height_offset,
                ),
                Style::toggled(self.enabled).palette,
            )
            .unwrap();

        METRONOME
            .draw(
                &mut renderer.display,
                Point::new(
                    120 + X_OFFSET - (WIDTH as i32 / 2),
                    BASE_HEIGHT + height_offset,
                ),
                Style::toggled(self.enabled).palette,
            )
            .unwrap();

        // Title
        Text::with_alignment(
            "Demo",
            Point::new(120, 120),
            STYLE_ACCENT.text_title(),
            Alignment::Center,
        )
        .draw(&mut renderer.display)
        .ok();

        // Active
        Text::with_alignment(
            self.active_song(),
            Point::new(120, 140),
            Style::generic(self.enabled).text_large(),
            Alignment::Center,
        )
        .draw(&mut renderer.display)
        .ok();

        // Selecting
        if self.is_selecting() {
            Text::with_alignment(
                self.selected_song(),
                Point::new(120, 160),
                STYLE_ACCENT.text_default(),
                Alignment::Center,
            )
            .draw(&mut renderer.display)
            .ok();
        }
    }
}
