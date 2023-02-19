mod util;

use std::ffi::{c_double, c_int, c_ulong};
use cairo::ffi::{cairo_create, cairo_destroy, cairo_fill, cairo_image_surface_create, cairo_rectangle, cairo_set_source_rgb, cairo_surface_create_similar, cairo_surface_t, cairo_t, cairo_xlib_surface_create};
use cairo::Surface;
use x11::xlib::{Display, XDefaultVisual, XGetGeometry, XOpenDisplay};
use bly_ac::Backend;


pub fn create_backend(window: c_ulong) -> CairoBackend {
    unsafe {
        let display = XOpenDisplay(std::ptr::null_mut());
        let (width,height) = util::get_xlib_window_size(display,window);
        let surface = cairo_xlib_surface_create(display, window, XDefaultVisual(display,0), width as c_int, height as c_int);
        let cairo = cairo_create(surface);

        CairoBackend {
            handle: window,
            display,

            surface,
            cairo
        }
    }
}

pub struct CairoBackend {
    handle: c_ulong,
    display:*mut Display,

    surface: *mut cairo_surface_t,
    cairo: *mut cairo_t
}

impl Backend for CairoBackend {
    unsafe fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        let (width,height) = util::get_xlib_window_size(self.display,self.handle);
        cairo_set_source_rgb(self.cairo, r as c_double, g as c_double, b as c_double);
        cairo_rectangle(self.cairo, 0 as c_double, 0 as c_double, width as c_double, height as c_double);
        cairo_fill(self.cairo);
    }
}

impl Drop for CairoBackend {
    fn drop(&mut self) {
        unsafe {
            cairo_destroy(self.cairo);
        }
    }
}