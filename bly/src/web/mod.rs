//! Drawing backend for the Web

use crate::{Backend, Point2};

#[doc(hidden)]
pub fn create_backend(id: u32) -> WebBackend {
    use wasm_bindgen::JsCast;
    info!("WebBackend is being created. ID:{}", id);

    let window = web_sys::window().unwrap();

    let width = window.inner_width().unwrap().as_f64().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();

    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("bly_canvas").unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let body = document.body().unwrap();

    canvas
        .style()
        .set_css_text(&format!("width: {}px; height: {}px;", width, height));

    body.append_child(&canvas).unwrap();
    WebBackend {
        canvas,
        context,
        window,

        width,
        height,
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    }
}

#[doc(hidden)]
pub struct WebBackend {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    window: web_sys::Window,

    width: f64,
    height: f64,

    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Backend for WebBackend {
    #[inline]
    unsafe fn begin_draw(&mut self) {
        let (width, height) = self.get_window_size();
        self.scale(width, height);
        self.context.begin_path();
    }

    #[inline]
    unsafe fn flush(&mut self) {
        self.context.fill();
    }

    #[inline]
    unsafe fn get_display_size(&mut self) -> (u32, u32) {
        let (width, height) = self.get_window_size();
        self.scale(width, height);
        (width as u32, height as u32)
    }

    #[inline]
    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str(&format!(
                "rgba({},{},{},{})",
                (r * 255.0) as u32,
                (g * 255.0) as u32,
                (b * 255.0) as u32,
                (a * 255.0) as u32
            )));
        self.context
            .fill_rect(0.0, 0.0, self.width, self.height + 300.0);
    }

    #[inline]
    unsafe fn ellipse(&mut self, point: Point2<f32>, radius: f32, r: f32, g: f32, b: f32, a: f32) {
        todo!()
    }

    #[inline]
    unsafe fn rectangle(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str(&format!(
                "rgba({},{},{},{})",
                (r * 255.0) as u32,
                (g * 255.0) as u32,
                (b * 255.0) as u32,
                (a * 255.0) as u32
            )));
        self.context.fill_rect(
            point1.0.into(),
            point1.1.into(),
            point2.0.into(),
            point2.1.into(),
        );
    }

    #[inline]
    unsafe fn rounded_rectangle(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        radius: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        todo!()
    }

    #[inline]
    unsafe fn line(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        stroke: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        self.context
            .set_fill_style(&wasm_bindgen::JsValue::from_str(&format!(
                "rgba({},{},{},{})",
                (r * 255.0) as u32,
                (g * 255.0) as u32,
                (b * 255.0) as u32,
                (a * 255.0) as u32
            )));
        self.context
            .set_stroke_style(&wasm_bindgen::JsValue::from_str(&format!(
                "rgba({},{},{},{});",
                (r * 255.0) as u32,
                (g * 255.0) as u32,
                (b * 255.0) as u32,
                (a * 255.0) as u32
            )));

        self.context.move_to(point1.0.into(), point1.1.into());
        self.context.line_to(point2.0.into(), point2.1.into());

        self.context.stroke();
    }
}

impl WebBackend {
    pub fn set_rgba(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.r = r;
        self.g = g;
        self.b = b;
        self.a = a;
    }
    fn scale(&mut self, width: f32, height: f32) {
        self.width = width as f64;
        self.height = height as f64;
        self.canvas.style().set_css_text(&format!(
            "width: {}px; height: {}px;",
            self.width as f64, self.height as f64
        ));
    }

    fn get_window_size(&mut self) -> (f32, f32) {
        let width = self.window.inner_width().unwrap().as_f64().unwrap() as f32;
        let height = self.window.inner_height().unwrap().as_f64().unwrap() as f32;
        (width, height)
    }
}
