use crate::{Backend, ColorType, Point2};
use std::ffi::{c_ulong, c_void};

#[doc(hidden)]
pub(crate) fn create_xlib_backend(window: c_ulong) -> UnixBackend {
    UnixBackend {
        backend: Box::new(crate::cairo::create_xlib_backend(window)),
    }
}

#[doc(hidden)]
pub(crate) fn create_wayland_backend(surface: *mut c_void) -> UnixBackend {
    UnixBackend {
        backend: Box::new(crate::wayland::create_wayland_backend(surface).unwrap()),
    }
}

#[doc(hidden)]
pub struct UnixBackend {
    /// Backend
    backend: Box<dyn Backend>,
}

impl Backend for UnixBackend {
    #[inline]
    unsafe fn begin_draw(&mut self) {
        self.backend.begin_draw();
    }

    #[inline]
    unsafe fn flush(&mut self) {
        self.backend.flush();
    }

    #[inline]
    unsafe fn get_display_size(&mut self) -> (u32, u32) {
        self.backend.get_display_size()
    }

    #[inline]
    unsafe fn clear(&mut self, color: ColorType) {
        self.backend.clear(color);
    }

    #[inline]
    unsafe fn ellipse(&mut self, point: Point2<f32>, radius: f32, r: f32, g: f32, b: f32, a: f32) {
        self.backend.ellipse(point, radius, r, g, b, a);
    }

    #[inline]
    unsafe fn rectangle(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        self.backend.rectangle(point1, point2, r, g, b, a);
    }

    #[inline]
    unsafe fn rounded_rectangle(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        radius: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        self.backend
            .rounded_rectangle(point1, point2, radius, r, g, b, a);
    }

    #[inline]
    unsafe fn line(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        stroke: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        self.backend.line(point1, point2, stroke, r, g, b, a);
    }
}
