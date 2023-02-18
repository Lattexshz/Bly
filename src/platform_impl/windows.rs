use std::ffi::{c_int, c_void};
use winapi::shared::minwindef::BYTE;
use winapi::shared::windef::{COLORREF, HWND};
use winapi::um::wingdi::{DC_BRUSH, DC_PEN, GetStockObject, Rectangle, RGB, SelectObject, SetDCBrushColor, SetDCPenColor};
use winapi::um::winuser::{GetDC, GetSystemMetrics, ReleaseDC, SM_CXSCREEN, SM_CYSCREEN};
use crate::Color;

pub fn get_color(color: Color) -> COLORREF{
    match color {
        Color::White => RGB(255,255,255),
        Color::WhiteGray => RGB(240,240,240),
        Color::Gray => RGB(128,128,128),
        Color::Black => RGB(0,0,0),
        Color::Red => RGB(255,0,0),
        Color::Green => RGB(0,255,0),
        Color::Blue => RGB(0,0,255),
        Color::Rgba(r,g,b,a) => RGB(r as BYTE, g as BYTE, b as BYTE)
    }
}

pub unsafe fn _fill(hwnd: *mut c_void,color:COLORREF) {
    let hdc = GetDC(hwnd as HWND);
    // Set brush color...
    SetDCBrushColor(hdc,color);
    SetDCPenColor(hdc, color);

    SelectObject(hdc, GetStockObject(DC_PEN as c_int));
    SelectObject(hdc, GetStockObject(DC_BRUSH as c_int));

    Rectangle(hdc, 0, 0, GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN));

    // Release hdc
    ReleaseDC(hwnd as HWND,hdc);
}