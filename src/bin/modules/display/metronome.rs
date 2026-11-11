use core::fmt::Write;
use embassy_time::Instant;
use embedded_graphics::{
    Drawable,
    geometry::Point,
    primitives::{Circle, StrokeAlignment, StyledDrawable, Triangle},
    text::{Alignment, Text},
};
use heapless::String;

use crate::modules::{
    display::{
        DisplayRenderer, PageRender,
        style::{STYLE_ACCENT, Style},
    },
    state::metronome::Metronome,
};

impl PageRender for Metronome {
    fn render(&self, renderer: &mut DisplayRenderer) {
        // Graphic

        // Time signature dots
        if self.time_signature >= 2 {
            const SIZE: u32 = 10;
            const SPACING: i32 = (SIZE + SIZE / 2) as i32;
            const MAX_PER_ROW: usize = 8;
            const START_Y: i32 = 80;

            let total = self.time_signature as usize;
            let y_offset = if total <= MAX_PER_ROW { SPACING / 2 } else { 0 };

            for index in 0..total {
                let row = index / MAX_PER_ROW;
                let col = index % MAX_PER_ROW;

                let dots_in_row = (total - row * MAX_PER_ROW).min(MAX_PER_ROW);

                let row_width = (dots_in_row as i32 - 1) * SPACING;
                let x = 120 - row_width / 2 + col as i32 * SPACING;
                let y = START_Y + y_offset + row as i32 * SPACING;

                let color = Style::toggled(renderer.metronome_state.is_some_and(|state| {
                    state.current_beat == index as u8
                        && Instant::now()
                            .duration_since(state.beat_instant)
                            .as_millis()
                            < 200
                }));
                let style = if index == 0 {
                    color.fill
                } else {
                    let mut stroke = color.stroke;
                    stroke.stroke_width = 3;
                    stroke.stroke_alignment = StrokeAlignment::Inside;
                    stroke
                };

                Circle::with_center(Point::new(x, y), SIZE)
                    .draw_styled(&style, &mut renderer.display)
                    .ok();
            }
        }

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
