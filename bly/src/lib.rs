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

use bly_ac::Backend;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

pub struct Bdc {
    pub(crate) backend: Box<dyn Backend>,
}

/// # Bly Drawing Context
impl Bdc {

    pub(crate) fn begin_draw(&mut self) {
        unsafe {
            self.backend.begin_draw();
        }
    }

    pub(crate) fn flush(&mut self) {
        unsafe {
            self.backend.flush();
        }
    }

    pub fn clear(&mut self, color: Color) {
        unsafe {
            let vec: Vec4 = color.into();
            self.backend
                .clear(vec.0 as f32, vec.1 as f32, vec.2 as f32, vec.3 as f32);
        }
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            self.backend.draw_rect(x,y,width,height,r,g,b,a);
        }
    }
}

pub struct Bly {
    pub(crate) bdc: Bdc
}

impl Bly {
    pub fn draw<F>(&mut self,mut f: F) where
        F: FnMut(&mut Bdc) {
        self.bdc.begin_draw();
        f(&mut self.bdc);
        self.bdc.flush();
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
            Color::WhiteGray => Vec4(230.0, 230.0, 230.0, 0.0),
            Color::Gray => Vec4(0.9, 0.9, 0.9, 0.0),
            Color::Black => Vec4(0.0, 0.0, 0.0, 255.0),
            Color::Red => Vec4(255.0, 0.0, 0.0, 255.0),
            Color::Green => Vec4(0.0, 255.0, 0.0, 255.0),
            Color::Blue => Vec4(0.0, 0.0, 255.0, 255.0),
            Color::Rgba(r, g, b, a) => Vec4(r as f64, g as f64, b as f64, a as f64),
        }
    }
}

/// Initialize bly
pub fn init(handle: &impl HasRawWindowHandle) -> Result<Bly, ()> {
    let mut backend = match handle.raw_window_handle() {
        RawWindowHandle::UiKit(_) => return Err(()),
        #[cfg(target_os = "macos")]
        RawWindowHandle::AppKit(handle) => {
            bly_corefoundation::create_backend()
        },
        RawWindowHandle::Orbital(_) => return Err(()),
        #[cfg(target_os = "linux")]
        RawWindowHandle::Xlib(handle) => {
            info!("Platform: Xlib Drawing backend is Cairo");
            {
                bly_cairo::create_backend(handle.window)
            }
        }
        RawWindowHandle::Xcb(_) => return Err(()),
        RawWindowHandle::Wayland(_) => return Err(()),
        RawWindowHandle::Drm(_) => return Err(()),
        RawWindowHandle::Gbm(_) => return Err(()),
        #[cfg(target_os = "windows")]
        RawWindowHandle::Win32(handle) => {
            info!("Platform: Win32 Drawing backend is Dx2D");
            {
                bly_dx2d::create_backend(handle.hwnd as isize).unwrap()
            }
        }
        RawWindowHandle::WinRt(_) => return Err(()),
        RawWindowHandle::Web(_) => return Err(()),
        RawWindowHandle::AndroidNdk(_) => return Err(()),
        RawWindowHandle::Haiku(_) => return Err(()),
        _ => return Err(()),
    };
    info!("Successfully acquired backend");
    Ok(Bly {
        bdc: Bdc {
            backend: Box::new(backend)
        },
    })
}
