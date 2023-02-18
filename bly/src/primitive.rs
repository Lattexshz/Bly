use raw_window_handle::RawWindowHandle;
use crate::{Color, Vec4};
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

    Ok(())
}