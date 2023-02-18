mod platform_impl;

#[macro_use]
extern crate log;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use crate::platform_impl::{_fill, get_color};

static mut HANDLE:Option<RawWindowHandle> = None;

pub struct Vec4(pub f64,pub f64,pub f64,pub f64);

pub enum Color {
    White,
    WhiteGray,
    Gray,
    Black,
    Red,
    Green,
    Blue,
    Rgba(u32,u32,u32,u32)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn init(handle:&impl HasRawWindowHandle)
{
    unsafe {
        HANDLE = Some(handle.raw_window_handle());
    }
}

pub fn fill(color: Color) -> Result<(),()>{
    let handle = match unsafe {HANDLE} {
        None => {
            error!("Method 'draw_point' cannot be performed because Bly is not initialized.");
            return Err(());
        }
        Some(s) => s
    };

    match handle {
        RawWindowHandle::UiKit(_) => {}
        RawWindowHandle::AppKit(handle) => {

        }
        RawWindowHandle::Orbital(_) => {}
        RawWindowHandle::Xlib(_) => {}
        RawWindowHandle::Xcb(_) => {}
        RawWindowHandle::Wayland(_) => {}
        RawWindowHandle::Drm(_) => {}
        RawWindowHandle::Gbm(_) => {}
        RawWindowHandle::Win32(handle) => {
            unsafe {
                _fill(handle.hwnd,get_color(color));
            }
        }
        RawWindowHandle::WinRt(_) => {}
        RawWindowHandle::Web(_) => {}
        RawWindowHandle::AndroidNdk(_) => {}
        RawWindowHandle::Haiku(_) => {}
        _ => {}
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
