#![allow(clippy::single_match)]

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use bly::{ColorEnums, gradient, GradientType, Point2};
use bly::Color::{Color, Gradient};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 728.0))
        .build(&event_loop)
        .unwrap();

    let mut canvas = match bly::create_canvas(&window) {
        Ok(c) => c,
        Err(_) => {
            panic!("Can't initialize Bly!");
        }
    };

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::MainEventsCleared => {
                window.request_redraw();
                canvas.draw(|painter| {
                    painter.clear(Color(bly::ColorEnums::WhiteGray));

                    painter.rectangle(Point2(10.0, 10.0), Point2(1000.0,600.0), Gradient(&[ColorEnums::Red,ColorEnums::Green,ColorEnums::Blue,],GradientType::Linear(Point2(50.0,0.0),Point2(200.0,50.0)),1.0))
                });
            }
            _ => (),
        }
    });
}