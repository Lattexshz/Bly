pub trait Backend {

    // Initialize
    /// Processing to start drawing (initialization, etc.)
    unsafe fn begin_draw(&mut self);
    /// Processing to finish drawing
    unsafe fn flush(&mut self);

    /// Get display size
    unsafe fn get_display_size(&mut self) -> (u32,u32);

    /// Fills the window background with a specific color
    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32);

    // Primitives
    /// Draws a rectangle
    unsafe fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, r: f32, g: f32, b: f32, a: f32);
}
