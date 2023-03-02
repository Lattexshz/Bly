use std::ffi::c_void;
use khronos_egl::Context;
use crate::{Backend, Point2};
use wayland_sys::client::*;
extern crate khronos_egl as egl;
use wayland_sys::client::WAYLAND_CLIENT_HANDLE;
use wayland_sys::egl::WAYLAND_EGL_HANDLE;

pub fn create_wayland_backend(surface: *mut c_void) -> Result<WaylandBackend, egl::Error> {
    let wl_display = unsafe {
        wayland_sys::ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_display_connect, ::std::ptr::null())
    };
    let egl = egl::Instance::new(egl::Static);
    let display = egl.get_display(wl_display as *mut std::ffi::c_void).unwrap();
    egl.initialize(display)?;

    let attributes = [
        egl::RED_SIZE, 8,
        egl::GREEN_SIZE, 8,
        egl::BLUE_SIZE, 8,
        egl::DEPTH_SIZE, 24,
        egl::SURFACE_TYPE,egl::OPENGL_ES2_BIT,
        egl::NONE
    ];

    let config = egl.choose_first_config(display, &attributes)?.expect("unable to find an appropriate EGL configuration");

    let context_attributes = [
		egl::CONTEXT_CLIENT_VERSION, 2,
        egl::NONE
    ];


    let egl_window = unsafe {
        wayland_sys::ffi_dispatch!(WAYLAND_EGL_HANDLE,wl_egl_window_create,wl_display as *mut wl_proxy,520,520)
    };

    let surface = unsafe {egl.create_window_surface(display,config,egl_window as *mut c_void,None)? };

    let context = egl.create_context(display, config, None, &context_attributes)?;

    egl.make_current(display,Some(surface),Some(surface),Some(context))?;

    Ok(WaylandBackend {
        egl,
        surface,
        display,
        wl_display,
        context
    })
}

#[doc(hidden)]
pub struct WaylandBackend {
    egl: egl::Instance<egl::Static>,
    surface: egl::Surface,
    display: egl::Display,
    wl_display: *mut wl_display,
    context: Context
}

impl Backend for WaylandBackend {
    unsafe fn begin_draw(&mut self) {
        unsafe {
            wayland_sys::ffi_dispatch!(WAYLAND_CLIENT_HANDLE,wl_display_dispatch_pending,self.wl_display);
        }
        gl::ClearColor(0.5, 0.3, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    unsafe fn flush(&mut self) {
        self.egl.swap_buffers(self.display,self.surface);
    }

    unsafe fn get_display_size(&mut self) -> (u32, u32) {
        todo!()
    }

    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        gl::ClearColor(0.5, 0.3, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
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
        todo!()
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
        todo!()
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
        todo!()
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
        todo!()
    }
}
