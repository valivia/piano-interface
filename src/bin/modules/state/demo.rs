use crate::{modules::state::ControlEvents, task::button::ButtonPressType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Demo {
    pub enabled: bool,
    active_song_index: usize,
    selected_song_index: usize,
}

impl Default for Demo {
    fn default() -> Self {
        Demo {
            enabled: false,
            active_song_index: 0,
            selected_song_index: 0,
        }
    }
}

impl Demo {
    pub fn is_selecting(&self) -> bool {
        self.selected_song_index != self.active_song_index
    }
    pub fn active_song(&self) -> &'static str {
        DEMOS[self.active_song_index]
    }

    pub fn selected_song(&self) -> &'static str {
        DEMOS[self.selected_song_index]
    }
}

impl ControlEvents for Demo {
    fn encoder_event(&mut self, delta: i8) {
        let len = DEMOS.len() as isize;
        self.selected_song_index =
            (self.selected_song_index as isize + delta as isize).rem_euclid(len) as usize;
    }

    fn click_event(&mut self, _press_type: ButtonPressType) {
        if self.is_selecting() {
            self.active_song_index = self.selected_song_index;
            self.enabled = true;
        } else {
            self.enabled = !self.enabled;
        }
    }

    fn power_state_reset(&mut self) {
        self.active_song_index = self.selected_song_index
    }
}

pub const DEMOS: &[&'static str] = &[
    // Songs
    "Für elise",
    "Moonlight sonata",
    // Instrument demos
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
