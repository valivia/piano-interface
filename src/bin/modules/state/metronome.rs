use crate::{
    modules::state::{ControlEvents, cursor::Cursor},
    task::button::ButtonPressType,
};

// Cursor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetronomeSetting {
    Bpm,
    TimeSignature,
}

pub const METRONOME_SETTINGS: [MetronomeSetting; 2] =
    [MetronomeSetting::Bpm, MetronomeSetting::TimeSignature];

// Struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Metronome {
    pub enabled: bool,
    pub bpm: u8,
    pub time_signature: u8,
    pub cursor: Cursor,
}

// Impl
impl Default for Metronome {
    fn default() -> Self {
        Self {
            enabled: false,
            bpm: 120,
            time_signature: 4,
            cursor: Cursor::default(),
        }
    }
}

impl ControlEvents for Metronome {
    fn encoder_event(&mut self, delta: i8) {
        let Some(i) = self.cursor.encoder(delta, METRONOME_SETTINGS.len()) else {
            return;
        };

        match METRONOME_SETTINGS[i] {
            MetronomeSetting::Bpm => {
                self.bpm = (self.bpm as i16 + delta as i16).clamp(30, 255) as u8;
            }
            MetronomeSetting::TimeSignature => {
                self.time_signature =
                    (self.time_signature as i16 + delta as i16).clamp(1, 16) as u8;
            }
        }
    }

    fn click_event(&mut self, press_type: ButtonPressType) {
        self.cursor.click(press_type);
    }

    fn power_state_reset(&mut self) {
        self.enabled = false;
        self.cursor = Cursor::default();
    }
}
