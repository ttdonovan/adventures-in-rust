extern crate sdl2;

// ask the compiler to import the macros defined in `events` module
#[macro_use]
mod events;

use sdl2::pixels::Color;

// cannot call functions at top-level. However, `struct_events` is not
// a usual function: it's a macro
struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down
    }
}

fn main() {
    // initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // create a window
    let window = video.window("ArcadeRS Shooter", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    // prepare the events record
    let mut events = Events::new(sdl_context.event_pump().unwrap());

    loop {
        events.pump();

        if events.now.key_escape == Some(true) {
            break;
        }

        // render a full black window
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();
    }
}
