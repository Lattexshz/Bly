//! # Bly - The 2D graphics Library
//! ## Bly is a simple 2D graphics library made in Rust.
//!

#[macro_use]
extern crate log;
extern crate env_logger as logger;

use bly_ac::Backend;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

pub type Point2<T> = bly_ac::Point2<T>;

/// # Bly Drawing Context - Wrapper for Backend
/// Used for actual drawing  
pub struct Bdc {
    pub(crate) backend: Box<dyn Backend>,
}

impl Bdc {
    /// Requests Backend to process the start of drawing
    /// This method is called internally in Bly::draw(). Therefore,  
    /// it is not possible for the library user to call this method.
    pub(crate) fn begin_draw(&mut self) {
        unsafe {
            self.backend.begin_draw();
        }
    }

    /// Requests the backend to process the end of drawing
    /// This method is called internally in Bly::draw(). Therefore,   
    /// it is not possible for the library user to call this method.
    pub(crate) fn flush(&mut self) {
        unsafe {
            self.backend.flush();
        }
    }

    /// Get display size
    pub fn get_size(&mut self) -> (u32, u32) {
        unsafe { self.backend.get_display_size() }
    }

    /// Fills the window background with the specified color
    pub fn clear(&mut self, color: Color) {
        unsafe {
            let vec: Vec4 = color.into();
            self.backend
                .clear(vec.0 as f32, vec.1 as f32, vec.2 as f32, vec.3 as f32);
        }
    }

    /// Draws an ellipse
    pub fn draw_ellipse(&mut self, pos:Point2<f32>, radius: f32, color: Color) {
        unsafe {
            let vec: Vec4 = color.into();
            self.backend.draw_ellipse(
                pos,
                radius,
                vec.0 as f32,
                vec.1 as f32,
                vec.2 as f32,
                vec.3 as f32,
            );
        }
    }

    /// Draws a rectangle
    pub fn draw_rect(&mut self, pos:Point2<f32>,
                     size:Point2<f32>, color: Color) {
        unsafe {
            let vec: Vec4 = color.into();
            self.backend.draw_rect(
                pos,size,
                vec.0 as f32,
                vec.1 as f32,
                vec.2 as f32,
                vec.3 as f32,
            );
        }
    }

    pub fn draw_rounded_rect(&mut self,
        pos:Point2<f32>,
        size:Point2<f32>,
        radius:f32,
        color: Color) {
            let vec:Vec4 = color.into();
            unsafe {
                self.backend.draw_rounded_rect(pos,size,radius,vec.0 as f32,vec.1 as f32,vec.2 as f32,vec.3 as f32);
            }
        }

    /// Draws a line
    pub fn draw_line(&mut self, x1: f32, point1:Point2<f32>,point2:Point2<f32>, stroke: f32, color: Color) {
        unsafe {
            let vec: Vec4 = color.into();
            self.backend.draw_line(
                point1,
                point2,
                stroke,
                vec.0 as f32,
                vec.1 as f32,
                vec.2 as f32,
                vec.3 as f32,
            );
        }
    }
}

pub struct Bly {
    pub(crate) bdc: Bdc,
}

impl Bly {
    /// drawing via bdc.
    pub fn draw<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Bdc),
    {
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
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    WhiteGray,
    Gray,
    Black,
    Red,
    Green,
    Blue,
    Rgba(f32, f32, f32, f32),
}

// Converts a Color enumerator to a vector.
impl Into<Vec4> for Color {
    fn into(self) -> Vec4 {
        match self {
            Color::White => Vec4(255.0, 255.0, 255.0, 1.0),
            Color::WhiteGray => Vec4(0.9, 0.9, 0.9, 1.0),
            Color::Gray => Vec4(0.9, 0.9, 0.9, 1.0),
            Color::Black => Vec4(0.0, 0.0, 0.0, 255.0),
            Color::Red => Vec4(1.0, 0.0, 0.0, 1.0),
            Color::Green => Vec4(0.0, 1.0, 0.0, 1.0),
            Color::Blue => Vec4(0.0, 0.0, 1.0, 1.0),
            Color::Rgba(r, g, b, a) => Vec4(r as f64, g as f64, b as f64, a as f64),
        }
    }
}

/// Initialize bly  
/// If Backend is not supported or some error occurs during initialization, Err is returned.
pub fn init(handle: &impl HasRawWindowHandle) -> Result<Bly, ()> {
    let backend = match handle.raw_window_handle() {
        RawWindowHandle::UiKit(_) => return Err(()),
        #[cfg(target_os = "macos")]
        RawWindowHandle::AppKit(handle) => return Err(()),
        RawWindowHandle::Orbital(_) => return Err(()),
        #[cfg(target_os = "linux")]
        RawWindowHandle::Xlib(handle) => {
            info!("Platform: Xlib Drawing backend is Cairo");
            {
                bly_cairo::create_xlib_backend(handle.window)
            }
        }
        RawWindowHandle::Xcb(_) => return Err(()),
        #[cfg(target_os = "linux")]
        RawWindowHandle::Wayland(handle) => return Err(()),
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
            backend: Box::new(backend),
        },
    })
}
