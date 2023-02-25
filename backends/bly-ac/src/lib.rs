//! Crate for common back-end processing and other variables.

/// Trait for common back-end processing
pub trait Backend {
    // Initialize
    /// Processing to start drawing (initialization, etc.)
    unsafe fn begin_draw(&mut self);
    /// Processing to finish drawing
    unsafe fn flush(&mut self);

    /// Get display size
    unsafe fn get_display_size(&mut self) -> (u32, u32);

    /// Fills the window background with a specific color
    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32);

    // Primitives
    /// Draws a ellipse
    unsafe fn draw_ellipse(
        &mut self,
        x: f32,
        y: f32,
        radius: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    );

    /// Draws a rectangle
    unsafe fn draw_rect(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    );

    unsafe fn draw_rounded_rect(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        radius:f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    );

    // Draws a line
    unsafe fn draw_line(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        stroke: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    );
}

pub struct Point2(pub f32,pub f32);
impl Point2 {
    pub fn new(a:f32,b:f32) -> Self {
        Self {
            0: a,
            1: b,
        }
    }
}
pub struct Point3(f32,f32,f32);
impl Point3 {
    pub fn new(a:f32,b:f32,c:f32) -> Self {
        Self {
            0: a,
            1: b,
            2: c,
        }
    }
}
pub struct Point4(f32,f32,f32,f32);
impl Point4 {
    pub fn new(a:f32,b:f32,c:f32,d:f32) -> Self {
        Self {
            0: a,
            1: b,
            2: c,
            3: d
        }
    }
}