use std::ffi::{c_int, c_void};
use std::ptr::null_mut;
use winapi::shared::windef::{HBRUSH, HWND, RECT};
use winapi::um::wingdi::{BLACK_BRUSH, BLACK_PEN, CreateSolidBrush, DC_BRUSH, DC_PEN, GetStockObject, GRAY_BRUSH, HOLLOW_BRUSH, Rectangle, RGB, SelectObject, SetDCBrushColor, SetDCPenColor, WHITE_PEN};
use winapi::um::winuser::{FillRect, GetDC, GetSystemMetrics, GetWindowInfo, ReleaseDC, SM_CXSCREEN, SM_CYSCREEN};

pub unsafe fn _fill(hwnd: *mut c_void) {
    let hdc = GetDC(hwnd as HWND);
    // Set brush color...
    SetDCBrushColor(hdc,RGB(255,0,0));
    SetDCPenColor(hdc, RGB(255,0,0));

    SelectObject(hdc, GetStockObject(DC_PEN as c_int));
    SelectObject(hdc, GetStockObject(DC_BRUSH as c_int));

    Rectangle(hdc, 0, 0, GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN));

    // Release hdc
    ReleaseDC(hwnd as HWND,hdc);
}