use core::fmt::Write;
use embedded_graphics::{
    Drawable,
    geometry::Point,
    text::{Alignment, Text},
};
use heapless::String;

use crate::modules::{
    display::{DisplayRenderer, PageRender, TEXT_LARGE},
    state::metronome::{Metronome, MetronomeSetting},
};

impl PageRender for Metronome {
    fn render(&self, renderer: &mut DisplayRenderer) {
        let mut text = String::<8>::new();

        // let image = Image::new(&tga, Point::zero());
        Text::with_alignment(
            "Metronome",
            Point::new(120, 90),
            TEXT_LARGE,
            Alignment::Center,
        )
        .draw(&mut renderer.display)
        .ok();

        write!(text, "{}", self.bpm).ok();
        let mut bpm =
            Text::with_alignment(&text, Point::new(120, 120), TEXT_LARGE, Alignment::Center);

        if self.selected_setting() == MetronomeSetting::Bpm {
            bpm.character_style.underline_color =
                embedded_graphics::text::DecorationColor::TextColor;
        }

        bpm.draw(&mut renderer.display).ok();
        text.clear();

        write!(text, "{}", self.time_signature).ok();
        let mut time_signature =
            Text::with_alignment(&text, Point::new(120, 140), TEXT_LARGE, Alignment::Center);

        if self.selected_setting() == MetronomeSetting::TimeSignature {
            time_signature.character_style.underline_color =
                embedded_graphics::text::DecorationColor::TextColor;
        }

        time_signature.draw(&mut renderer.display).ok();
    }
}
