#![allow(clippy::single_match)]

use raw_window_handle::HasRawWindowHandle;
use std::env;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use bly::primitive::Rectangle;
use bly::{Color, Vec4};

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)
        .unwrap();

    bly::init(&window);


    let rect = Rectangle::new(Vec4(10.0,500.0,300.0,300.0),Color::Red);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::MainEventsCleared => {}
            Event::RedrawEventsCleared => {
                bly::fill(bly::Color::Blue);
                bly::primitive::draw_rect(rect);
            }
            _ => (),
        }
    });
}