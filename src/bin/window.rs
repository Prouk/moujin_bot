#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]

use beryllium::event::Event;
use beryllium::gl_window::{GlProfile, GlWindow};
use beryllium::init::{InitFlags, Sdl};
use beryllium::window::{Window, WindowFlags};
use zstring::zstr;

pub fn window_create() {
    let sdl = Sdl::init(InitFlags::EVERYTHING).expect("couldn't start SDL");
    #[cfg(target_os = "macos")]
    {
        sdl
            .gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
            .unwrap();
    }

    let _win = sdl
        .create_gl_window(
            zstr!(b"Hello Window"),
            Some((800, 600)),
            (800, 600),
            WindowFlags::RESIZABLE
        )
        .expect("couldn't make a window and context");

    'main_loop: loop {
        // handle events this frame
        while let Some(event) = sdl.poll_events().and_then(Result::ok) {
            match event {
                Event::Quit|
                _ => (),
            }
        }
        // now the events are clear.

        // here's where we could change the world state and draw.
    }
}