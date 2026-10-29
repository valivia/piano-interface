use crate::{
    modules::state::{ControlEvents, cursor::Cursor},
    task::button::ButtonPressType,
};

// Cursor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReverbSetting {
    ReverbType,
    ReverbDepth,
}

pub const REVERB_SETTINGS: [ReverbSetting; 2] =
    [ReverbSetting::ReverbType, ReverbSetting::ReverbDepth];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reverb {
    pub enabled: bool,
    reverb_type_index: usize,
    pub reverb_depth: u8,
    pub cursor: Cursor,
}

impl Default for Reverb {
    fn default() -> Self {
        Self {
            enabled: true,
            reverb_type_index: 0,
            reverb_depth: 50,
            cursor: Cursor::default(),
        }
    }
}

impl Reverb {
    pub fn reverb_type(&self) -> &'static ReverbType {
        &REVERB_TYPES[self.reverb_type_index]
    }
}

impl ControlEvents for Reverb {
    fn encoder_event(&mut self, delta: i8) {
        let Some(i) = self.cursor.encoder(delta, REVERB_SETTINGS.len()) else {
            return;
        };

        match REVERB_SETTINGS[i] {
            ReverbSetting::ReverbType => {
                self.reverb_type_index = (self.reverb_type_index as isize + delta as isize)
                    .clamp(0, REVERB_TYPES.len() as isize - 1)
                    as usize;
            }
            ReverbSetting::ReverbDepth => {
                self.reverb_depth =
                    (self.reverb_depth as i16 + delta as i16 * 5).clamp(10, 100) as u8;
            }
        }
    }

    fn click_event(&mut self, press_type: ButtonPressType) {
        self.cursor.click(press_type);
    }
}

pub struct ReverbType {
    pub name: &'static str,
}

static REVERB_TYPES: &[ReverbType] = &[
    ReverbType { name: "Room" },
    ReverbType { name: "Hall 1" },
    ReverbType { name: "Hall 2" },
    ReverbType { name: "Stage" },
];
