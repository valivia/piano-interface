use crate::{modules::state::ControlEvents, task::button::ButtonPressType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instrument {
    active_instrument_index: usize,
    selected_instrument_index: usize,
}

impl Default for Instrument {
    fn default() -> Self {
        Self {
            active_instrument_index: 0,
            selected_instrument_index: 0,
        }
    }
}

impl Instrument {
    pub fn active_instrument(&self) -> &'static str {
        return INSTRUMENTS[self.active_instrument_index];
    }

    pub fn selected_instrument(&self) -> &'static str {
        return INSTRUMENTS[self.selected_instrument_index];
    }

    pub fn is_selecting(&self) -> bool {
        self.active_instrument_index != self.selected_instrument_index
    }
}

impl ControlEvents for Instrument {
    fn encoder_event(&mut self, delta: i8) {
        // Wrapping version
        // let len = INSTRUMENTS.len() as isize;
        // self.selected_instrument_index =
        //     (self.selected_instrument_index as isize + delta as isize).rem_euclid(len) as usize;

        // Clamped version
        self.selected_instrument_index = (self.selected_instrument_index as isize + delta as isize)
            .clamp(0, INSTRUMENTS.len() as isize - 1)
            as usize;
    }

    fn click_event(&mut self, _press_type: ButtonPressType) {
        self.active_instrument_index = self.selected_instrument_index;
    }
}

pub const INSTRUMENTS: &[&'static str] = &[
    "Grand piano 1",
    "Grand piano 2",
    "Electric piano 1",
    "Electric piano 2",
    "Harpsicord 1",
    "Harpsicord 2",
    "Vibraphone",
    "Church organ 1",
    "Church organ 2",
    "Strings",
];
