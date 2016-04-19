extern crate sdl2;

use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use std::fmt;

#[allow(dead_code)]
struct Vector {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

#[allow(dead_code)]
impl Vector {
    fn new() -> Vector {
        Vector {w: 0, x: 0, y: 0, z: 0}
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.w, self.x, self.y, self.z)
    }
}

#[allow(unused_variables)]
fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let mut window = video.window("Rust", 640, 480).build().unwrap();
    window.show();
    let mut event_pump = sdl.event_pump().unwrap();
    {
        let mut surface = window.surface_mut(&event_pump).unwrap();
        let must_lock = surface.must_lock();
        let rect = surface.clip_rect();
        let color = Color::RGB(255, 255, 255);
        surface.fill_rect(rect, color);
    }
    {
        window.update_surface();
    }
    let mut done = false;
    while !done {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {timestamp, window_id, keycode, scancode,
                                keymod, repeat} => {
                    done = true;
                },
                _ => ()
            }
        }
    }
}

