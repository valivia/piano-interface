use embassy_time::Instant;
use embedded_graphics::{
    mono_font::{
        MonoTextStyle,
        iso_8859_1::{FONT_7X13, FONT_7X13_BOLD, FONT_10X20},
    },
    pixelcolor::{Rgb565, RgbColor},
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder},
    text::renderer::CharacterStyle,
};
use profont::{PROFONT_14_POINT, PROFONT_18_POINT, PROFONT_24_POINT};

use crate::modules::display::{BLINK_DURATION_MS, sprite::Palette};

// Color
pub const COLOR_INACTIVE: Rgb565 = Rgb565::new(9, 19, 9);
pub const COLOR_ACCENT: Rgb565 = Rgb565::new(26, 47, 18);
pub const COLOR_ACTIVE: Rgb565 = Rgb565::new(29, 42, 6);

pub struct Style {
    pub fill: PrimitiveStyle<Rgb565>,
    pub stroke: PrimitiveStyle<Rgb565>,
    pub color: Rgb565,
    pub palette: Palette,
}

impl Style {
    pub const fn new(color: Rgb565) -> Style {
        Style {
            color,
            fill: PrimitiveStyleBuilder::new().fill_color(color).build(),
            stroke: PrimitiveStyleBuilder::new()
                .stroke_color(color)
                .stroke_width(1)
                .build(),
            palette: [Rgb565::BLACK, color, color, Rgb565::BLACK],
        }
    }

    pub fn generic(enabled: bool) -> &'static Style {
        if enabled {
            &STYLE_ACTIVE
        } else {
            &STYLE_ACCENT
        }
    }

    pub fn toggled(enabled: bool) -> &'static Style {
        if enabled {
            &STYLE_ACTIVE
        } else {
            &STYLE_INACTIVE
        }
    }

    pub fn text_title(&self) -> MonoTextStyle<'_, Rgb565> {
        MonoTextStyle::new(&PROFONT_18_POINT, self.color)
    }

    pub fn text_large(&self) -> MonoTextStyle<'_, Rgb565> {
        MonoTextStyle::new(&FONT_10X20, self.color)
    }

    pub fn text_default(&self) -> MonoTextStyle<'_, Rgb565> {
        MonoTextStyle::new(&FONT_7X13_BOLD, self.color)
    }
}

pub const STYLE_INACTIVE: Style = Style::new(COLOR_INACTIVE);
pub const STYLE_ACCENT: Style = Style::new(COLOR_ACCENT);
pub const STYLE_ACTIVE: Style = Style::new(COLOR_ACTIVE);
