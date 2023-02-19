pub trait Backend {
    unsafe fn clear(&self, r: f32, g: f32, b: f32, a: f32);

    unsafe fn resize(&self, width: u32,height: u32);
}