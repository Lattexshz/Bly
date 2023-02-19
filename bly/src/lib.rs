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
extern crate env_logger as logger;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use bly_ac::Backend;

pub struct Bly
{
    pub(crate) backend: Box<dyn Backend>,
}

impl Bly
{
    pub fn clear(&self, color: Color) {
        unsafe {
            let vec:Vec4 = color.into();
            self.backend.clear(vec.0 as f32, vec.1 as f32, vec.2 as f32, vec.3 as f32);
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

// Converts a Color enumerator to a vector.
impl Into<Vec4> for Color {
    fn into(self) -> Vec4 {
        match self {
            Color::White => Vec4(255.0, 255.0, 255.0, 0.0),
            Color::WhiteGray => Vec4(240.0, 240.0, 240.0, 0.0),
            Color::Gray => Vec4(128.0, 128.0, 128.0, 128.0),
            Color::Black => Vec4(0.0, 0.0, 0.0, 255.0),
            Color::Red => Vec4(255.0, 0.0, 0.0, 255.0),
            Color::Green => Vec4(0.0, 255.0, 0.0, 255.0),
            Color::Blue => Vec4(0.0, 0.0, 255.0, 255.0),
            Color::Rgba(r, g, b, a) => Vec4(r as f64, g as f64, b as f64, a as f64),
        }
    }
}

/// Initialize bly
pub fn init(handle: &impl HasRawWindowHandle) -> Bly
{
    let backend = match handle.raw_window_handle() {
        RawWindowHandle::UiKit(_) => panic!("This platform is not supported"),
        RawWindowHandle::AppKit(_) => panic!("This platform is not supported"),
        RawWindowHandle::Orbital(_) => panic!("This platform is not supported"),
        #[cfg(target_os="linux")]
        RawWindowHandle::Xlib(handle) => {
            info!("Platform: Xlib Drawing backend is Cairo");
            {
                bly_cairo::create_backend(handle.window)
            }
        },
        RawWindowHandle::Xcb(_) => panic!("This platform is not supported"),
        RawWindowHandle::Wayland(_) => panic!("This platform is not supported"),
        RawWindowHandle::Drm(_) => panic!("This platform is not supported"),
        RawWindowHandle::Gbm(_) => panic!("This platform is not supported"),
        #[cfg(target_os="windows")]
        RawWindowHandle::Win32(handle) => {
            info!("Platform: Win32 Drawing backend is Dx2D");
            {
                bly_dx2d::create_backend(handle.hwnd as isize).unwrap()
            }
        }
        RawWindowHandle::WinRt(_) => panic!("This platform is not supported"),
        RawWindowHandle::Web(_) => panic!("This platform is not supported"),
        RawWindowHandle::AndroidNdk(_) => panic!("This platform is not supported"),
        RawWindowHandle::Haiku(_) => panic!("This platform is not supported"),
        _ => panic!("This platform is not supported"),
    };
    info!("Successfully acquired backend");
    unsafe {backend.clear(255.0,255.0,255.0,255.0);}
    Bly {
        backend:Box::new(backend),
    }
}