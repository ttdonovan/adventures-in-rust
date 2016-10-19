use phi::{Phi, View, ViewAction};
use sdl2::pixels::Color;

pub struct ViewA;

impl View for ViewA {
    fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
        let renderer = &mut context.renderer;
        let events = &mut context.events;


        if events.now.quit || events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if events.now.key_space == Some(true) {
            return ViewAction::ChangeView(Box::new(ViewB));
        }

        // render a full red window
        renderer.set_draw_color(Color::RGB(255, 0, 0));
        renderer.clear();

        ViewAction::None
    }
}

pub struct ViewB;

impl View for ViewB {
    fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
        let renderer = &mut context.renderer;
        let events = &mut context.events;


        if events.now.quit || events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if events.now.key_space == Some(true) {
            return ViewAction::ChangeView(Box::new(ViewA));
        }

        // render a full blue window
        renderer.set_draw_color(Color::RGB(0, 0, 255));
        renderer.clear();

        ViewAction::None
    }
}

// `Drop::drop` is called whenever the object isn't bound to any variable. This
// is how the SDL context and its components know when to release their memory:
// they get out of scope, `drop` is called, and they dispose of their resources.

impl Drop for ViewA {
    fn drop(&mut self) {
        println!("Dropped ViewA");
    }
}

impl Drop for ViewB {
    fn drop(&mut self) {
        println!("Dropped ViewB");
    }
}