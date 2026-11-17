use tauri::{AppHandle, Manager};

pub fn setup_window_effects(app: &mut tauri::App) {
    let handle = app.handle().clone();
    let handle_for_task = handle.clone();
    // The native window handle cannot be reached from inside the setup callback:
    // `window.hwnd()` round-trips through the event loop dispatcher, which is not
    // running yet, so it reports `HandleError::Unavailable`. `window-shadows-v2`
    // calls `unwrap()` on that result and aborts the process. Queue the work so it
    // runs on the main thread after the event loop starts instead.
    let _ = handle.run_on_main_thread(move || apply_window_effects(&handle_for_task));
}

fn apply_window_effects(app: &AppHandle) {
    apply_window_shadow(app);
    remove_window_border(app);
}

// Same DWM frame extension that `window-shadows-v2` performs, without the
// unguarded unwrap.
#[cfg(target_os = "windows")]
fn apply_window_shadow(app: &AppHandle) {
    use windows_sys::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea;
    use windows_sys::Win32::UI::Controls::MARGINS;

    let Some(window) = app.get_webview_window("main") else {
        return;
    };
    let Ok(hwnd) = window.hwnd() else {
        return;
    };

    let margins = MARGINS {
        cxLeftWidth: 1,
        cxRightWidth: 1,
        cyTopHeight: 1,
        cyBottomHeight: 1,
    };
    unsafe {
        DwmExtendFrameIntoClientArea(hwnd.0 as _, &margins);
    }
}

#[cfg(not(target_os = "windows"))]
fn apply_window_shadow(_app: &AppHandle) {}

#[cfg(target_os = "windows")]
fn remove_window_border(app: &AppHandle) {
    use windows_sys::Win32::Graphics::Dwm::{
        DwmSetWindowAttribute, DWMWA_BORDER_COLOR, DWMWA_COLOR_NONE,
    };

    let Some(window) = app.get_webview_window("main") else {
        return;
    };

    let Ok(hwnd) = window.hwnd() else {
        return;
    };

    let border_color = DWMWA_COLOR_NONE;
    unsafe {
        DwmSetWindowAttribute(
            hwnd.0 as _,
            DWMWA_BORDER_COLOR as u32,
            &border_color as *const _ as *const core::ffi::c_void,
            core::mem::size_of_val(&border_color) as u32,
        );
    }
}

#[cfg(not(target_os = "windows"))]
fn remove_window_border(_app: &AppHandle) {}
