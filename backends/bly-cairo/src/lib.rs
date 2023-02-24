//! Cairo backend for Bly

use bly_ac::Backend;
use std::ffi::{c_ulong, c_void};

mod util;
mod wayland;
mod xlib;

pub fn create_xlib_backend(window: c_ulong) -> CairoBackend {
    xlib::create_backend(window)
}

pub fn create_wayland_backend(surface: *mut c_void) -> CairoBackend {
    wayland::create_backend(surface)
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
        self.backend
            .draw_ellipse(x, y, radius, r, g, b, a);
    }

    unsafe fn draw_rect(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        self.backend.draw_rect(x, y, width, height, r, g, b, a);
    }

    unsafe fn draw_rounded_rect(&mut self, x: f32, y: f32, width: f32, height: f32, radius: f32, r: f32, g: f32, b: f32, a: f32) {
        self.backend.draw_rounded_rect(x,y,width,height,radius,r,g,b,a);
    }

    unsafe fn draw_line(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        stroke: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        self.backend.draw_line(x1, y1, x2, y2, stroke, r, g, b, a);
    }
}
