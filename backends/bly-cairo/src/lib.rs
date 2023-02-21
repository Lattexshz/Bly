//! Cairo backend for Bly

mod util;

use std::f64::consts::PI;
use bly_ac::Backend;
use cairo::ffi::{cairo_arc, cairo_create, cairo_destroy, cairo_fill, cairo_image_surface_create, cairo_line_to, cairo_move_to, cairo_rectangle, cairo_scale, cairo_set_line_width, cairo_set_source_rgb, cairo_set_source_rgba, cairo_stroke, cairo_surface_create_similar, cairo_surface_t, cairo_t, cairo_xlib_surface_create};
use cairo::Surface;
use std::ffi::{c_double, c_int, c_ulong};
use x11::xlib::{Display, XDefaultVisual, XFlush, XGetGeometry, XOpenDisplay};

pub fn create_backend(window: c_ulong) -> CairoBackend {
    unsafe {
        let display = XOpenDisplay(std::ptr::null_mut());
        let (width, height) = util::get_xlib_window_size(display, window);

        let surface = cairo_xlib_surface_create(
            display,
            window,
            XDefaultVisual(display, 0),
            width as c_int,
            height as c_int,
        );
        let cairo = cairo_create(surface);

        CairoBackend {
            handle: window,
            display,
            width,
            height,
            surface,
            cairo,
        }
    }
}

pub struct CairoBackend {
    handle: c_ulong,
    display: *mut Display,

    width: c_ulong,
    height: c_ulong,

    surface: *mut cairo_surface_t,
    cairo: *mut cairo_t,
}

impl Backend for CairoBackend {
    unsafe fn begin_draw(&mut self) {
        let (width, height) = util::get_xlib_window_size(self.display, self.handle);
        self.scale(width, height);
    }

    unsafe fn flush(&mut self) {
        XFlush(self.display);
    }

    unsafe fn get_display_size(&mut self) -> (u32, u32) {
        let (width, height) = util::get_xlib_window_size(self.display, self.handle);
        (width as u32,height as u32)
    }

    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        let (width, height) = util::get_xlib_window_size(self.display, self.handle);

        cairo_set_source_rgb(self.cairo, r as c_double, g as c_double, b as c_double);
        cairo_rectangle(
            self.cairo,
            0 as c_double,
            0 as c_double,
            width as c_double,
            height as c_double,
        );
        cairo_fill(self.cairo);
    }

    unsafe fn draw_ellipse(&mut self, x: f32, y: f32, radius_x: f32, radius_y: f32, r: f32, g: f32, b: f32, a: f32) {
        cairo_set_source_rgb(self.cairo, r as c_double, g as c_double, b as c_double);
        cairo_rectangle(
            self.cairo,
            0 as c_double,
            0 as c_double,
            width as c_double,
            height as c_double,
        );
        cairo_arc(self.cairo, x as c_double, y as c_double, 3.0, 0.0, 2.0 * PI);

        cairo_fill(self.cairo);
    }

    unsafe fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, r: f32, g: f32, b: f32, a: f32) {
        cairo_set_source_rgba(self.cairo, r as c_double, g as c_double, b as c_double,a as c_double);
        cairo_rectangle(
            self.cairo,
            x as c_double,
            y as c_double,
            width as c_double,
            height as c_double,
        );
        cairo_fill(self.cairo);
    }

    unsafe fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, stroke: f32, r: f32, g: f32, b: f32, a: f32) {
        let (width,height) = util::get_xlib_window_size(self.display,self.handle);

        cairo_set_line_width(self.cairo,stroke as c_double);

        cairo_set_source_rgba(self.cairo, r as c_double, g as c_double, b as c_double,a as c_double);
        cairo_move_to(self.cairo,x1 as c_double, y1 as c_double);
        cairo_line_to(self.cairo, x2 as c_double, y2 as c_double);
    
        cairo_stroke(self.cairo);
    }
}

impl CairoBackend {
    unsafe fn scale(&mut self, width: c_ulong, height: c_ulong) {
        if width != self.width || height != self.height {
            self.surface = cairo_xlib_surface_create(
                self.display,
                self.handle,
                XDefaultVisual(self.display, 0),
                width as c_int,
                height as c_int,
            );
            self.cairo = cairo_create(self.surface);
        }
    }
}

impl Drop for CairoBackend {
    fn drop(&mut self) {
        unsafe {
            cairo_destroy(self.cairo);
        }
    }
}
