// Simple floating eye icon window - Keep it simple!

use anyhow::Result;
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::UI::WindowsAndMessaging::*,
};

pub struct EyeWindow {
    hwnd: HWND,
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            
            // Draw simple eye icon (circle with dot)
            let brush = CreateSolidBrush(COLORREF::from(0x00000000)); // Black
            SelectObject(hdc, brush);
            
            // Draw eye shape (simplified - just a circle)
            Ellipse(hdc, 10, 10, 50, 50);
            
            // Draw pupil (smaller circle)
            let pupil_brush = CreateSolidBrush(COLORREF::from(0x00FFFFFF)); // White
            SelectObject(hdc, pupil_brush);
            Ellipse(hdc, 20, 20, 40, 40);
            
            DeleteObject(brush);
            DeleteObject(pupil_brush);
            EndPaint(hwnd, &ps);
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

impl EyeWindow {
    pub fn new() -> Result<Self> {
        unsafe {
            let class_name = w!("BlinkoEyeWindow");
            
            let wc = WNDCLASSW {
                hInstance: HINSTANCE::default(),
                lpfnWndProc: Some(window_proc),
                lpszClassName: class_name,
                style: CS_HREDRAW | CS_VREDRAW,
                ..Default::default()
            };
            
            RegisterClassW(&wc);
            
            // Create small topmost window (64x64 pixels)
            let hwnd = CreateWindowExW(
                WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
                class_name,
                w!("Blinko Eye"),
                WS_POPUP,
                100, // X position
                100, // Y position
                64,  // Width
                64,  // Height
                HWND::default(),
                None,
                None,
                None,
            )?;
            
            // Make it always on top and click-through
            SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                0,
                0,
                0,
                0,
                SET_WINDOW_POS_FLAGS::SWP_NOMOVE | SET_WINDOW_POS_FLAGS::SWP_NOSIZE,
            )?;
            
            // Note: For simplicity, we keep the window visible and clickable
            // To make it click-through, uncomment the following:
            // SetWindowLongW(
            //     hwnd,
            //     GWL_EXSTYLE,
            //     (GetWindowLongW(hwnd, GWL_EXSTYLE) as u32
            //         | WS_EX_LAYERED as i32
            //         | WS_EX_TRANSPARENT as i32) as i32,
            // );
            
            Ok(Self { hwnd })
        }
    }

    pub fn show(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
        }
    }

    pub fn hide(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE);
        }
    }

    pub async fn show_reminder(&self) -> Result<()> {
        // Simple notification - just show a message box
        unsafe {
            MessageBoxW(
                self.hwnd,
                w!("该休息了！看看远方 20 秒吧\n\nTime to rest! Look at something 20 feet away for 20 seconds."),
                w!("Blinko 提醒"),
                MESSAGEBOX_STYLE::MB_OK | MESSAGEBOX_STYLE::MB_ICONINFORMATION,
            );
        }
        Ok(())
    }

    pub async fn show_blink_reminder(&self) -> Result<()> {
        unsafe {
            MessageBoxW(
                self.hwnd,
                w!("眨眨眼吧！让眼睛放松一下\n\nBlink your eyes! Give them a rest."),
                w!("Blinko 提醒"),
                MESSAGEBOX_STYLE::MB_OK | MESSAGEBOX_STYLE::MB_ICONINFORMATION,
            );
        }
        Ok(())
    }

    pub async fn show_posture_reminder(&self) -> Result<()> {
        unsafe {
            MessageBoxW(
                self.hwnd,
                w!("坐姿提醒：请保持正确姿势\n\nPosture reminder: Please sit up straight."),
                w!("Blinko 提醒"),
                MESSAGEBOX_STYLE::MB_OK | MESSAGEBOX_STYLE::MB_ICONINFORMATION,
            );
        }
        Ok(())
    }
}

