#![allow(clippy::single_match)]

#[macro_use]
extern crate log;
extern crate env_logger as logger;

use std::env;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use windows::Win32::UI::WindowsAndMessaging::PostQuitMessage;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit::platform::windows::WindowExtWindows;

fn main() {
    env::set_var("RUST_LOG", "info");

    logger::init();
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();

    let mut backend = bly_dx2d::create_backend(window.hwnd()).unwrap();


    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::MainEventsCleared => {
                window.request_redraw();
                unsafe {
                    backend.clear(100.0,100.0,100.0,100.0);
                }
            },
            Event::LoopDestroyed => unsafe {
                backend.destroy();
            }
            _ => (),
        }
    });

}