extern crate rand;
extern crate sdl2;

use rand::Rng;

use sdl2::pixels::Color;
use sdl2::rect::{Rect};

use std::{thread, time};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::EventPump;

const MAX_X: i32 = 199;
const MAX_Y: i32 = MAX_X;
const CELL_WIDTH: i32 = 5;
const CELL_HEIGHT: i32 = CELL_WIDTH;
const NCELLS: i32 = (MAX_X + 1) / CELL_WIDTH;

fn init<'a>() -> (Renderer<'a>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsytem = sdl_context.video().unwrap();

    let window = video_subsytem.window(
                    "demo", MAX_X as u32 + 1, MAX_Y as u32 + 1
                 )
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

fn life_random(ncells: i32) -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng();

    let mut v: Vec<Vec<bool>> = Vec::new();

    for i in 0..ncells {
        v.push(Vec::new());
        for _ in 0..ncells {
            v[i as usize].push(rng.gen());
        }
    }

    v
}

fn display_cell(r: &mut Renderer, row: i32, col: i32) {
    let x = CELL_WIDTH * col;
    let y = CELL_WIDTH * row;

    let cell_color = Color::RGB(255, 0, 0);
    r.set_draw_color(cell_color);
    r.fill_rect(Rect::new(x, y,
                          CELL_WIDTH as u32,
                          CELL_HEIGHT as u32)).unwrap();
}

fn display_frame(r: &mut Renderer, v: &Vec<Vec<bool>>) {
    r.set_draw_color(Color::RGB(200, 200, 200));
    r.clear();
    for i in 0..NCELLS {
        for j in 0..NCELLS {
            if v[i as usize][j as usize] {
                display_cell(r, i, j);
            }
        }
    }
    r.present();
}

fn inc(n: usize) -> usize {
    (n + 1) % (NCELLS as usize)
}

fn dec(n: usize) -> usize {
    if n == 0 {
        (NCELLS - 1) as usize
    } else {
        (n - 1) as usize
    }
}

fn count_surrounding(r: i32, c: i32, v: &Vec<Vec<bool>>) -> i32 {
    let r = r as usize;
    let c = c as usize;

    v[dec(r)][c] as i32 +
    v[r][dec(c)] as i32 +
    v[r][inc(c)] as i32 +
    v[dec(r)][dec(c)] as i32 +
    v[dec(r)][inc(c)] as i32 +
    v[inc(r)][inc(c)] as i32 +
    v[inc(r)][dec(c)] as i32

}

fn alive(r: i32, c: i32, v: &Vec<Vec<bool>>) -> bool {
    let n = count_surrounding(r, c, v);
    let curr = v[r as usize][c as usize] as i32;

    match (curr, n) {
        (1, 0...1) => false,
        (1, 4...8) => false,
        (1, 2...3) => true,
        (0, 3)     => true,
        (0, 0...2) => false,
        (0, 4...8) => false,
        _ => panic!("alive: error in match"),
    }
}

fn life_next(v: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut v2:Vec<Vec<bool>> = Vec::new();

    for i in 0..NCELLS {
            v2.push(Vec::new());
        for j in 0..NCELLS {
            if alive(i,  j, &v) {
                v2[i as usize].push(true);
            } else {
                v2[i as usize].push(false);
            }
        }
    }

    v2
}

fn main() {
    let (mut r, mut e) = init();
    let mut v = life_random(NCELLS);

    'running:loop {
        for event in e.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => { break 'running },
                _ => {}
            }
        }
        display_frame(&mut r, &v);
        v = life_next(v);
        thread::sleep(time::Duration::from_millis(50));
    }
}
