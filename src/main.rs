use minifb::{Key, Window, WindowOptions};

mod entities {
    pub mod player;
    pub mod bug;
}
mod traits;

use crate::entities::player::Player;
use crate::entities::bug::Bug;
use crate::traits::{Drawable, Spatial};

/// Constant variables
const SCALE: usize = 3;
const TILESIZE: usize = 16 * SCALE;
const COL: usize = 12;
const ROW: usize = 16;
const WIDTH: usize = ROW * TILESIZE;
const HEIGHT: usize = COL * TILESIZE;

fn main() {
    // Buffer for drawing pixels
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Move the square with WASD - Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    window.set_target_fps(60);

    let mut player = Player {
        x: (WIDTH/2 + 16/2) as i32,
        y: (HEIGHT/2 + 16/2) as i32,
        size: 16 * SCALE,
        color: 0x004040BF, // Blue color
        speed: 5,
    };

    let mut bug = Bug {
        x: 0,
        y: 0,
        size: 2,
        color: 0x112349BF,
        speed: 3,
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::W) {
            player.y -= player.speed;
        }
        if window.is_key_down(Key::S) {
            player.y += player.speed;
        }
        if window.is_key_down(Key::A) {
            player.x -= player.speed;
        }
        if window.is_key_down(Key::D) {
            player.x += player.speed;
        }

        if bug.x > player.x {
            bug.x -= bug.speed;
        }
        if bug.y > player.y {
            bug.y -= bug.speed;
        }
        if bug.x < player.x {
            bug.x += bug.speed;
        }
        if bug.y < player.y {
            bug.y += bug.speed;
        }

        if bug.intersects(&player) {
            println!("HIT!");
        }
        
        // Entire screen is cleared to black at each start to prevent smearing
        for pixel in buffer.iter_mut() {
            *pixel = 0;
        }
        
        // Draw objects at new positions
        player.draw(&mut buffer, WIDTH, HEIGHT);
        bug.draw(&mut buffer, WIDTH, HEIGHT);

        // update the window instance
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}