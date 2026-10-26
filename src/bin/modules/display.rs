use defmt::{info, println};
use embedded_graphics::{
    Drawable,
    geometry::Point,
    mono_font::{MonoTextStyle, ascii::FONT_7X13_BOLD},
    pixelcolor::Rgb565,
    primitives::{Circle, PrimitiveStyle, PrimitiveStyleBuilder, StyledDrawable},
    text::{Alignment, Text},
};

use crate::{modules::state::State, task::display::Display};

const ACCENT_COLOR: Rgb565 = Rgb565::new(26, 47, 18);
const ACTIVE_COLOR: Rgb565 = Rgb565::new(31, 49, 0);

const CHAR_STYLE: MonoTextStyle<'_, Rgb565> = MonoTextStyle::new(&FONT_7X13_BOLD, ACCENT_COLOR);
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

        self.draw_instrument_page(state);

        self.previous_state = new_state.clone();

        self.display.flush().ok();
    }
}

impl DisplayRenderer {
    pub fn draw_instrument_page(&mut self, state: &State) {
        self.draw_status_icons(state);
    }

    pub fn draw_status_icons(&mut self, state: &State) {
        let mut demo_icon =
            Text::with_alignment("DEMO", Point::new(120, 220), CHAR_STYLE, Alignment::Center);
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
