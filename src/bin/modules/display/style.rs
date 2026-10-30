use embedded_graphics::{
    mono_font::{
        MonoTextStyle,
        iso_8859_1::{FONT_7X13, FONT_10X20},
    },
    pixelcolor::{Rgb565, RgbColor},
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder},
};

use crate::modules::display::sprite::Palette;

// Color
pub const COLOR_INACTIVE: Rgb565 = Rgb565::new(9, 19, 9);
pub const COLOR_ACCENT: Rgb565 = Rgb565::new(26, 47, 18);
pub const COLOR_ACTIVE: Rgb565 = Rgb565::new(29, 42, 6);

// Outline
pub const STROKE_INACTIVE: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .stroke_color(COLOR_INACTIVE)
    .stroke_width(1)
    .build();
pub const STROKE_ACCENT: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .stroke_color(COLOR_ACCENT)
    .stroke_width(1)
    .build();
pub const STROKE_ACTIVE: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .stroke_color(COLOR_ACTIVE)
    .stroke_width(1)
    .build();

// Fill
pub const FILL_INACTIVE: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .fill_color(COLOR_INACTIVE)
    .build();
pub const FILL_ACCENT: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .fill_color(COLOR_ACCENT)
    .build();
pub const FILL_ACTIVE: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .fill_color(COLOR_ACTIVE)
    .build();

pub const PALETTE_INACTIVE: Palette =
    [Rgb565::BLACK, COLOR_INACTIVE, COLOR_INACTIVE, Rgb565::BLACK];
pub const PALETTE_ACCENT: Palette = [Rgb565::BLACK, COLOR_ACCENT, COLOR_ACCENT, Rgb565::BLACK];
pub const PALETTE_ACTIVE: Palette = [Rgb565::BLACK, COLOR_ACTIVE, COLOR_ACTIVE, Rgb565::BLACK];

pub struct Style {
    pub fill: &'static PrimitiveStyle<Rgb565>,
    pub stroke: &'static PrimitiveStyle<Rgb565>,
    pub color: Rgb565,
    pub palette: &'static Palette,
}

impl Style {
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
        MonoTextStyle::new(&FONT_10X20, self.color)
    }

    pub fn text_default(&self) -> MonoTextStyle<'_, Rgb565> {
        MonoTextStyle::new(&FONT_7X13, self.color)
    }

    pub fn text_large(&self) -> MonoTextStyle<'_, Rgb565> {
        MonoTextStyle::new(&FONT_10X20, self.color)
    }
}

pub const STYLE_INACTIVE: Style = Style {
    fill: &FILL_INACTIVE,
    stroke: &STROKE_INACTIVE,
    color: COLOR_INACTIVE,
    palette: &PALETTE_INACTIVE,
};

pub const STYLE_ACCENT: Style = Style {
    fill: &FILL_ACCENT,
    stroke: &STROKE_ACCENT,
    color: COLOR_ACCENT,
    palette: &PALETTE_ACCENT,
};

pub const STYLE_ACTIVE: Style = Style {
    fill: &FILL_ACTIVE,
    stroke: &STROKE_ACTIVE,
    color: COLOR_ACTIVE,
    palette: &PALETTE_ACTIVE,
};
