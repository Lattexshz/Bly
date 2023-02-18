//! # Bly - The 2D graphics Library
//! ## Bly is a simple 2D graphics library made in Rust.
//!
//! ## Example
//! ```Rust
//! #![allow(clippy::single_match)]
//!
//! use raw_window_handle::HasRawWindowHandle;
//! use winit::{
//!     event::{Event, WindowEvent},
//!     event_loop::EventLoop,
//!     window::WindowBuilder,
//!};
//!
//! fn main() {
//!     let event_loop = EventLoop::new();
//!
//!     // Create window
//!     let window = WindowBuilder::new()
//!         .with_title("A fantastic window!")
//!         .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
//!         .build(&event_loop)
//!         .unwrap();
//!
//!    // Initialize bly with window.
//!    bly::init(&window);
//!
//!     // Run Application
//!     event_loop.run(move |event, _, control_flow| {
//!         control_flow.set_wait();
//!         match event {
//!             Event::WindowEvent {
//!                 event: WindowEvent::CloseRequested,
//!                 window_id,
//!             } if window_id == window.id() => control_flow.set_exit(),
//!             Event::MainEventsCleared => {
//!             },
//!             Event::RedrawEventsCleared => {
//!                 // Fills the Window with the specified color
//!                 bly::fill(bly::Color::Red);
//!             }
//!             _ => (),
//!         }
//!     });
//! }
//! ```

pub mod primitive;

#[macro_use]
extern crate log;

use crate::platform_impl::{_fill, get_color};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

pub struct Bly {
    pub(crate) backend: Backend
}

impl Bly {
    pub fn clear(&self,color: Color) {
        self.backend.clear(color_to_vec(color));
    }
}

pub struct Backend {
    #[cfg(target_os="windows")]
    backend: bly_dx2d::Direct2DBackend
}

impl Backend {
    #[cfg(target_os="windows")]
    pub(crate) fn new(handle: &impl HasRawWindowHandle) -> Self {
        match handle.raw_window_handle() {
            RawWindowHandle::Win32(handle) => {
                let backend = bly_dx2d::create_backend(handle.hwnd as isize).unwrap();
                return Self {
                    backend
                };
            }
            _ => {
                panic!("Unsupported platform");
            }
        }
    }

    pub fn clear(&self,color:Vec4) {
        unsafe {
            self.backend.clear(color.0 as f32, color.1 as f32, color.2 as f32, color.3 as f32).unwrap();
        }
    }
}

/// Mainly used to store vertex information
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec4(pub f64, pub f64, pub f64, pub f64);

/// Enumeration of colors defined by default.
/// Used to specify fill color, etc.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    White,
    WhiteGray,
    Gray,
    Black,
    Red,
    Green,
    Blue,
    Rgba(u32, u32, u32, u32),
}

/// Initialize bly
pub fn init(handle: &impl HasRawWindowHandle) -> Bly {
    Bly {
        backend: Backend::new(handle)
    }
}

/// Fills the Window background with the specified color. (bly initialization is required)
pub fn fill(color: Color) -> Result<(), ()> {
    Ok(())
}

fn color_to_vec(color: Color) -> Vec4 {
    match color {
        Color::White => Vec4(255.0,255.0,255.0,0.0),
        Color::WhiteGray => Vec4(240.0,240.0,240.0,0.0),
        Color::Gray => Vec4(128.0,128.0,128.0,128.0),
        Color::Black => Vec4(0.0,0.0,0.0,255.0),
        Color::Red => Vec4(255.0,0.0,0.0,255.0),
        Color::Green => Vec4(0.0,255.0,0.0,255.0),
        Color::Blue => Vec4(0.0,0.0,255.0,255.0),
        Color::Rgba(r,g,b,a) => Vec4(r as f64, g as f64, b as f64, a as f64)
    }
}