#![allow(clippy::single_match)]

extern crate env_logger as logger;

use bly::{Bly, Color};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::env;
use winit::event::{KeyboardInput, ScanCode};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    env::set_var("RUST_LOG", "info");

    logger::init();
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(520.0, 520.0))
        .build(&event_loop)
        .unwrap();



    let mut bly = match bly::init(&window) {
        Ok(b) => b,
        Err(_) => {
            panic!("Can't initialize Bly!");
        }
    };

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
            } => match ch {
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
            },
            Event::MainEventsCleared => {
                bly.draw(| bdc| {
                    bdc.clear(color);
                    bdc.draw_rect(20.0,20.0, 150.0, 150.0, 1.0, 1.0, 1.0, 1.0);
                    bdc.draw_rect(180.0,20.0, 150.0, 150.0, 0.5, 0.5, 0.5, 1.0);
                    bdc.draw_rect(340.0,20.0, 150.0, 150.0, 0.0, 0.0, 0.0, 1.0);

                    bdc.draw_rect(20.0,180.0, 150.0, 150.0, 1.0, 0.0, 0.0, 1.0);
                    bdc.draw_rect(180.0,180.0, 150.0, 150.0, 0.0, 1.0, 0.0, 1.0);
                    bdc.draw_rect(340.0,180.0, 150.0, 150.0, 0.0, 0.0, 1.0, 1.0);

                    bdc.draw_rect(20.0,340.0, 150.0, 150.0, 1.0, 1.0, 0.0, 1.0);
                    bdc.draw_rect(180.0,340.0, 150.0, 150.0, 0.0, 1.0, 1.0, 1.0);
                    bdc.draw_rect(340.0,340.0, 150.0, 150.0, 1.0, 0.0, 1.0, 1.0);

                })
            }
            Event::LoopDestroyed => unsafe {},
            _ => (),
        }
    });
}