use crate::ac::{Backend, Point2};


pub fn create_backend(id:u32) -> WebBackend {
    use wasm_bindgen::JsCast;
    info!("WebBackend is being created. ID:{}",id);

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


    canvas.style().set_css_text(&format!("width: {}px; height: {}px;",width,height));

    // context.begin_path();
    //
    // // Draw the outer circle.
    // context
    //     .arc(75.0, 75.0, 50.0, 0.0, std::f64::consts::PI * 2.0)
    //     .unwrap();
    //
    // // Draw the mouth.
    // context.move_to(110.0, 75.0);
    // context.arc(75.0, 75.0, 35.0, 0.0, std::f64::consts::PI).unwrap();
    //
    // // Draw the left eye.
    // context.move_to(65.0, 65.0);
    // context
    //     .arc(60.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0)
    //     .unwrap();
    //
    // // Draw the right eye.
    // context.move_to(95.0, 65.0);
    // context
    //     .arc(90.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0)
    //     .unwrap();
    //
    // context.stroke();
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

pub struct WebBackend {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    window: web_sys::Window,

    width: f64,
    height: f64,

    r:f32,
    g:f32,
    b:f32,
    a:f32
}

impl Backend for WebBackend {
    unsafe fn begin_draw(&mut self) {
        let (width,height) = self.get_window_size();
        self.scale(width,height);
    }

    unsafe fn flush(&mut self) {

    }

    unsafe fn get_display_size(&mut self) -> (u32, u32) {
        let (width,height) = self.get_window_size();
        self.scale(width,height);
        (width,height)
    }

    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.set_rgba(r,g,b,a);
        self.canvas.style().set_css_text(&format!("background-color: rgba({},{},{},{});width: {}px; height: {}px;",(self.r*255.0) as u32,(self.g*255.0) as u32,(self.b*255.0) as u32,(self.a*255.0) as u32,self.width,self.height));
    }

    unsafe fn draw_ellipse(&mut self, point: Point2<f32>, radius: f32, r: f32, g: f32, b: f32, a: f32) {
        todo!()
    }

    unsafe fn draw_rect(&mut self, point1: Point2<f32>, point2: Point2<f32>, r: f32, g: f32, b: f32, a: f32) {
        todo!()
    }

    unsafe fn draw_rounded_rect(&mut self, point1: Point2<f32>, point2: Point2<f32>, radius: f32, r: f32, g: f32, b: f32, a: f32) {
        todo!()
    }

    unsafe fn draw_line(&mut self, point1: Point2<f32>, point2: Point2<f32>, stroke: f32, r: f32, g: f32, b: f32, a: f32) {
        todo!()
    }
}

impl WebBackend {
    pub fn set_rgba(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.r = r;
        self.g = g;
        self.b = b;
        self.a = a;
    }
    fn scale(&mut self,width:f32,height:f32) {
        self.width = width as f64;
        self.height = height as f64;
        self.canvas.style().set_css_text(&format!("background-color: green;width: {}px; height: {}px;",self.width as f64,self.height as f64));
    }

    fn get_window_size(&mut self) -> (f32,f32) {
        let width = self.window.inner_width().unwrap().as_f64().unwrap() as f32;
        let height = self.window.inner_height().unwrap().as_f64().unwrap() as f32;
        (width,height)
    }
}