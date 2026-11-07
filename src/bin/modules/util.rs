pub fn map_range(old: (u32, u32), new: (u32, u32), x: u32) -> u32 {
    new.0 + (x * (new.1 - new.0) / (old.1 - old.0))
}

pub fn map_range_f(old: (f32, f32), new: (f32, f32), x: f32) -> f32 {
    new.0 + (x * (new.1 - new.0) / (old.1 - old.0))
}
