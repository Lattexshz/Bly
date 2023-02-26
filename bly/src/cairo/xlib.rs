use crate::ac::{Backend, Point2};
use crate::cairo::{util, CairoBackend};
use cairo_sys::{
    cairo_arc, cairo_close_path, cairo_create, cairo_destroy, cairo_fill, cairo_fill_preserve,
    cairo_line_to, cairo_move_to, cairo_new_sub_path, cairo_rectangle, cairo_set_line_width,
    cairo_set_source_rgb, cairo_set_source_rgba, cairo_stroke, cairo_surface_t, cairo_t,
    cairo_xlib_surface_create,
};
use std::f64::consts::PI;
use std::ffi::{c_double, c_int, c_ulong};
use x11::xlib::{Display, XDefaultVisual, XFlush, XGetGeometry, XOpenDisplay};

pub(crate) fn create_backend(window: c_ulong) -> CairoBackend {
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
            backend: Box::new(XLibBackend {
                handle: window,
                display,
                width,
                height,
                surface,
                cairo,
            }),
        }
    }
}

pub(crate) struct XLibBackend {
    handle: c_ulong,
    display: *mut Display,

    width: c_ulong,
    height: c_ulong,

    surface: *mut cairo_surface_t,
    cairo: *mut cairo_t,
}

impl Backend for XLibBackend {
    unsafe fn begin_draw(&mut self) {
        let (width, height) = get_xlib_window_size(self.display, self.handle);
        self.scale(width, height);
    }

    unsafe fn flush(&mut self) {
        XFlush(self.display);
    }

    unsafe fn get_display_size(&mut self) -> (u32, u32) {
        let (width, height) = get_xlib_window_size(self.display, self.handle);
        (width as u32, height as u32)
    }

    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        let (width, height) = get_xlib_window_size(self.display, self.handle);

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

    unsafe fn draw_ellipse(
        &mut self,
        point: Point2<f32>,
        radius: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        cairo_set_source_rgba(
            self.cairo,
            r as c_double,
            g as c_double,
            b as c_double,
            a as c_double,
        );

        cairo_arc(
            self.cairo,
            (point.0 + radius) as c_double,
            (point.1 + radius) as c_double,
            radius as c_double,
            0.0,
            2.0 * PI,
        );

        cairo_fill(self.cairo);
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
        cairo_set_source_rgba(
            self.cairo,
            r as c_double,
            g as c_double,
            b as c_double,
            a as c_double,
        );
        cairo_rectangle(
            self.cairo,
            point1.0 as c_double,
            point1.1 as c_double,
            point2.0 as c_double,
            point2.1 as c_double,
        );
        cairo_fill(self.cairo);
    }

    unsafe fn draw_rounded_rect(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        radius: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        cairo_set_source_rgba(
            self.cairo,
            r as c_double,
            g as c_double,
            b as c_double,
            a as c_double,
        );

        let degrees: f32 = (PI / 180.0) as f32;

        cairo_new_sub_path(self.cairo);
        cairo_arc(
            self.cairo,
            (point1.0 + point2.0 - radius) as c_double,
            (point1.1 + radius) as c_double,
            radius as c_double,
            (-90.0 * degrees) as c_double,
            (0.0 * degrees) as c_double,
        );
        cairo_arc(
            self.cairo,
            (point1.0 + point2.0 - radius) as c_double,
            (point1.1 + point2.1 - radius) as c_double,
            radius as c_double,
            (0.0 * degrees) as c_double,
            (90.0 * degrees) as c_double,
        );
        cairo_arc(
            self.cairo,
            (point1.0 + radius) as c_double,
            (point1.1 + point2.1 - radius) as c_double,
            radius as c_double,
            (90.0 * degrees) as c_double,
            (180.0 * degrees) as c_double,
        );
        cairo_arc(
            self.cairo,
            (point1.0 + radius) as c_double,
            (point1.1 + radius) as c_double,
            radius as c_double,
            (180.0 * degrees) as c_double,
            (270.0 * degrees) as c_double,
        );
        cairo_close_path(self.cairo);
        cairo_fill_preserve(self.cairo);
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
        cairo_set_line_width(self.cairo, stroke as c_double);

        cairo_set_source_rgba(
            self.cairo,
            r as c_double,
            g as c_double,
            b as c_double,
            a as c_double,
        );
        cairo_move_to(self.cairo, point1.0 as c_double, point1.1 as c_double);
        cairo_line_to(self.cairo, point2.0 as c_double, point2.1 as c_double);

        cairo_stroke(self.cairo);
    }
}

impl XLibBackend {
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

impl Drop for XLibBackend {
    fn drop(&mut self) {
        unsafe {
            cairo_destroy(self.cairo);
        }
    }
}

pub(crate) unsafe fn get_xlib_window_size(
    display: *mut Display,
    window: c_ulong,
) -> (c_ulong, c_ulong) {
    let mut width = 0;
    let mut height = 0;
    let mut dummy = 0;
    let mut c_int_dummy = 0;
    let mut c_uint_dummy = 0;
    XGetGeometry(
        display,
        window,
        &mut dummy,
        &mut c_int_dummy,
        &mut c_int_dummy,
        &mut width,
        &mut height,
        &mut c_uint_dummy,
        &mut c_uint_dummy,
    );

    (width.into(), height.into())
}
