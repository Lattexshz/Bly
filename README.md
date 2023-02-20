# Bly
## Rusty fast cross-platform 2D graphics library
[![rust-clippy analyze](https://github.com/Lattexshz/Bly/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/Lattexshz/Bly/actions/workflows/rust-clippy.yml)

# Functions to be implemented

 - [X] Window background fill
 - [X] Rectangle Drawing

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

use bly::Color;

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

    let mut bly = match bly::init(&window) {
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

            Event::MainEventsCleared => bly.draw(|bdc| {
                bdc.clear(Color::WhiteGray);
                bdc.draw_rect(20.0, 20.0, 150.0, 150.0, Color::Rgba(1.0, 1.0, 1.0, 1.0));
                bdc.draw_rect(180.0, 20.0, 150.0, 150.0, Color::Rgba(0.5, 0.5, 0.5, 1.0));
                bdc.draw_rect(340.0, 20.0, 150.0, 150.0, Color::Rgba(0.0, 0.0, 0.0, 1.0));

                bdc.draw_rect(20.0, 180.0, 150.0, 150.0, Color::Rgba(1.0, 0.0, 0.0, 1.0));
                bdc.draw_rect(180.0, 180.0, 150.0, 150.0, Color::Rgba(0.0, 1.0, 0.0, 1.0));
                bdc.draw_rect(340.0, 180.0, 150.0, 150.0, Color::Rgba(0.0, 0.0, 1.0, 1.0));

                bdc.draw_rect(20.0, 340.0, 150.0, 150.0, Color::Rgba(1.0, 1.0, 0.0, 1.0));
                bdc.draw_rect(180.0, 340.0, 150.0, 150.0, Color::Rgba(0.0, 1.0, 1.0, 1.0));
                bdc.draw_rect(340.0, 340.0, 150.0, 150.0, Color::Rgba(1.0, 0.0, 1.0, 1.0));
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