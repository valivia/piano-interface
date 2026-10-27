#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Instrument,
    Metronome,
    Reverb,
    Demo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Metronome {
    pub enabled: bool,
    pub bpm: u8,
    pub time_signature: u8,
}

impl Default for Metronome {
    fn default() -> Self {
        Self {
            enabled: false,
            bpm: 120,
            time_signature: 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReverbType {
    Room,
    Hall1,
    Hall2,
    Stage,
}

impl ReverbType {
    pub fn name(&self) -> &'static str {
        match self {
            ReverbType::Room => "Room",
            ReverbType::Hall1 => "Hall 1",
            ReverbType::Hall2 => "Hall 2",
            ReverbType::Stage => "Stage",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reverb {
    pub enabled: bool,
    pub reverb_type: ReverbType,
    pub reverb_depth: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Demo {
    pub enabled: bool,
    pub active_song: &'static str,
    pub selected_song: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    pub active_page: Page,
    pub metronome: Metronome,
    pub reverb: Reverb,
    pub demo: Demo,
    active_instrument_index: usize,
    selected_instrument_index: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            active_page: Page::Instrument,
            active_instrument_index: 0,
            selected_instrument_index: 0,
            metronome: Metronome::default(),
            demo: Demo {
                enabled: false,
                active_song: "",
                selected_song: "",
            },
            reverb: Reverb {
                enabled: true,
                reverb_type: ReverbType::Hall1,
                reverb_depth: 50,
            },
        }
    }
}

impl State {
    pub fn encoder_event(&mut self, delta: i16) {
        match self.active_page {
            Page::Instrument => {
                self.active_instrument_index = (self.active_instrument_index as i16
                    + delta.clamp(-1, 1)) as usize
                    % (INSTRUMENTS.len() - 1);
            }
            Page::Demo => {}
            Page::Metronome => {}
            Page::Reverb => {}
        }
    }

    pub fn page_event(&mut self, page: Page) {
        if self.active_page == page {
            self.active_page = Page::Instrument;
        } else {
            self.active_page = page;
        }
    }
}

impl State {
    pub fn active_instrument(&self) -> &'static str {
        return INSTRUMENTS[self.active_instrument_index];
    }

    pub fn selected_instrument(&self) -> &'static str {
        return INSTRUMENTS[self.selected_instrument_index];
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
