use core::f32::consts::PI;
use embedded_graphics::geometry::Point;
use micromath::F32Ext;

pub fn map_range(old: (u32, u32), new: (u32, u32), x: u32) -> u32 {
    new.0 + (x * (new.1 - new.0) / (old.1 - old.0))
}

pub fn map_range_f(old: (f32, f32), new: (f32, f32), x: f32) -> f32 {
    new.0 + (x * (new.1 - new.0) / (old.1 - old.0))
}

pub fn get_point_edge(angle: f32) -> Point {
    let x = 120.0 + 120.0 * f32::cos(angle * PI / 180.0);
    let y = 120.0 + 120.0 * f32::sin(angle * PI / 180.0);
    Point::new(x as i32, y as i32)
}
