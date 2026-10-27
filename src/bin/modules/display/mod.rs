use embedded_graphics::{
    Drawable,
    geometry::Point,
    mono_font::{
        MonoTextStyle,
        ascii::{FONT_7X13, FONT_10X20},
    },
    pixelcolor::Rgb565,
    primitives::{Circle, PrimitiveStyle, PrimitiveStyleBuilder, StyledDrawable},
    text::{Alignment, Text},
};

use crate::{
    modules::state::{Page, State},
    task::display::Display,
};

mod demo;
mod instrument;
mod metronome;
mod reverb;

// Color
const ACCENT_COLOR: Rgb565 = Rgb565::new(26, 47, 18);
const ACTIVE_COLOR: Rgb565 = Rgb565::new(31, 49, 0);

// Text
const CHAR_DEFAULT: MonoTextStyle<'_, Rgb565> = MonoTextStyle::new(&FONT_7X13, ACCENT_COLOR);
const CHAR_LARGE: MonoTextStyle<'_, Rgb565> = MonoTextStyle::new(&FONT_10X20, ACCENT_COLOR);

// Fill
const ACCENT_FILL: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .fill_color(ACCENT_COLOR)
    .build();
const ACTIVE_FILL: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .fill_color(ACTIVE_COLOR)
    .build();

pub struct DisplayRenderer {
    pub display: Display<'static>,
    pub previous_state: Option<State>,
}

impl DisplayRenderer {
    pub fn render(&mut self, new_state: &Option<State>) {
        let Some(state) = new_state else {
            return;
        };

        self.display.clear();

        match state.active_page {
            Page::Instrument => self.draw_instrument_page(state),
            Page::Metronome => self.draw_metronome_page(state),
            Page::Reverb => self.draw_reverb_page(state),
            Page::Demo => self.draw_demo_page(state),
        }

        self.previous_state = new_state.clone();

        self.display.flush().ok();
    }
}

impl DisplayRenderer {
    pub fn draw_status_icons(&mut self, state: &State) {
        let mut demo_icon = Text::with_alignment(
            "DEMO",
            Point::new(120, 220),
            CHAR_DEFAULT,
            Alignment::Center,
        );
        if state.demo.enabled {
            demo_icon.character_style.text_color = Some(ACTIVE_COLOR);
        }
        demo_icon.draw(&mut self.display).ok();

        Circle::with_center(Point::new(120 - 20, 190), 16)
            .draw_styled(
                if state.reverb.enabled {
                    &ACTIVE_FILL
                } else {
                    &ACCENT_FILL
                },
                &mut self.display,
            )
            .ok();

        Circle::with_center(Point::new(120 + 20, 190), 16)
            .draw_styled(
                if state.metronome.enabled {
                    &ACTIVE_FILL
                } else {
                    &ACCENT_FILL
                },
                &mut self.display,
            )
            .ok();
    }
}
