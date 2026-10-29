use core::fmt::Write;
use embassy_time::Instant;
use embedded_graphics::{
    Drawable,
    geometry::{Point, Size},
    primitives::{Circle, Rectangle, RoundedRectangle, StyledDrawable},
    text::{Alignment, Text},
};
use heapless::String;

use crate::modules::{
    display::{DisplayRenderer, PageRender, STYLE_ACCENT, STYLE_ACTIVE, Style, TEXT_LARGE},
    state::reverb::Reverb,
    util::map_range,
};

impl PageRender for Reverb {
    fn render(&self, renderer: &mut DisplayRenderer) {
        let mut text = String::<8>::new();

        // Logo
        const MIN_SIZE: u32 = 20;
        const MAX_SIZE: u32 = 64;

        let has_reverb = self.enabled && self.reverb_depth > 0;

        let size = map_range((0, 100), (MIN_SIZE, MAX_SIZE), self.reverb_depth);
        let location = Point::new(120, 60);
        RoundedRectangle::with_equal_corners(
            Rectangle::with_center(location, Size::new_equal(size)),
            Size::new_equal(16),
        )
        .draw_styled(STYLE_ACCENT.stroke, &mut renderer.display)
        .ok();

        Circle::with_center(location, 4)
            .draw_styled(STYLE_ACCENT.fill, &mut renderer.display)
            .ok();

        let anim_state = &mut renderer.animation_state;

        if has_reverb {
            if (Instant::now() - anim_state.timings[0]).as_millis() > 100 {
                anim_state.timings[0] = Instant::now();
                anim_state.delay_circle_size += 2;
                if anim_state.delay_circle_size > size - 1 {
                    anim_state.delay_circle_size = 10;
                }
            }

            RoundedRectangle::with_equal_corners(
                Rectangle::with_center(location, Size::new_equal(anim_state.delay_circle_size)),
                Size::new_equal(16),
            )
            .draw_styled(STYLE_ACTIVE.stroke, &mut renderer.display)
            .unwrap();
        }

        Circle::with_center(location, 4)
            .draw_styled(Style::generic(has_reverb).stroke, &mut renderer.display)
            .unwrap();

        // Title
        Text::with_alignment(
            "Reverb",
            Point::new(120, 120),
            TEXT_LARGE,
            Alignment::Center,
        )
        .draw(&mut renderer.display)
        .ok();

        // Options
        Text::with_alignment(
            self.reverb_type().name,
            Point::new(120, 150),
            TEXT_LARGE,
            Alignment::Center,
        )
        .draw(&mut renderer.display)
        .ok();

        write!(text, "{}", self.reverb_depth).ok();
        Text::with_alignment(&text, Point::new(120, 170), TEXT_LARGE, Alignment::Center)
            .draw(&mut renderer.display)
            .ok();
    }
}
