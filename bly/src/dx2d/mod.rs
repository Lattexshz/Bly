//! dx2d

use crate::{Backend, Color, GradientType, Point4};
use crate::Point2;
use windows::{
    core::*, Foundation::Numerics::*, Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Direct2D::*, Win32::UI::WindowsAndMessaging::*,
};

#[doc(hidden)]
pub fn create_backend(hwnd: isize) -> std::result::Result<Direct2DBackend, ()> {
    let backend = Direct2DBackend::new(HWND(hwnd)).unwrap();

    Ok(backend)
}

#[doc(hidden)]
pub struct Direct2DBackend {
    handle: HWND,

    width: u32,
    height: u32,

    factory: ID2D1Factory1,
    target: ID2D1HwndRenderTarget,
}

impl Backend for Direct2DBackend {
    #[inline]
    unsafe fn begin_draw(&mut self) {
        self.update_target();
        self.target.BeginDraw();
    }

    #[inline]
    unsafe fn flush(&mut self) {
        self.target
            .EndDraw(std::ptr::null_mut(), std::ptr::null_mut())
            .unwrap();
    }

    #[inline]
    unsafe fn get_display_size(&mut self) -> (u32, u32) {
        unsafe {
            let size = self.target.GetSize();
            (size.width as u32, size.height as u32)
        }
    }

    #[inline]
    unsafe fn clear(&mut self, color: Color) {
        let size = &self.target.GetSize();
        match color {
            Color::Color(c) => {
                let point:Point4<f32> = c.into();
                let color = D2D1_COLOR_F { r:point.0, g:point.1, b:point.2, a:point.3 };
                self.target.Clear(&color);
            }
            Color::Gradient(c,gradient,position) => {
                let point:Point4<f32> = c[0].into();
                let color = D2D1_COLOR_F { r:point.0, g:point.1, b:point.2, a:point.3 };
                self.target.Clear(&color);
            }
        }
    }

    #[inline]
    unsafe fn ellipse(&mut self, point: Point2<f32>, radius: f32, color: Color) {
        match color {
            Color::Color(c) => {
                let point:Point4<f32> = c.into();
                let color = D2D1_COLOR_F { r:point.0, g:point.1, b:point.2, a:point.3 };

                let properties = D2D1_BRUSH_PROPERTIES {
                    opacity: 1.0,
                    transform: Matrix3x2::identity(),
                };

                let brush = &self
                    .target
                    .CreateSolidColorBrush(&color, &properties)
                    .unwrap();

                self.target.FillEllipse(
                    &mut D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: point.0 + radius,
                            y: point.1 + radius,
                        },
                        radiusX: radius,
                        radiusY: radius,
                    },
                    brush,
                );
            }
            Color::Gradient(c,gradient,position) => {
                let mut vec = vec![];
                let mut pos = 0.0;
                for i in c {
                    pos += 1.0;
                    let point:Point4<f32> = (*i).into();
                    let color = D2D1_COLOR_F { r:point.0, g:point.1, b:point.2, a:point.3 };

                    vec.push(D2D1_GRADIENT_STOP { position:pos, color });
                }

                let collection = self.target.CreateGradientStopCollection(vec.as_slice(), Default::default(), Default::default()).unwrap();

                let properties = D2D1_BRUSH_PROPERTIES {
                    opacity: 1.0,
                    transform: Matrix3x2::identity(),
                };

                match gradient {
                    GradientType::Linear(from, to) => {
                        let brush = self.target.CreateLinearGradientBrush(&D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES { startPoint: D2D_POINT_2F { x: from.0, y: from.1 }, endPoint: D2D_POINT_2F { x: to.0, y: to.1 } }, &properties, collection).unwrap();
                        self.target.FillEllipse(
                            &mut D2D1_ELLIPSE {
                                point: D2D_POINT_2F {
                                    x: point.0 + radius,
                                    y: point.1 + radius,
                                },
                                radiusX: radius,
                                radiusY: radius,
                            },
                            brush,
                        );
                    }
                }
            }
        }
    }

    #[inline]
    unsafe fn rectangle(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        color: Color,
    ) {
        match color {
            Color::Color(c) => {
                let point:Point4<f32> = c.into();
                let color = D2D1_COLOR_F { r:point.0, g:point.1, b:point.2, a:point.3 };

                let properties = D2D1_BRUSH_PROPERTIES {
                    opacity: 1.0,
                    transform: Matrix3x2::identity(),
                };

                let brush = &self
                    .target
                    .CreateSolidColorBrush(&color, &properties)
                    .unwrap();

                let rect = D2D_RECT_F {
                    left: point1.0,
                    right: point1.0 + point2.0,
                    top: point1.1,
                    bottom: point1.1 + point2.1,
                };

                self.target.FillRectangle(&rect, brush);
            }
            Color::Gradient(c,gradient,position) => {
                let mut vec = vec![];
                for i in c {
                    let point:Point4<f32> = (*i).into();
                    let color = D2D1_COLOR_F { r:point.0, g:point.1, b:point.2, a:point.3 };

                    vec.push(D2D1_GRADIENT_STOP { position, color });
                }

                let collection = self.target.CreateGradientStopCollection(vec.as_slice(), Default::default(), Default::default()).unwrap();

                let properties = D2D1_BRUSH_PROPERTIES {
                    opacity: 1.0,
                    transform: Matrix3x2::identity(),
                };

                let rect = D2D_RECT_F {
                    left: point1.0,
                    right: point1.0 + point2.0,
                    top: point1.1,
                    bottom: point1.1 + point2.1,
                };

                match gradient {
                    GradientType::Linear(from, to) => {
                        let brush = self.target.CreateLinearGradientBrush(&D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES { startPoint: D2D_POINT_2F { x: from.0, y: from.1 }, endPoint: D2D_POINT_2F { x: to.0, y: to.1 } }, &properties, collection).unwrap();
                        self.target.FillRectangle(&rect, brush);
                    }
                }
            }
        }
    }

    #[inline]
    unsafe fn rounded_rectangle(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        radius: f32,
        color: Color,
    ) {
        match color {
            Color::Color(c) => {
                let point:Point4<f32> = c.into();
                let color = D2D1_COLOR_F { r:point.0, g:point.1, b:point.2, a:point.3 };

                let properties = D2D1_BRUSH_PROPERTIES {
                    opacity: 1.0,
                    transform: Matrix3x2::identity(),
                };

                let brush = &self
                    .target
                    .CreateSolidColorBrush(&color, &properties)
                    .unwrap();

                let rect = D2D_RECT_F {
                    left: point1.0,
                    right: point1.0 + point2.0,
                    top: point1.1,
                    bottom: point1.1 + point2.1,
                };

                let rounded_rect = D2D1_ROUNDED_RECT {
                    rect,
                    radiusX: radius,
                    radiusY: radius,
                };

                self.target.FillRoundedRectangle(&rounded_rect, brush);
            }
            Color::Gradient(c,gradient,position) => {
                let mut vec = vec![];
                for i in c {
                    let point:Point4<f32> = (*i).into();
                    let color = D2D1_COLOR_F { r:point.0, g:point.1, b:point.2, a:point.3 };

                    vec.push(D2D1_GRADIENT_STOP { position, color });
                }

                let collection = self.target.CreateGradientStopCollection(vec.as_slice(), Default::default(), Default::default()).unwrap();

                let properties = D2D1_BRUSH_PROPERTIES {
                    opacity: 1.0,
                    transform: Matrix3x2::identity(),
                };

                let rect = D2D_RECT_F {
                    left: point1.0,
                    right: point1.0 + point2.0,
                    top: point1.1,
                    bottom: point1.1 + point2.1,
                };

                let rounded_rect = D2D1_ROUNDED_RECT {
                    rect,
                    radiusX: radius,
                    radiusY: radius,
                };

                match gradient {
                    GradientType::Linear(from, to) => {
                        let brush = self.target.CreateLinearGradientBrush(&D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES { startPoint: D2D_POINT_2F { x: from.0, y: from.1 }, endPoint: D2D_POINT_2F { x: to.0, y: to.1 } }, &properties, collection).unwrap();
                        self.target.FillRoundedRectangle(&rounded_rect, brush);
                    }
                }
            }
        }
    }

    #[inline]
    unsafe fn line(
        &mut self,
        point1: Point2<f32>,
        point2: Point2<f32>,
        stroke: f32,
        color: Color,
    ) {
        match color {
            Color::Color(c) => {
                let point:Point4<f32> = c.into();
                let color = D2D1_COLOR_F { r:point.0, g:point.1, b:point.2, a:point.3 };

                let properties = D2D1_BRUSH_PROPERTIES {
                    opacity: 1.0,
                    transform: Matrix3x2::identity(),
                };

                let brush = &self
                    .target
                    .CreateSolidColorBrush(&color, &properties)
                    .unwrap();

                let props = D2D1_STROKE_STYLE_PROPERTIES {
                    startCap: D2D1_CAP_STYLE_ROUND,
                    endCap: D2D1_CAP_STYLE_TRIANGLE,
                    ..Default::default()
                };

                let style = unsafe { self.factory.CreateStrokeStyle(&props, &[]).unwrap() };

                self.target.DrawLine(
                    D2D_POINT_2F {
                        x: point1.0,
                        y: point1.1,
                    },
                    D2D_POINT_2F {
                        x: point2.0,
                        y: point2.1,
                    },
                    brush,
                    stroke,
                    style,
                );
            }
            Color::Gradient(c,gradient,position) => {
                let mut vec = vec![];
                for i in c {
                    let point:Point4<f32> = (*i).into();
                    let color = D2D1_COLOR_F { r:point.0, g:point.1, b:point.2, a:point.3 };

                    vec.push(D2D1_GRADIENT_STOP { position, color });
                }

                let collection = self.target.CreateGradientStopCollection(vec.as_slice(), Default::default(), Default::default()).unwrap();

                let properties = D2D1_BRUSH_PROPERTIES {
                    opacity: 1.0,
                    transform: Matrix3x2::identity(),
                };

                let props = D2D1_STROKE_STYLE_PROPERTIES {
                    startCap: D2D1_CAP_STYLE_ROUND,
                    endCap: D2D1_CAP_STYLE_TRIANGLE,
                    ..Default::default()
                };

                let style = unsafe { self.factory.CreateStrokeStyle(&props, &[]).unwrap() };

                match gradient {
                    GradientType::Linear(from, to) => {
                        let brush = self.target.CreateLinearGradientBrush(&D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES { startPoint: D2D_POINT_2F { x: from.0, y: from.1 }, endPoint: D2D_POINT_2F { x: to.0, y: to.1 } }, &properties, collection).unwrap();
                        self.target.DrawLine(
                            D2D_POINT_2F {
                                x: point1.0,
                                y: point1.1,
                            },
                            D2D_POINT_2F {
                                x: point2.0,
                                y: point2.1,
                            },
                            brush,
                            stroke,
                            style,
                        );
                    }
                }
            }
        }
    }
}

fn create_target(hwnd: HWND, factory: &ID2D1Factory1) -> (ID2D1HwndRenderTarget, u32, u32) {
    let mut rect = RECT::default();

    unsafe {
        GetClientRect(hwnd, &mut rect);
    }

    let d2d_rect = D2D_SIZE_U {
        width: (rect.right - rect.left) as u32,
        height: (rect.bottom - rect.top) as u32,
    };

    let render_properties = D2D1_RENDER_TARGET_PROPERTIES::default();

    let hwnd_render_properties = D2D1_HWND_RENDER_TARGET_PROPERTIES {
        hwnd,
        pixelSize: d2d_rect,
        presentOptions: D2D1_PRESENT_OPTIONS_NONE,
    };

    let target = unsafe {
        factory
            .CreateHwndRenderTarget(&render_properties, &hwnd_render_properties)
            .unwrap()
    };
    (
        target,
        (rect.right - rect.left) as u32,
        (rect.bottom - rect.top) as u32,
    )
}

impl Direct2DBackend {
    /// Create a new backend
    fn new(hwnd: HWND) -> Result<Self> {
        let factory = create_factory()?;
        let (target, width, height) = create_target(hwnd, &factory);
        Ok(Self {
            handle: hwnd,
            width,
            height,
            factory,
            target,
        })
    }

    /// Regenerate Target (to accommodate window resizing)
    fn update_target(&mut self) {
        let mut rect = RECT::default();

        unsafe {
            GetClientRect(self.handle, &mut rect);
        }

        let d2d_rect = D2D_SIZE_U {
            width: (rect.right - rect.left) as u32,
            height: (rect.bottom - rect.top) as u32,
        };

        // Re-create Target only when the window size changes
        if self.width != d2d_rect.width || self.height != d2d_rect.height {
            self.width = d2d_rect.width;
            self.height = d2d_rect.height;

            let render_properties = D2D1_RENDER_TARGET_PROPERTIES::default();

            let hwnd_render_properties = D2D1_HWND_RENDER_TARGET_PROPERTIES {
                hwnd: self.handle,
                pixelSize: d2d_rect,
                presentOptions: D2D1_PRESENT_OPTIONS_NONE,
            };

            self.target = unsafe {
                self.factory
                    .CreateHwndRenderTarget(&render_properties, &hwnd_render_properties)
                    .unwrap()
            };
        }
    }
}

fn create_factory() -> Result<ID2D1Factory1> {
    let mut options = D2D1_FACTORY_OPTIONS::default();

    if cfg!(debug_assertions) {
        options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
    }

    let mut result = None;

    unsafe {
        D2D1CreateFactory(
            D2D1_FACTORY_TYPE_SINGLE_THREADED,
            &ID2D1Factory1::IID,
            &options,
            std::mem::transmute(&mut result),
        )
        .map(|()| result.unwrap())
    }
}
