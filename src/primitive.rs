use raw_window_handle::RawWindowHandle;
use crate::{Color, HANDLE, Vec4};
use crate::platform_impl::{_draw_rect, get_color};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rectangle {
    pub(crate) vertex: Vec4,
    pub(crate) color: Color,
}

impl Rectangle {
    pub fn new(vertex: Vec4, color: Color) -> Self {
        Self { vertex, color }
    }
}


pub fn draw_rect(rect:Rectangle) -> Result<(),()> {
    let handle = match unsafe { HANDLE } {
        None => {
            error!("Method 'draw_rect' cannot be performed because Bly is not initialized.");
            return Err(());
        }
        Some(s) => s,
    };

    match handle {
        RawWindowHandle::UiKit(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::AppKit(handle) => {}
        RawWindowHandle::Orbital(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::Xlib(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::Xcb(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::Wayland(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::Drm(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::Gbm(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::Win32(handle) => unsafe {
            _draw_rect(handle.hwnd, rect);
        },
        RawWindowHandle::WinRt(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::Web(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::AndroidNdk(_) => {
            error!("This platform is not supported");
        }
        RawWindowHandle::Haiku(_) => {
            error!("This platform is not supported");
        }
        _ => {}
    };

    Ok(())
}