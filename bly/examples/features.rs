#![allow(clippy::single_match)]

use bly::{Color, Point2};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    window.set_title("A fantastic window!");

    let mut bly = bly::init(&window).unwrap();

    let point = Point2::new(1.0,1.0);

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
                    let (width, height) = bdc.get_size();
                    bdc.draw_rect(Point2::new(10.0,10.0), Point2::new(100.0,100.0), Color::Red);
                    bdc.draw_rounded_rect(Point2::new(120.0,10.0), Point2::new(100.0,100.0),10.0,Color::Red);
                    bdc.draw_ellipse(230.0,10.0,50.0,Color::Red);
                });
            }
            _ => (),
        }
    });
}
