use embedded_graphics::{pixelcolor::Rgb565, prelude::*, primitives::Rectangle};

pub type Palette = [Rgb565; 4];

pub struct IndexedImage<'a> {
    pub size: Size,
    /// 4bpp, two pixels per byte, high nibble first, row-major, no row padding
    pub data: &'a [u8],
}

impl<'a> IndexedImage<'a> {
    #[inline]
    fn index_at(&self, i: usize) -> u8 {
        let b = self.data[i / 2];
        if i % 2 == 0 { b >> 4 } else { b & 0x0F }
    }

    /// Fast path: opaque image, streamed as one contiguous block
    pub fn draw<D>(&self, target: &mut D, top_left: Point, pal: &Palette) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let area = Rectangle::new(top_left, self.size);
        let count = (self.size.width * self.size.height) as usize;
        target.fill_contiguous(&area, (0..count).map(|i| pal[self.index_at(i) as usize]))
    }

    /// Sprite path: index 0 is transparent (slower, per-pixel)
    pub fn draw_transparent<D>(
        &self,
        target: &mut D,
        top_left: Point,
        pal: &Palette,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let w = self.size.width as usize;
        let pixels = (0..(w * self.size.height as usize)).filter_map(|i| {
            let idx = self.index_at(i);
            (idx != 0).then(|| {
                let p = Point::new((i % w) as i32, (i / w) as i32) + top_left;
                Pixel(p, pal[idx as usize])
            })
        });
        target.draw_iter(pixels)
    }
}
