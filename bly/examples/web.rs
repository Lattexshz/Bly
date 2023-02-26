#![allow(clippy::single_match)]

use bly::{Color, Point2};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
#[macro_use]
extern crate log;
extern crate env_logger as logger;

pub fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")]
    wasm::create_log_list(&window);

    let mut canvas = match bly::create_canvas(&window) {
        Ok(c) => c,
        Err(_) => {
            panic!("")
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
                canvas.draw(|painter| {
                    painter.clear(Color::WhiteGray);

                    painter.draw_rect(
                        Point2::new(20.0, 20.0),
                        Point2::new(150.0, 150.0),
                        Color::Rgba(1.0, 1.0, 1.0, 1.0),
                    );
                    painter.draw_rect(
                        Point2::new(180.0, 20.0),
                        Point2::new(150.0, 150.0),
                        Color::Rgba(0.5, 0.5, 0.5, 1.0),
                    );
                    painter.draw_rect(
                        Point2::new(340.0, 20.0),
                        Point2::new(150.0, 150.0),
                        Color::Rgba(0.0, 0.0, 0.0, 1.0),
                    );

                    painter.draw_rect(
                        Point2::new(20.0, 180.0),
                        Point2::new(150.0, 150.0),
                        Color::Rgba(1.0, 0.0, 0.0, 1.0),
                    );
                    painter.draw_rect(
                        Point2::new(180.0, 180.0),
                        Point2::new(150.0, 150.0),
                        Color::Rgba(0.0, 1.0, 0.0, 1.0),
                    );
                    painter.draw_rect(
                        Point2::new(340.0, 180.0),
                        Point2::new(150.0, 150.0),
                        Color::Rgba(0.0, 0.0, 1.0, 1.0),
                    );

                    painter.draw_rect(
                        Point2::new(20.0, 340.0),
                        Point2::new(150.0, 150.0),
                        Color::Rgba(1.0, 1.0, 0.0, 1.0),
                    );
                    painter.draw_rect(
                        Point2::new(180.0, 340.0),
                        Point2::new(150.0, 150.0),
                        Color::Rgba(0.0, 1.0, 1.0, 1.0),
                    );
                    painter.draw_rect(
                        Point2::new(340.0, 340.0),
                        Point2::new(150.0, 150.0),
                        Color::Rgba(1.0, 0.0, 1.0, 1.0),
                    );

                    painter.draw_line(
                        Point2::new(0.0, 0.0),
                        Point2::new(500.0, 50.0),
                        1.0,
                        Color::Red,
                    );
                });

                window.request_redraw();
            }
            _ => (),
        }
    });
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use bly::{Canvas, Color};
    use wasm_bindgen::prelude::*;
    use winit::{event::Event, window::Window};

    #[wasm_bindgen(start)]
    pub fn run() {
        console_log::init_with_level(log::Level::Debug).expect("error initializing logger");

        #[allow(clippy::main_recursion)]
        super::main();
    }

    pub fn create_log_list(window: &Window) {
        use winit::platform::web::WindowExtWebSys;

        let canvas = window.canvas();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        // Set a background color for the canvas to make it easier to tell the where the canvas is for debugging purposes.
        canvas.set_id("bly_canvas");
        body.append_child(&canvas).unwrap();
    }
}
