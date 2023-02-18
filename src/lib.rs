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

mod platform_impl;

#[macro_use]
extern crate log;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use crate::platform_impl::{_fill, get_color};

static mut HANDLE:Option<RawWindowHandle> = None;

/// Mainly used to store vertex information
pub struct Vec4(pub f64,pub f64,pub f64,pub f64);

/// Enumeration of colors defined by default.
/// Used to specify fill color, etc.
pub enum Color {
    White,
    WhiteGray,
    Gray,
    Black,
    Red,
    Green,
    Blue,
    Rgba(u32,u32,u32,u32)
}

/// Initialize bly
pub fn init(handle:&impl HasRawWindowHandle)
{
    unsafe {
        HANDLE = Some(handle.raw_window_handle());
    }
}

/// Fills the Window background with the specified color. (bly initialization is required)
pub fn fill(color: Color) -> Result<(),()>{
    let handle = match unsafe {HANDLE} {
        None => {
            error!("Method 'draw_point' cannot be performed because Bly is not initialized.");
            return Err(());
        }
        Some(s) => s
    };

    match handle {
        RawWindowHandle::UiKit(_) => {}
        RawWindowHandle::AppKit(handle) => {

        }
        RawWindowHandle::Orbital(_) => {}
        RawWindowHandle::Xlib(_) => {}
        RawWindowHandle::Xcb(_) => {}
        RawWindowHandle::Wayland(_) => {}
        RawWindowHandle::Drm(_) => {}
        RawWindowHandle::Gbm(_) => {}
        RawWindowHandle::Win32(handle) => {
            unsafe {
                _fill(handle.hwnd,get_color(color));
            }
        }
        RawWindowHandle::WinRt(_) => {}
        RawWindowHandle::Web(_) => {}
        RawWindowHandle::AndroidNdk(_) => {}
        RawWindowHandle::Haiku(_) => {}
        _ => {}
    };

    Ok(())
}
