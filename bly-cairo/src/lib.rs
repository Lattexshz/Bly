use std::ffi::c_ulong;
use cairo::ffi::{cairo_create, cairo_destroy, cairo_image_surface_create, cairo_surface_create_similar, cairo_surface_t, cairo_t, cairo_xlib_surface_create};
use cairo::Surface;
use x11::xlib::{XDefaultVisual, XGetGeometry, XOpenDisplay};
use bly_ac::Backend;


pub fn create_backend(window: c_ulong) -> CairoBackend {
    unsafe {
        let display = XOpenDisplay(std::ptr::null_mut());
        let surface = cairo_xlib_surface_create(display, window, XDefaultVisual(display,0), 100, 100);
        let cairo = cairo_create(surface);

        CairoBackend {
            surface,
            cairo
        }
    }
}

pub struct CairoBackend {
    surface: *mut cairo_surface_t,
    cairo: *mut cairo_t
}

impl Backend for CairoBackend {
    unsafe fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        todo!()
    }
}

impl Drop for CairoBackend {
    fn drop(&mut self) {
        unsafe {
        }
    }
}