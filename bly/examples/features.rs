#![allow(clippy::single_match)]

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use bly::Color;

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    window.set_title("A fantastic window!");

    let mut bly = bly::init(&window).unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => control_flow.set_exit(),
            Event::RedrawEventsCleared => {
                bly.draw(|bdc| {
                    bdc.clear(Color::WhiteGray);
                    let (width,height) = bdc.get_size();
                    bdc.draw_rect(10.0,10.0,100.0,100.0,Color::Red);
                    bdc.draw_ellipse(120.0,10.0,50.0,50.0,Color::Blue);

                });
            }
            _ => (),
        }
    });
}