use crate::traits::Drawable;
use crate::traits::Spatial;

pub struct Bug {
    pub x: i32,
    pub y: i32,
    pub size: usize,
    pub color: u32,
    pub speed: i32,
}

impl Drawable for Bug {
    fn draw(&self, buffer: &mut Vec<u32>, screen_width: usize, screen_height: usize) {
        // a loop from the start and to the end point of height and width of the object
        for row in self.y..(self.y + self.size as i32) {
            for col in self.x..(self.x + self.size as i32) {
                if row >= 0 && col >= 0 && row < screen_height as i32 && col < screen_width as i32 {
                    let index = row as usize * screen_width + col as usize;
                    buffer[index] = self.color;
                }
            }
        }
    }
}

impl Spatial for Bug {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn width(&self) -> usize { self.size }
    fn height(&self) -> usize { self.size }
}