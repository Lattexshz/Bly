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

pub(crate) mod platform_impl;
pub mod primitive;

#[macro_use]
extern crate log;

use crate::platform_impl::{_fill, get_color};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

pub static mut HANDLE: Option<RawWindowHandle> = None;

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
pub fn init(handle: &impl HasRawWindowHandle) {
    unsafe {
        HANDLE = Some(handle.raw_window_handle());
    }
}

/// Fills the Window background with the specified color. (bly initialization is required)
pub fn fill(color: Color) -> Result<(), ()> {
    let handle = match unsafe { HANDLE } {
        None => {
            error!("Method 'draw_point' cannot be performed because Bly is not initialized.");
            return Err(());
        }
        Some(s) => s,
    };

    match handle {
        RawWindowHandle::UiKit(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::AppKit(handle) => {}
        RawWindowHandle::Orbital(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::Xlib(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::Xcb(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::Wayland(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::Drm(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::Gbm(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::Win32(handle) => unsafe {
            _fill(handle.hwnd, get_color(color));
        },
        RawWindowHandle::WinRt(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::Web(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::AndroidNdk(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::Haiku(_) => {
            error!("This platform is not supported");
        }
        _ => {}
    };

    Ok(())
}
