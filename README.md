# Bly
## Rusty fast cross-platform 2D graphics library
[![rust-clippy analyze](https://github.com/Lattexshz/Bly/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/Lattexshz/Bly/actions/workflows/rust-clippy.yml)
![GitHub](https://img.shields.io/github/license/Lattexshz/Bly)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/Lattexshz/Bly)
![Lines of code](https://img.shields.io/tokei/lines/github/Lattexshz/Bly)
# Functions to be implemented

 - [X] Window background fill
 - [X] Rectangle Drawing
 - [ ] Line Drawing
 - [ ] Triangle Drawing

# Examples
```bash
git clone https://github.com/Lattexshz/Bly
cd Bly
cd bly
cargo run --example tiles
```


## How it can be coded
```Rust
#![allow(clippy::single_match)]

extern crate env_logger as logger;

use bly::{Color, Point2};

use std::env;

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

    let mut canvas = match bly::create_canvas(&window) {
        Ok(b) => b,
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

            Event::MainEventsCleared => canvas.draw(|painter| {
                painter.clear(Color::WhiteGray);

                painter.draw_rect(Point2::new(20.0,20.0), Point2::new(150.0,150.0), Color::Rgba(1.0, 1.0, 1.0, 1.0));
                painter.draw_rect(Point2::new(180.0,20.0), Point2::new(150.0,150.0), Color::Rgba(0.5, 0.5, 0.5, 1.0));
                painter.draw_rect(Point2::new(340.0,20.0), Point2::new(150.0,150.0), Color::Rgba(0.0, 0.0, 0.0, 1.0));

                painter.draw_rect(Point2::new(20.0,180.0), Point2::new(150.0,150.0), Color::Rgba(1.0, 0.0, 0.0, 1.0));
                painter.draw_rect(Point2::new(180.0,180.0), Point2::new(150.0,150.0), Color::Rgba(0.0, 1.0, 0.0, 1.0));
                painter.draw_rect(Point2::new(340.0,180.0), Point2::new(150.0,150.0), Color::Rgba(0.0, 0.0, 1.0, 1.0));

                painter.draw_rect(Point2::new(20.0,340.0), Point2::new(150.0,150.0), Color::Rgba(1.0, 1.0, 0.0, 1.0));
                painter.draw_rect(Point2::new(180.0,340.0), Point2::new(150.0,150.0), Color::Rgba(0.0, 1.0, 1.0, 1.0));
                painter.draw_rect(Point2::new(340.0,340.0), Point2::new(150.0,150.0), Color::Rgba(1.0, 0.0, 1.0, 1.0));
            }),
            _ => (),
        }
    });
}

```

## Output
#### Windows
![windows](res/img/tiles_windows.png)
#### XLib
![xlib](res/img/tiles_xlib.png)

# Crates

## bly
This is the core of bly. This crate is used to do the actual drawing

## bly-ac
Provides traits for abstraction of backend crates, etc., as described below

## bly-dx2d
Bly's drawing backend for Windows (uses Direct2D internally).

## bly-cairo
Bly's drawing backend for Unix (uses Cairo internally).