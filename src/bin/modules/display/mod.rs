use embassy_time::Instant;
use embedded_graphics::{
    Drawable,
    geometry::{Point, Size},
    mono_font::{
        MonoTextStyle,
        ascii::{FONT_7X13, FONT_7X14_BOLD, FONT_10X20},
    },
    pixelcolor::{Rgb565, RgbColor},
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, RoundedRectangle, StyledDrawable,
    },
    text::{Alignment, Text},
};

use crate::{
    modules::{
        display::sprite::{IndexedImage, Palette},
        state::{Page, State},
    },
    task::display::Display,
};

mod demo;
mod instrument;
mod metronome;
mod reverb;
mod sprite;

// Color
const COLOR_INACTIVE: Rgb565 = Rgb565::new(9, 19, 9);
const COLOR_ACCENT: Rgb565 = Rgb565::new(26, 47, 18);
const COLOR_ACTIVE: Rgb565 = Rgb565::new(29, 42, 6);

// Text
const TEXT_DEFAULT: MonoTextStyle<'_, Rgb565> = MonoTextStyle::new(&FONT_7X13, COLOR_ACCENT);
const TEXT_LARGE: MonoTextStyle<'_, Rgb565> = MonoTextStyle::new(&FONT_10X20, COLOR_ACCENT);

// Outline
const STROKE_INACTIVE: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .stroke_color(COLOR_INACTIVE)
    .stroke_width(1)
    .build();
const STROKE_ACCENT: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .stroke_color(COLOR_ACCENT)
    .stroke_width(1)
    .build();
const STROKE_ACTIVE: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .stroke_color(COLOR_ACTIVE)
    .stroke_width(1)
    .build();

// Fill
const FILL_INACTIVE: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .fill_color(COLOR_INACTIVE)
    .build();
const FILL_ACCENT: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .fill_color(COLOR_ACCENT)
    .build();
const FILL_ACTIVE: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .fill_color(COLOR_ACTIVE)
    .build();

const PALETTE_INACTIVE: Palette = [Rgb565::BLACK, COLOR_INACTIVE, COLOR_INACTIVE, Rgb565::BLACK];
const PALETTE_ACCENT: Palette = [Rgb565::BLACK, COLOR_ACCENT, COLOR_ACCENT, Rgb565::BLACK];
const PALETTE_ACTIVE: Palette = [Rgb565::BLACK, COLOR_ACTIVE, COLOR_ACTIVE, Rgb565::BLACK];

struct Style {
    fill: &'static PrimitiveStyle<Rgb565>,
    stroke: &'static PrimitiveStyle<Rgb565>,
    color: Rgb565,
    palette: &'static Palette,
}

impl Style {
    pub fn generic(enabled: bool) -> &'static Style {
        if enabled {
            &STYLE_ACTIVE
        } else {
            &STYLE_ACCENT
        }
    }
}

const STYLE_INACTIVE: Style = Style {
    fill: &FILL_INACTIVE,
    stroke: &STROKE_INACTIVE,
    color: COLOR_INACTIVE,
    palette: &PALETTE_INACTIVE,
};

const STYLE_ACCENT: Style = Style {
    fill: &FILL_ACCENT,
    stroke: &STROKE_ACCENT,
    color: COLOR_ACCENT,
    palette: &PALETTE_ACCENT,
};

const STYLE_ACTIVE: Style = Style {
    fill: &FILL_ACTIVE,
    stroke: &STROKE_ACTIVE,
    color: COLOR_ACTIVE,
    palette: &PALETTE_ACTIVE,
};

pub struct AnimationState {
    pub timings: [Instant; 4],
    // Specifics
    pub delay_circle_size: u32,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            timings: [Instant::now(); 4],
            delay_circle_size: 10,
        }
    }
}

pub struct DisplayRenderer<'a> {
    pub display: Display<'a>,
    pub state: State,
    pub previous_state: Option<State>,
    pub animation_state: AnimationState,
}

impl<'a> DisplayRenderer<'a> {
    pub fn render(&mut self, new_state: &Option<State>) {
        if self.previous_state.is_some() {
            self.previous_state = None;
        }

        if let Some(state) = new_state {
            self.previous_state = Some(self.state);
            self.state = *state;
        }

        self.display.clear();

        let state = self.state.clone();

        state.get_render_page().render(self);

        match self.state.active_page {
            Page::Instrument => {
                self.draw_status_icons(&state);
            }
            _ => {}
        }

        // Reset anim state
        if let Some(previous_state) = self.previous_state {
            if previous_state.active_page != self.state.active_page {
                self.animation_state = AnimationState::default();
            }
        }

        self.display.flush().ok();
    }
}

fn icon_style(enabled: bool) -> &'static Style {
    if enabled {
        &STYLE_ACTIVE
    } else {
        &STYLE_INACTIVE
    }
}

impl<'a> DisplayRenderer<'a> {
    pub fn draw_status_icons(&mut self, state: &State) {
        // Demo
        let text_style = MonoTextStyle::new(&FONT_7X14_BOLD, icon_style(state.demo.enabled).color);

        let demo_icon =
            Text::with_alignment("DEMO", Point::new(120, 220), text_style, Alignment::Center);

        demo_icon.draw(&mut self.display).ok();

        static ICON_OFFSET: i32 = 26;
        static ICON_HEIGHT: i32 = 175;

        // Reverb
        let location = Point::new(120 - ICON_OFFSET, ICON_HEIGHT);
        Circle::with_center(location, 4)
            .draw_styled(icon_style(state.reverb.enabled).fill, &mut self.display)
            .ok();
        RoundedRectangle::with_equal_corners(
            Rectangle::with_center(location, Size::new_equal(32)),
            Size::new_equal(6),
        )
        .draw_styled(icon_style(state.reverb.enabled).stroke, &mut self.display)
        .ok();

        // Metronome
        static METRONOME: IndexedImage = IndexedImage {
            size: Size::new(32, 32),
            data: include_bytes!("./assets/metronome.4bpp"),
        };

        METRONOME
            .draw(
                &mut self.display,
                Point::new(120 + ICON_OFFSET - 16, ICON_HEIGHT - 16),
                icon_style(state.metronome.enabled).palette,
            )
            .unwrap();
    }
}

pub trait PageRender {
    fn render(&self, renderer: &mut DisplayRenderer);
}
