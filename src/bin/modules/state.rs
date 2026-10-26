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
    pub active_instrument: &'static str,
    pub selected_instrument: &'static str,
}

impl Default for State {
    fn default() -> Self {
        Self {
            active_page: Page::Instrument,
            active_instrument: "",
            selected_instrument: "",
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
