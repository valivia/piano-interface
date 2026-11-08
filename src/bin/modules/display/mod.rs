use crate::modules::display::style::{STYLE_ACCENT, STYLE_ACTIVE, STYLE_INACTIVE};
use crate::modules::util::get_point_edge;
use crate::modules::util::map_range_f;
use crate::modules::{
    display::{sprite::IndexedImage, style::Style},
    state::{Page, State, cursor::OptionState},
};
use crate::task::display::Display;
use core::fmt::Write;
use embassy_time::Instant;
use embedded_graphics::geometry::Angle;
use embedded_graphics::primitives::{Arc, PrimitiveStyle};
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

const BLINK_DURATION_MS: u64 = 500;

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

        // draw status icons
        match self.state.active_page {
            Page::Instrument => {
                self.draw_status_icons(&state);
            }
            _ => {}
        }

        // Menu indicator
        self.draw_current_menu_indicator();

        // Selector indicator
        let should_blink = match state.active_page {
            Page::Instrument => state.instrument.is_selecting(),
            Page::Metronome => state.metronome.cursor.editing,
            Page::Reverb => state.metronome.cursor.editing,
            Page::Demo => state.demo.is_selecting(),
        };

        self.draw_selector_indicator(should_blink);

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

    pub fn should_blink() -> bool {
        Instant::now().as_millis() % (BLINK_DURATION_MS * 2) < BLINK_DURATION_MS
    }

    pub fn draw_selector_indicator(&mut self, blinking: bool) {
        Circle::with_center(Point::new(240, 120), 16)
            .draw_styled(
                &Style::toggled(if blinking {
                    DisplayRenderer::should_blink()
                } else {
                    true
                })
                .fill,
                &mut self.display,
            )
            .ok();
    }

    pub fn draw_current_menu_indicator(&mut self) {
        let (location, active) = match self.state.active_page {
            Page::Metronome => (get_point_edge(210.0), self.state.metronome.enabled),
            Page::Reverb => (Point::new(0, 120), self.state.reverb.enabled),
            Page::Demo => (get_point_edge(150.0), self.state.demo.enabled),
            _ => return,
        };

        let style = if active {
            &STYLE_ACTIVE.fill
        } else {
            &STYLE_ACCENT.fill
        };

        Circle::with_center(location, 16)
            .draw_styled(style, &mut self.display)
            .ok();
    }

    pub fn draw_option(
        &mut self,
        label: &'static str,
        value: impl core::fmt::Display,
        y: i32,
        state: OptionState,
    ) {
        let mut buffer = String::<16>::new();
        write!(buffer, "{}", value).ok();

        // Styling
        let mut label_style = STYLE_ACCENT.text_large();
        let mut value_style = STYLE_ACCENT.text_large();
        match state {
            OptionState::Idle => (),
            OptionState::Selected => label_style.text_color = Some(STYLE_ACTIVE.color),
            OptionState::Editing => {
                value_style.text_color = None;
                value_style.background_color = Some(STYLE_ACTIVE.color);
            }
        }

        // Draw label
        Text::with_alignment(label, Point::new(120 - 6, y), label_style, Alignment::Right)
            .draw(&mut self.display)
            .ok();

        // Draw divider
        Text::with_alignment(
            ":",
            Point::new(120, y),
            STYLE_ACCENT.text_default(),
            Alignment::Center,
        )
        .draw(&mut self.display)
        .ok();

        // Draw value text
        let text = Text::with_alignment(
            &buffer,
            Point::new(120 + 8, y),
            value_style,
            Alignment::Left,
        );

        text.draw(&mut self.display).ok();

        // Draw line to selector
        //         const LINE_STROKE: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
        //             .stroke_color(STYLE_ACTIVE.color)
        //             .stroke_width(3)
        //             .build();
        //
        //         Circle::with_center(Point::new(240, 120), 16)
        //             .draw_styled(&STYLE_ACTIVE.fill, &mut self.display)
        //             .ok();
        //
        //         Line::new(Point::new(200, 120), Point::new(240, 120))
        //             .draw_styled(&LINE_STROKE, &mut self.display)
        //             .ok();
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
            .draw_styled(
                &Style::toggled(state.reverb.enabled).fill,
                &mut self.display,
            )
            .ok();
        RoundedRectangle::with_equal_corners(
            Rectangle::with_center(location, Size::new_equal(32)),
            Size::new_equal(6),
        )
        .draw_styled(
            &Style::toggled(state.reverb.enabled).stroke,
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
                &Style::toggled(state.metronome.enabled).palette,
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
