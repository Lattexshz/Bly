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
        point1: Point2<f32>,
        point2: Point2<f32>,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    );

    unsafe fn draw_rounded_rect(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        radius:f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    );

    // Draws a line
    unsafe fn draw_line(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        stroke: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    );
}

pub struct Point2<T>(pub T,pub T);
impl<T> Point2<T> {
    pub fn new(a:T,b:T) -> Self {
        Self {
            0: a,
            1: b,
        }
    }
}
pub struct Point3<T>(pub T,pub T,pub T);
impl<T> Point3<T> {
    pub fn new(a:T,b:T,c:T) -> Self {
        Self {
            0: a,
            1: b,
            2: c,
        }
    }
}
pub struct Point4<T>(pub T,pub T,pub T,pub T);
impl<T> Point4<T> {
    pub fn new(a:T,b:T,c:T,d:T) -> Self {
        Self {
            0: a,
            1: b,
            2: c,
            3: d
        }
    }
}