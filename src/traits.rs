pub trait Drawable {
    fn draw(&self, buffer: &mut Vec<u32>, width: usize, height: usize);
}

// a rectangle to frame the objects
#[derive(Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: usize,
    pub height: usize,
}

// Spatial is a trait for the entities, which must have an x and y on the coordinate system
pub trait Spatial {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn position(&self) -> (i32, i32) {
        (self.x(), self.y())
    }

    fn frame(&self) -> Rect {
        Rect {
            x: self.x(),
            y: self.y(),
            width: self.width(),
            height: self.height(),
        }
    }

    fn intersects(&self, other: &dyn Spatial) -> bool {
        let this_frame = self.frame();
        let other_frame = other.frame();

        this_frame.x < other_frame.x + other_frame.width as i32 && 
        this_frame.x + this_frame.width as i32 > other_frame.x &&
        this_frame.y < other_frame.y + other_frame.height as i32 &&
        this_frame.y + this_frame.height as i32 > other_frame.y
    }
}