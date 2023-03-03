# Your first Bly!

## Create cargo project
```bash
cargo new my-first-app
```

Then add the following dependency to Cargo.toml  
```toml
[dependencies]
bly = "0.1.2"
winit = "0.28.1"
```

Bly does not provide Windowing functionality. So how do you create a window and draw on it?  
Fortunately, Rust has two crates, winit for window management.  
With these you can easily create windows and use Bly!  

Put the following in main.rs
```rust
#![allow(clippy::single_match)]

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();

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
```
You can now display the window!  
Next, this window allows drawing using Bly.
After creating a window with WindowBuilder, write the following code  

```rust
let mut canvas = match bly::create_canvas(&window) {
    Ok(c) => c,
    Err(_) => {
        panic!("Can't initialize Bly!");
    }
};
```

Then you can get the Canvas.  
Now let's write the drawing code using this Canvas.  

```rust
canvas.draw(|painter| {
    painter.clear(Color::WhiteGray);
});
```

Overall Code
```rust
#![allow(clippy::single_match)]

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();

    let mut canvas = match bly::create_canvas(&window) {
        Ok(c) => c,
        Err(_) => {
            panic!("Can't initialize Bly!");
        }
    };

    canvas.draw(|painter| {
        painter.clear(bly::Color::WhiteGray);
    });

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
```
The following window will then appear

You did it, you got the drawing done in Bly!