pub trait Backend {
    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32);

    #[deprecated(since="0.1.0")]
    unsafe fn resize(&mut self, width: u32,height: u32);
}