macro_rules! struct_events {
    (
        keyboard: { $( $k_alias:ident : $k_sdl:ident ),* }
    )
    => {
        use sdl2::EventPump;

        pub struct ImmediateEvents {
            // for every keyboard event there is an Option<bool>
            // Some(true) => was just pressed
            // Some(false) => was just released
            // None => nothing happening _now_
            $( pub $k_alias: Option<bool> ),*
        }

        impl ImmediateEvents {
            pub fn new() -> ImmediateEvents {
                ImmediateEvents {
                    // when reinitialized, nothing has yet happened all are set to None
                    $( $k_alias: None ),*
                }
            }
        }

        pub struct Events {
            pump: EventPump,
            pub now: ImmediateEvents,

            // true  => pressed
            // false => not pressed
            $( pub $k_alias: bool ),*
        }

        impl Events {
            pub fn new(pump: EventPump) -> Events {
                Events {
                    pump: pump,
                    now: ImmediateEvents::new(),

                    // by default, initialize every key with _not pressed_
                    $( $k_alias: false ),*
                }
            }

            pub fn pump(&mut self) {
                self.now = ImmediateEvents::new();

                for event in self.pump.poll_iter() {
                    use sdl2::event::Event::*;
                    use sdl2::keyboard::Keycode::*;

                    match event {
                        KeyDown { keycode, .. } => match keycode {
                            // $( ... ),* containing $k_sdl and $k_alias means:
                            // "for every element ($k_alias : $k_sdl) pair,
                            // check whether the keycode is Some($k_sdl). If
                            // it is, then set the $k_alias fields to true."
                            $(
                                Some($k_sdl) => {
                                    // prevent multiple presses when keeping a key down
                                    // was previously not pressed?
                                    if !self.$k_alias {
                                        // key pressed
                                        self.now.$k_alias = Some(true);
                                    }

                                    self.$k_alias = true;
                                }
                            ),* // and add a comma after every option
                            _ => {}
                        },

                        KeyUp { keycode, .. } => match keycode {
                            $(
                                Some($k_sdl) => {
                                    // key released
                                    self.now.$k_alias = Some(false);
                                    self.$k_alias = false;
                                }
                            ),* // and add a comma after every option
                            _ => {}
                        },

                        _ => {}
                    }
                }
            }
        }
    }
}
