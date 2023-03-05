use crate::{Backend, Point2};
use khronos_egl::Context;
use std::ffi::c_void;
use wayland_sys::client::*;
extern crate khronos_egl as egl;
use wayland_sys::client::WAYLAND_CLIENT_HANDLE;
use wayland_sys::egl::WAYLAND_EGL_HANDLE;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[doc(hidden)]
pub fn create_wayland_backend(wl_surface: *mut c_void) -> Result<WaylandBackend, egl::Error> {
    let wl_display = unsafe {
        wayland_sys::ffi_dispatch!(
            WAYLAND_CLIENT_HANDLE,
            wl_display_connect,
            ::std::ptr::null()
        )
    };
    let egl = egl::Instance::new(egl::Static);
    let display = egl
        .get_display(wl_display as *mut std::ffi::c_void)
        .unwrap();
    egl.initialize(display)?;

    let attributes = [
        egl::SURFACE_TYPE,
        egl::WINDOW_BIT,
        egl::RED_SIZE,
        8,
        egl::GREEN_SIZE,
        8,
        egl::BLUE_SIZE,
        8,
        egl::DEPTH_SIZE,
        24,
        egl::RENDERABLE_TYPE,
        egl::OPENGL_ES2_BIT,
        egl::NONE,
    ];

    let config = egl
        .choose_first_config(display, &attributes)?
        .expect("unable to find an appropriate EGL configuration");

    let context_attributes = [egl::CONTEXT_CLIENT_VERSION, 2, egl::NONE];

    let egl_window = unsafe {
        wayland_sys::ffi_dispatch!(
            WAYLAND_EGL_HANDLE,
            wl_egl_window_create,
            wl_display as *mut wl_proxy,
            520,
            520
        )
    };

    let surface =
        unsafe { egl.create_window_surface(display, config, egl_window as *mut c_void, None)? };

    let context = egl.create_context(display, config, None, &context_attributes)?;

    egl.make_current(display, Some(surface), Some(surface), Some(context))?;
    let gl = gl::load_with(|s| egl.get_proc_address(s).unwrap() as *const _);

    unsafe {
        use std::ffi::CStr;
        use std::os::raw::c_char;

        let v: *const c_char = gl::GetString(gl::VERSION) as *const i8;
        let c_str: &CStr = unsafe { CStr::from_ptr(v) };
        let str_: &str = c_str.to_str().unwrap();
        info!("Using {}", str_);
    }

    Ok(WaylandBackend {
        egl,
        surface,
        display,
        wl_display,
    })
}

#[doc(hidden)]
pub struct WaylandBackend {
    egl: egl::Instance<egl::Static>,
    surface: egl::Surface,
    display: egl::Display,
    wl_display: *mut wl_display,
}

impl Backend for WaylandBackend {
    #[inline]
    unsafe fn begin_draw(&mut self) {
        unsafe {
            wayland_sys::ffi_dispatch!(
                WAYLAND_CLIENT_HANDLE,
                wl_display_dispatch_pending,
                self.wl_display
            );
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    #[inline]
    unsafe fn flush(&mut self) {
        self.egl.swap_buffers(self.display, self.surface);
    }

    #[inline]
    unsafe fn get_display_size(&mut self) -> (u32, u32) {
        todo!()
    }

    #[inline]
    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {}

    #[inline]
    unsafe fn ellipse(&mut self, point: Point2<f32>, radius: f32, r: f32, g: f32, b: f32, a: f32) {
        todo!()
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
        //todo!()
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
        todo!()
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
        todo!()
    }
}
