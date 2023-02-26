use std::ffi::c_void;
use khronos_egl::Context;
use wayland_client::protocol::wl_surface::WlSurface;
use crate::ac::{Backend, Point2};
extern crate khronos_egl as egl;

pub fn create_wayland_backend(surface: *mut c_void) -> Result<WaylandBackend, egl::Error> {
    let surface = surface as wayland_client::protocol::wl_surface::WlSurface;
    let egl = egl::Instance::new(egl::Static);

    let wayland_display = wayland_client::Display::connect_to_env().expect("unable to connect to the wayland server");
    let display = egl.get_display(wayland_display.get_display_ptr() as *mut std::ffi::c_void).unwrap();
    egl.initialize(display)?;

    let attributes = [
        egl::RED_SIZE, 8,
        egl::GREEN_SIZE, 8,
        egl::BLUE_SIZE, 8,
        egl::NONE
    ];

    let config = egl.choose_first_config(display, &attributes)?.expect("unable to find an appropriate EGL configuration");

    let context_attributes = [
        egl::CONTEXT_MAJOR_VERSION, 4,
        egl::CONTEXT_MINOR_VERSION, 0,
        egl::CONTEXT_OPENGL_PROFILE_MASK, egl::CONTEXT_OPENGL_CORE_PROFILE_BIT,
        egl::NONE
    ];

    let context = egl.create_context(display, config, None, &context_attributes)?;

    info!("Context:{:?}",context);

    Ok(WaylandBackend {
        surface,
        context
    })
}

pub struct WaylandBackend {
    surface: WlSurface,
    context: Context
}

impl Backend for WaylandBackend {
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

    unsafe fn draw_ellipse(&mut self, point: Point2<f32>, radius: f32, r: f32, g: f32, b: f32, a: f32) {
        todo!()
    }

    unsafe fn draw_rect(&mut self, point1: Point2<f32>, point2: Point2<f32>, r: f32, g: f32, b: f32, a: f32) {
        todo!()
    }

    unsafe fn draw_rounded_rect(&mut self, point1: Point2<f32>, point2: Point2<f32>, radius: f32, r: f32, g: f32, b: f32, a: f32) {
        todo!()
    }

    unsafe fn draw_line(&mut self, point1: Point2<f32>, point2: Point2<f32>, stroke: f32, r: f32, g: f32, b: f32, a: f32) {
        todo!()
    }
}