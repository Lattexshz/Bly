#![allow(clippy::single_match)]

extern crate env_logger as logger;

use bly::Color;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::env;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit::event::{KeyboardInput, ScanCode};

fn main() {
    env::set_var("RUST_LOG", "info");

    logger::init();
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 120.0))
        .build(&event_loop)
        .unwrap();

    let mut bly = bly::init(&window);
    let mut color = Color::Gray;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter(ch),
                ..
            } => {
                match ch {
                    'r' => {
                        color = Color::Red;
                    }
                    'g' => {
                        color = Color::Green;
                    }
                    'b' => {
                        color = Color::Blue;
                    }
                    'w' => {
                        color = Color::WhiteGray;
                    }
                    _ => {}
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
                bly.clear(color);
            }
            Event::LoopDestroyed => unsafe {},
            _ => (),
        }
    });
}
