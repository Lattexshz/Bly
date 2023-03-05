//! # Bly - The 2D graphics Library
//! ## Bly is a simple 2D graphics library made in Rust.
//!

#[macro_use]
extern crate log;
extern crate env_logger as logger;

use once_cell::sync::OnceCell;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

#[cfg(target_os = "linux")]
pub(crate) mod cairo;
#[cfg(target_os = "windows")]
mod dx2d;
#[cfg(target_os = "linux")]
mod unix;
#[cfg(target_os = "linux")]
pub(crate) mod wayland;
#[cfg(target_arch = "wasm32")]
mod web;

/// Trait for common back-end processing
pub trait Backend {
    // Initialize
    /// Processing to start drawing (initialization, etc.)
    /// # Safety
    /// Call the method from Painter
    unsafe fn begin_draw(&mut self);

    /// Processing to finish drawing
    /// # Safety
    /// Call the method from Painter
    unsafe fn flush(&mut self);
    /// Get display size
    /// # Safety
    /// Call the method from Painter
    unsafe fn get_display_size(&mut self) -> (u32, u32);

    /// Fills the window background with a specific color
    /// # Safety
    /// Call the method from Painter
    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32);

    // Primitives
    /// Draws a ellipse
    /// # Safety
    /// Call the method from Painter
    unsafe fn ellipse(&mut self, point: Point2<f32>, radius: f32, r: f32, g: f32, b: f32, a: f32);

    /// Draws a rectangle
    /// # Safety
    /// Call the method from Painter
    unsafe fn rectangle(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    );

    /// Draws a rounded rectangle
    /// # Safety
    /// Call the method from Painter
    unsafe fn rounded_rectangle(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        radius: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    );

    /// Draws a line
    /// # Safety
    /// Call the method from Painter
    unsafe fn line(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        stroke: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    );
}

/// Represents two points in two dimensions
pub struct Point2<T>(pub T, pub T);
impl<T> Point2<T> {
    pub fn new(a: T, b: T) -> Self {
        Self { 0: a, 1: b }
    }
}

/// Represents three points in two dimensions
pub struct Point3<T>(pub T, pub T, pub T);
impl<T> Point3<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Self { 0: a, 1: b, 2: c }
    }
}

/// Represents four points in two dimensions
pub struct Point4<T>(pub T, pub T, pub T, pub T);
impl<T> Point4<T> {
    pub fn new(a: T, b: T, c: T, d: T) -> Self {
        Self {
            0: a,
            1: b,
            2: c,
            3: d,
        }
    }
}

/// # Bly Drawing Context - Wrapper for Backend
/// Used for actual drawing  
pub struct Painter {
    pub(crate) backend: Box<dyn Backend>,
}

impl Painter {
    /// Requests Backend to process the start of drawing
    /// This method is called internally in Bly::draw(). Therefore,  
    /// it is not possible for the library user to call this method.
    #[inline]
    pub(crate) fn begin_draw(&mut self) {
        unsafe {
            self.backend.begin_draw();
        }
    }

    /// Requests the backend to process the end of drawing
    /// This method is called internally in Bly::draw(). Therefore,   
    /// it is not possible for the library user to call this method.
    #[inline]
    pub(crate) fn flush(&mut self) {
        unsafe {
            self.backend.flush();
        }
    }

    /// Get display size
    #[inline]
    pub fn get_size(&mut self) -> (u32, u32) {
        unsafe { self.backend.get_display_size() }
    }

    /// Fills the window background with the specified color
    #[inline]
    pub fn clear(&mut self, color: Color) {
        unsafe {
            let vec: Vec4 = color.into();
            self.backend
                .clear(vec.0 as f32, vec.1 as f32, vec.2 as f32, vec.3 as f32);
        }
    }

    /// Draws an ellipse
    #[inline]
    pub fn ellipse(&mut self, pos: Point2<f32>, radius: f32, color: Color) {
        unsafe {
            let vec: Vec4 = color.into();
            self.backend.ellipse(
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
    #[inline]
    pub fn rectangle(&mut self, pos: Point2<f32>, size: Point2<f32>, color: Color) {
        unsafe {
            let vec: Vec4 = color.into();
            self.backend.rectangle(
                pos,
                size,
                vec.0 as f32,
                vec.1 as f32,
                vec.2 as f32,
                vec.3 as f32,
            );
        }
    }

    #[inline]
    pub fn rounded_rectangle(
        &mut self,
        pos: Point2<f32>,
        size: Point2<f32>,
        radius: f32,
        color: Color,
    ) {
        let vec: Vec4 = color.into();
        unsafe {
            self.backend.rounded_rectangle(
                pos,
                size,
                radius,
                vec.0 as f32,
                vec.1 as f32,
                vec.2 as f32,
                vec.3 as f32,
            );
        }
    }

    /// Draws a line
    #[inline]
    pub fn line(&mut self, point1: Point2<f32>, point2: Point2<f32>, stroke: f32, color: Color) {
        unsafe {
            let vec: Vec4 = color.into();
            self.backend.line(
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

/// An interface to make Painter easier to use
/// (in fact, it is also a structure that hides Painter's behavior to some extent)
pub struct Canvas {
    pub(crate) painter: Painter,
}

impl Canvas {
    /// drawing via painter.
    pub fn draw<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Painter),
    {
        self.painter.begin_draw();
        f(&mut self.painter);
        self.painter.flush();
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
pub fn create_canvas(handle: &impl HasRawWindowHandle) -> Result<Canvas, ()> {
    //panic if the canvas has been created once
    static CANVAS_CREATED: OnceCell<()> = OnceCell::new();
    if CANVAS_CREATED.set(()).is_err() {
        error!("Creating EventLoop multiple times is not supported.");
        panic!("Creating EventLoop multiple times is not supported.");
    }

    #[cfg(feature = "experimental")]
    info!("You are using the experimental version of Bly");

    let backend = match handle.raw_window_handle() {
        RawWindowHandle::UiKit(_) => {
            info!("Platform: UiKit");
            error!("This platform is unsupported");
            return Err(());
        }
        #[cfg(target_os = "macos")]
        RawWindowHandle::AppKit(handle) => {
            info!("Platform: AppKit");
            error!("This platform is unsupported");
            return Err(());
        }
        RawWindowHandle::Orbital(_) => {
            info!("Platform: Orbital");
            error!("This platform is unsupported");
            return Err(());
        }
        #[cfg(target_os = "linux")]
        #[cfg(feature = "xlib")]
        RawWindowHandle::Xlib(handle) => {
            info!("Platform: Xlib Drawing backend is Cairo");
            {
                unix::create_xlib_backend(handle.window)
            }
        }
        RawWindowHandle::Xcb(_) => {
            info!("Platform: Xcb");
            error!("This platform is unsupported");
            return Err(());
        }
        #[cfg(target_os = "linux")]
        #[cfg(feature = "wayland")]
        RawWindowHandle::Wayland(handle) => {
            info!("Platform: Wayland Drawing backend is EGL");
            unix::create_wayland_backend(handle.surface)
        }
        RawWindowHandle::Drm(_) => {
            info!("Platform: Drm");
            error!("This platform is unsupported");
            return Err(());
        }
        RawWindowHandle::Gbm(_) => {
            info!("Platform: Gbm");
            error!("This platform is unsupported");
            return Err(());
        }
        #[cfg(target_os = "windows")]
        RawWindowHandle::Win32(handle) => {
            info!("Platform: Win32 Drawing backend is Dx2D");
            dx2d::create_backend(handle.hwnd as isize).unwrap()
        }
        RawWindowHandle::WinRt(_) => {
            info!("Platform: WinRt");
            error!("This platform is unsupported");
            return Err(());
        }
        #[cfg(target_arch = "wasm32")]
        RawWindowHandle::Web(handle) => {
            info!("Platform: Web Drawing backend is web-sys");
            web::create_backend(handle.id)
        }
        RawWindowHandle::AndroidNdk(_) => {
            info!("Platform: AndroidNDK");
            error!("This platform is unsupported");
            return Err(());
        }
        RawWindowHandle::Haiku(_) => {
            info!("Platform: Haiku");
            error!("This platform is unsupported");
            return Err(());
        }
        _ => {
            error!("Unknown platform");
            return Err(());
        }
    };
    info!("Successfully acquired backend");

    Ok(Canvas {
        painter: Painter {
            backend: Box::new(backend),
        },
    })
}
