use raw_window_handle::HasRawWindowHandle;
use winit::window::Window;

use crate::Exception;

// pub const Y: f64 = 20.0;

#[cfg(target_os = "macos")]
pub fn init_window(window: &Window) -> Result<(), Exception> {
    // 效果不好的函数，使我的领带旋转
    // update_window(window)

    apply_empty_tool_bar(window).unwrap();
    // window_vibrancy::apply_vibrancy(window, NSVisualEffectMaterial::Sidebar, None, None)
    //     .expect("Could not apply Window Vibrancy!");
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn apply_empty_tool_bar(window: &Window) -> Result<(), Exception> {
    let raw_window_handle = window.raw_window_handle();
    match raw_window_handle {
        raw_window_handle::RawWindowHandle::AppKit(handle) => {
            extern crate objc;
            use cocoa::appkit::NSWindow;
            use cocoa::base::id;
            use objc::{class, msg_send, runtime::NO, sel, sel_impl};

            let ns_window = handle.ns_window as id;
            unsafe {
                // 居中
                NSWindow::center(ns_window);

                // 设置可以拖动窗口
                // NSWindow::setMovableByWindowBackground_(ns_window, NO);
                // NSWindow::setMovable_(ns_window, NO);

                // 标题栏或者说是工具栏
                let ns_toolbar = msg_send![class!(NSToolbar), new];
                ns_window.setToolbar_(ns_toolbar);
                ns_window.setToolbarStyle_(
                    cocoa::appkit::NSWindowToolbarStyle::NSWindowToolbarStyleUnifiedCompact,
                );
            }
        }
        _ => {}
    }

    Ok(())
}

// pub fn update_window(window: &Window) -> Result<(), Exception> {
//     let raw_window_handle = window.raw_window_handle();
//     match raw_window_handle {
//         #[cfg(target_os = "macos")]
//         raw_window_handle::RawWindowHandle::AppKit(handle) => {
//             extern crate objc;
//             use objc::{sel, sel_impl, msg_send, runtime::Object};
//             use cocoa::base::id;
//             use cocoa::appkit::{NSView, NSWindow, NSWindowTitleVisibility, NSWindowButton};
//             use cocoa::foundation::NSRect;

//             let ns_window = handle.ns_window as id;
//             unsafe {
//                 ns_window.setTitleVisibility_(NSWindowTitleVisibility::NSWindowTitleHidden);

//                 let close: *mut Object = ns_window.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
//                 let miniaturize = ns_window.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton);
//                 let zoom = ns_window.standardWindowButton_(NSWindowButton::NSWindowZoomButton);

//                 let title_bar_container_view = close.superview().superview();

//                 let close_rect: NSRect = msg_send![close, frame];
//                 let button_height = close_rect.size.height;
//                 println!("{:?}", button_height);

//                 let title_bar_frame_height = button_height + Y;
//                 let mut title_bar_rect = NSView::frame(title_bar_container_view);
//                 title_bar_rect.size.height = title_bar_frame_height;
//                 title_bar_rect.origin.y = NSView::frame(ns_window).size.height - title_bar_frame_height;
//                 let _: () = msg_send![title_bar_container_view, setFrame: title_bar_rect];
//                 let window_buttons = vec![close, miniaturize, zoom];
//                 let space_between = NSView::frame(miniaturize).origin.x - NSView::frame(close).origin.x;

//                 for (i, button) in window_buttons.into_iter().enumerate() {
//                     let mut rect: NSRect = NSView::frame(button);
//                     rect.origin.x = (Y - 4.0) + (i as f64 * space_between);
//                     button.setFrameOrigin(rect.origin);
//                 }
//             }
//         }
//         _ => {}
//     }

//     Ok(())
// }
