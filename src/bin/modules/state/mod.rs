use defmt::info;

use crate::{
    modules::{
        display::PageRender,
        state::{demo::Demo, instrument::Instrument, metronome::Metronome, reverb::Reverb},
    },
    task::button::{ButtonEvent, ButtonId, ButtonPressType},
};

pub mod cursor;
pub mod demo;
pub mod instrument;
pub mod metronome;
pub mod reverb;

const TAG: &'static str = "[state]";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Instrument,
    Metronome,
    Reverb,
    Demo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    pub is_on: bool,
    pub active_page: Page,

    pub instrument: Instrument,
    pub metronome: Metronome,
    pub reverb: Reverb,
    pub demo: Demo,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_on: false,
            active_page: Page::Instrument,
            instrument: Instrument::default(),
            metronome: Metronome::default(),
            demo: Demo::default(),
            reverb: Reverb::default(),
        }
    }
}

trait ControlEvents {
    fn encoder_event(&mut self, delta: i8);
    fn click_event(&mut self, press_type: ButtonPressType);
    fn power_state_reset(&mut self);
}

impl State {
    pub fn encoder_event(&mut self, delta: i8) {
        if !self.is_on {
            return;
        }
        self.get_control_page().encoder_event(delta);
    }

    pub fn button_event(&mut self, event: ButtonEvent) {
        if !self.is_on {
            return;
        }

        let is_instrument_page = self.active_page != Page::Instrument;

        match event.id {
            ButtonId::OnOff => (),
            ButtonId::Demo => {
                self.page_event(Page::Demo);
            }
            ButtonId::Metronome => {
                if event.press_type == ButtonPressType::Short {
                    self.metronome.enabled = !self.metronome.enabled;
                } else {
                    self.page_event(Page::Metronome);
                }
            }
            ButtonId::Reverb => {
                if event.press_type == ButtonPressType::Short {
                    self.reverb.enabled = !self.reverb.enabled;
                } else {
                    self.page_event(Page::Reverb);
                }
            }
            ButtonId::Click => {
                self.get_control_page().click_event(event.press_type);
            }
        }
    }

    pub fn power_state_event(&mut self, state: bool) {
        self.is_on = state;
        if !state {
            info!("{} power state changed, resetting state", TAG);
            self.instrument.power_state_reset();
            self.metronome.power_state_reset();
            self.reverb.power_state_reset();
            self.demo.power_state_reset();
            self.active_page = Page::Instrument;
        };
    }

    fn page_event(&mut self, page: Page) {
        if self.active_page == page {
            self.active_page = Page::Instrument;
        } else {
            self.active_page = page;
        }
    }

    fn get_control_page(&mut self) -> &mut dyn ControlEvents {
        match self.active_page {
            Page::Instrument => &mut self.instrument,
            Page::Demo => &mut self.demo,
            Page::Metronome => &mut self.metronome,
            Page::Reverb => &mut self.reverb,
        }
    }

    pub fn get_render_page(&self) -> &dyn PageRender {
        match self.active_page {
            Page::Instrument => &self.instrument,
            Page::Demo => &self.demo,
            Page::Metronome => &self.metronome,
            Page::Reverb => &self.reverb,
        }
    }
}
