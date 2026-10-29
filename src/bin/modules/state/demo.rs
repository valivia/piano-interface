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
    pub fn active_song(&self) -> &'static str {
        DEMOS[self.active_song_index]
    }

    pub fn selected_song(&self) -> &'static str {
        DEMOS[self.selected_song_index]
    }
}

impl ControlEvents for Demo {
    fn encoder_event(&mut self, delta: i8) {}

    fn click_event(&mut self, press_type: ButtonPressType) {}
}

pub const DEMOS: &[&'static str] = &[
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
    // aaa
    "Für elise",
    "Moonlight sonata",
];
