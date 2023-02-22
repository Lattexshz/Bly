#![allow(clippy::single_match)]

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    let mut bly = bly::init(&window).unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::MainEventsCleared => {
                bly.draw(|bdc| {
                    let (width,height) = bdc.get_size();
                    bdc.clear(bly::Color::White);
                    bdc.draw_rect(50.0,50.0,50.0,50.0,bly::Color::Red);
                });
            }
            _ => (),
        }
    });
}
