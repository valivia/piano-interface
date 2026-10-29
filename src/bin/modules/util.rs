pub fn map_range(old: (u32, u32), new: (u32, u32), x: u8) -> u32 {
    (new.0 + (x as u32 * (new.1 - new.0) / (old.1 - old.0))) as u32
}
