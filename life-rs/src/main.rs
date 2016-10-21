extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::{Rect};

use std::{thread, time};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::EventPump;

fn init<'a>() -> (Renderer<'a>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsytem = sdl_context.video().unwrap();

    let window = video_subsytem.window("demo", 400, 400)
      .position_centered()
      .opengl()
      .build()
      .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let event_pump = sdl_context.event_pump().unwrap();

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();

    (renderer, event_pump)
}

fn main() {
    let (mut r, mut e) = init();

    r.set_draw_color(Color::RGB(255, 0, 0));
    r.fill_rect(Rect::new(10, 10, 100, 100));
    r.present();

    'running:loop {
        for event in e.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => { break 'running },
                _ => {}
            }
        }
        thread::sleep(time::Duration::from_millis(10));
    }
}
