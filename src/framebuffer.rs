use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Dimensions;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Pixel;
use std::vec::IntoIter;

pub(crate) struct HeapBuffer<C: PixelColor + Copy> {
    pub data: Vec<C>,
    bounding_box: Rectangle,
}

impl<C: RgbColor> HeapBuffer<C> {
    pub fn new(bounding_box: Rectangle) -> HeapBuffer<C> {
        Self {
            data: vec![C::BLACK; (bounding_box.size.width * bounding_box.size.height) as usize],
            bounding_box,
        }
    }
}

impl<C: PixelColor> IntoIterator for HeapBuffer<C> {
    type Item = C;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<C: PixelColor> Dimensions for HeapBuffer<C> {
    fn bounding_box(&self) -> Rectangle {
        self.bounding_box
    }
}

impl<C: PixelColor> DrawTarget for HeapBuffer<C> {
    type Color = C;
    type Error = std::io::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let idx_end = self.data.len() - 1;
        for Pixel(point, color) in pixels {
            let idx = point.y as usize * self.bounding_box.size.width as usize + point.x as usize;
            self.data[idx.clamp(0, idx_end)] = color;
        }
        Ok(())
    }
}
