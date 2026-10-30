use crate::task::button::ButtonPressType;

pub enum OptionState {
    Idle,
    Selected,
    Editing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Cursor {
    pub selected: usize,
    pub editing: bool,
}

impl Cursor {
    pub fn encoder(&mut self, delta: i8, len: usize) -> Option<usize> {
        if self.editing {
            Some(self.selected)
        } else {
            self.selected =
                (self.selected as isize + delta as isize).clamp(0, len as isize - 1) as usize;
            None
        }
    }

    pub fn click(&mut self, press_type: ButtonPressType) {
        if press_type == ButtonPressType::Short {
            self.editing = !self.editing;
        }
    }

    pub fn option_state(&self, index: usize) -> OptionState {
        match (self.selected == index, self.editing) {
            (false, _) => OptionState::Idle,
            (true, false) => OptionState::Selected,
            (true, true) => OptionState::Editing,
        }
    }
}
