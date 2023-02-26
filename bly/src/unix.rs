use std::ffi::{c_ulong, c_void};
use crate::ac::{Backend, Point2};

pub(crate) fn create_xlib_backend(window: c_ulong) -> UnixBackend {
    UnixBackend {
        backend: Box::new(crate::cairo::create_xlib_backend(window))
    }
}

pub(crate) fn create_wayland_backend(surface: *mut c_void) -> UnixBackend {
    UnixBackend {
        backend: Box::new(crate::wayland::create_wayland_backend(surface).unwrap())
    }
}


pub struct UnixBackend {
    backend: Box<dyn Backend>
}

impl Backend for UnixBackend {
    unsafe fn begin_draw(&mut self) {
        self.backend.begin_draw();
    }

    unsafe fn flush(&mut self) {
        self.backend.flush();
    }

    unsafe fn get_display_size(&mut self) -> (u32, u32) {
        self.backend.get_display_size()
    }

    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.backend.clear(r,g,b,a);
    }

    unsafe fn draw_ellipse(&mut self, point: Point2<f32>, radius: f32, r: f32, g: f32, b: f32, a: f32) {
        self.backend.draw_ellipse(point,radius,r,g,b,a);
    }

    unsafe fn draw_rect(&mut self, point1: Point2<f32>, point2: Point2<f32>, r: f32, g: f32, b: f32, a: f32) {
        self.backend.draw_rect(point1,point2,r,g,b,a);
    }

    unsafe fn draw_rounded_rect(&mut self, point1: Point2<f32>, point2: Point2<f32>, radius: f32, r: f32, g: f32, b: f32, a: f32) {
        self.backend.draw_rounded_rect(point1,point2,radius,r,g,b,a);
    }

    unsafe fn draw_line(&mut self, point1: Point2<f32>, point2: Point2<f32>, stroke: f32, r: f32, g: f32, b: f32, a: f32) {
        self.backend.draw_line(point1,point2,stroke,r,g,b,a);
    }
}