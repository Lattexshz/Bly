use crate::CairoBackend;
use bly_ac::{Backend, Point2};
use std::ffi::c_void;

pub(crate) fn create_backend(surface: *mut c_void) -> CairoBackend {
    unsafe {
        CairoBackend {
            backend: Box::new(WayLandBackend {}),
        }
    }
}

pub struct WayLandBackend {}

impl Backend for WayLandBackend {
    unsafe fn begin_draw(&mut self) {
        todo!()
    }

    unsafe fn flush(&mut self) {
        todo!()
    }

    unsafe fn get_display_size(&mut self) -> (u32, u32) {
        todo!()
    }

    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        todo!()
    }

    unsafe fn draw_ellipse(
        &mut self,
        x: f32,
        y: f32,
        radius: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        todo!()
    }

    unsafe fn draw_rect(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        todo!()
    }

    unsafe fn draw_rounded_rect(&mut self,
                                point1: Point2<f32>,
                                point2: Point2<f32>,
                                radius:f32,
                                r: f32,
                                g: f32,
                                b: f32,
                                a: f32,) {
        todo!()
    }

    unsafe fn draw_line(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        stroke: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        todo!()
    }
}
