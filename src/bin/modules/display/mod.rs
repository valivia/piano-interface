use core::fmt::Write;

use crate::modules::display::style::STYLE_ACCENT;
use crate::modules::display::style::STYLE_ACTIVE;
use crate::modules::display::style::STYLE_INACTIVE;
use crate::modules::util::map_range_f;
use crate::modules::{
    display::{sprite::IndexedImage, style::Style},
    state::{Page, State, cursor::OptionState},
};
use crate::task::display::Display;
use embassy_time::Instant;
use embedded_graphics::geometry::Angle;
use embedded_graphics::geometry::Dimensions;
use embedded_graphics::primitives::Arc;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::text::DecorationColor;
use embedded_graphics::{
    Drawable,
    geometry::{Point, Size},
    mono_font::{MonoTextStyle, ascii::FONT_7X14_BOLD},
    primitives::{Circle, Rectangle, RoundedRectangle, StyledDrawable},
    text::{Alignment, Text},
};
use heapless::String;

mod demo;
mod instrument;
mod metronome;
mod reverb;
mod sprite;
mod style;

const FLASH_DURATION_MS: u64 = 1000;

pub struct AnimationState {
    pub timings: [Instant; 4],
    // Specifics
    pub delay_circle_size: u32,
}

pub struct VolumeState {
    pub last_change: Instant,
    pub value: u8,
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
    pub volume_state: Option<VolumeState>,
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

        // Draw volume UI
        if let Some(volume_state) = &self.volume_state {
            if Instant::now()
                .checked_duration_since(volume_state.last_change)
                .is_some_and(|duration| duration.as_millis() < 1000)
            {
                self.draw_volume(volume_state.value);
            }
        }

        // Reset anim state
        if let Some(previous_state) = self.previous_state {
            if previous_state.active_page != self.state.active_page {
                self.animation_state = AnimationState::default();
            }
        }

        self.display.flush().ok();
    }

    pub fn store_volume(&mut self, volume: u8) {
        self.volume_state = Some(VolumeState {
            last_change: Instant::now(),
            value: volume,
        });
    }

    pub fn draw_option(&mut self, value: impl core::fmt::Display, y: i32, state: OptionState) {
        let mut buffer = String::<16>::new();
        write!(buffer, "{}", value).ok();

        let mut style = STYLE_ACCENT.text_large();
        match state {
            OptionState::Idle => (),
            OptionState::Selected => style.text_color = Some(STYLE_ACTIVE.color),
            OptionState::Editing => {
                style.text_color = Some(STYLE_ACTIVE.color);
                style.underline_color = DecorationColor::TextColor
            }
        }

        let text = Text::with_alignment(&buffer, Point::new(120, y), style, Alignment::Center);

        text.bounding_box();

        text.draw(&mut self.display).ok();
    }

    pub fn draw_status_icons(&mut self, state: &State) {
        // Demo
        let text_style =
            MonoTextStyle::new(&FONT_7X14_BOLD, Style::toggled(state.demo.enabled).color);

        let demo_icon =
            Text::with_alignment("DEMO", Point::new(120, 220), text_style, Alignment::Center);

        demo_icon.draw(&mut self.display).ok();

        static ICON_OFFSET: i32 = 26;
        static ICON_HEIGHT: i32 = 175;

        // Reverb
        let location = Point::new(120 - ICON_OFFSET, ICON_HEIGHT);
        Circle::with_center(location, 4)
            .draw_styled(Style::toggled(state.reverb.enabled).fill, &mut self.display)
            .ok();
        RoundedRectangle::with_equal_corners(
            Rectangle::with_center(location, Size::new_equal(32)),
            Size::new_equal(6),
        )
        .draw_styled(
            Style::toggled(state.reverb.enabled).stroke,
            &mut self.display,
        )
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
                Style::toggled(state.metronome.enabled).palette,
            )
            .unwrap();
    }

    pub fn draw_volume(&mut self, volume: u8) {
        let circle = Circle::new(Point::new(0, 0), 240);
        let start_angle = Angle::from_degrees(150.0);

        Arc::from_circle(circle, start_angle, Angle::from_degrees(60.0))
            .draw_styled(
                &PrimitiveStyle::with_stroke(STYLE_INACTIVE.color, 5),
                &mut self.display,
            )
            .ok();

        Arc::from_circle(
            circle,
            start_angle,
            Angle::from_degrees(map_range_f((0.0, 100.0), (0.0, 60.0), volume as f32)),
        )
        .draw_styled(
            &PrimitiveStyle::with_stroke(STYLE_ACTIVE.color, 5),
            &mut self.display,
        )
        .ok();
    }
}

pub trait PageRender {
    fn render(&self, renderer: &mut DisplayRenderer);
}
