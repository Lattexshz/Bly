//! Cairo backend for Bly

use crate::{Backend, Point2};
use std::ffi::c_ulong;

mod util;
mod xlib;

pub fn create_xlib_backend(window: c_ulong) -> CairoBackend {
    xlib::create_backend(window)
}

pub struct CairoBackend {
    backend: Box<dyn Backend>,
}

impl Backend for CairoBackend {
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
        self.backend.clear(r, g, b, a);
    }

    unsafe fn ellipse(
        &mut self,
        point: Point2<f32>,
        radius: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        self.backend.ellipse(point, radius, r, g, b, a);
    }

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
