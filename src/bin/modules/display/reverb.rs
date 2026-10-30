use embassy_time::Instant;
use embedded_graphics::{
    Drawable,
    geometry::{Point, Size},
    primitives::{Circle, Rectangle, RoundedRectangle, StyledDrawable},
    text::{Alignment, Text},
};

use crate::modules::{
    display::{
        DisplayRenderer, PageRender,
        style::{STYLE_ACCENT, STYLE_ACTIVE, Style},
    },
    state::reverb::Reverb,
    util::map_range,
};

impl PageRender for Reverb {
    fn render(&self, renderer: &mut DisplayRenderer) {
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
            STYLE_ACCENT.text_title(),
            Alignment::Center,
        )
        .draw(&mut renderer.display)
        .ok();

        // Options
        renderer.draw_option(&self.reverb_type().name, 150, self.cursor.option_state(0));
        renderer.draw_option(&self.reverb_depth, 170, self.cursor.option_state(1));
    }
}
