use crate::{Color, Vec4};
use raw_window_handle::RawWindowHandle;
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

pub fn draw_rect(rect: Rectangle) -> Result<(), ()> {
    Ok(())
}
