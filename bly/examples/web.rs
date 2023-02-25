#![allow(clippy::single_match)]

use std::env;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use bly::{Canvas, Color, Point2};

pub fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    // #[cfg(target_arch = "wasm32")]
    //     wasm::create_log_list(&window);

    let mut canvas = match bly::create_canvas(&window) {
        Ok(c) => c,
        Err(_) => {
            panic!("")
        }
    };

    #[cfg(target_arch = "wasm32")]
    wasm::draw(canvas);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;
    use winit::{event::Event, window::Window};
    use bly::{Canvas,Color};

    #[wasm_bindgen(start)]
    pub fn run() {
        console_log::init_with_level(log::Level::Debug).expect("error initializing logger");

        #[allow(clippy::main_recursion)]
        super::main();
    }

    pub fn draw(mut canvas: Canvas) {
        canvas.draw(|painter| {
            painter.clear(Color::Red);
        });
    }

    pub fn create_log_list(window: &Window) {
        use winit::platform::web::WindowExtWebSys;

        let canvas = window.canvas();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        // Set a background color for the canvas to make it easier to tell the where the canvas is for debugging purposes.
        canvas.style().set_css_text("background-color: blue;");
        body.append_child(&canvas).unwrap();

        let log_header = document.create_element("h2").unwrap();
        log_header.set_text_content(Some("Event Log"));
        body.append_child(&log_header).unwrap();

        let log_list = document.create_element("ul").unwrap();
        body.append_child(&log_list).unwrap();
    }

    pub fn log_event(log_list: &web_sys::Element, event: &Event<()>) {

        // Getting access to browser logs requires a lot of setup on mobile devices.
        // So we implement this basic logging system into the page to give developers an easy alternative.
        // As a bonus its also kind of handy on desktop.
        if let Event::WindowEvent { event, .. } = &event {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let log = document.create_element("li").unwrap();
            log.set_text_content(Some(&format!("{:?}", event)));
            log_list
                .insert_before(&log, log_list.first_child().as_ref())
                .unwrap();
        }
    }
}