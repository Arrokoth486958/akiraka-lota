use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{WindowBuilder, WindowButtons}, platform::windows::WindowExtWindows,
};

use crate::wgpu::WGPUInstance;

// struct LauncherState {
//     // TODO: 用来传递应用启动参数
// }

pub fn launch() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        // .with_titlebar_transparent(true)
        // .with_title_hidden(true)
        // .with_fullsize_content_view(true)
        // .with_blur(true)
        .with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE)
        .with_inner_size(LogicalSize::new(600, 400))
        .with_min_inner_size(LogicalSize::new(600, 400))
        .with_visible(false)
        // .with_transparent(false)
        .build(&event_loop)
        .unwrap();

    let monitor_size = window.current_monitor().unwrap().size();
    let window_size = window.inner_size();
    println!("{:?}", monitor_size);
    println!("{:?}", window_size);

    // TODO: 一些特定平台的函数
    #[cfg(target_os = "macos")]
    {
        // use platform::macos::WindowBuilderExtMacOS,
        use crate::{platform::macos::apply_empty_tool_bar, wgpu::WGPUInstance};

        crate::platform::macos::init_window(&window);
        // window.
    }
    #[cfg(target_os = "windows")]
    {
        window.set_decorations(false);
        window.set_undecorated_shadow(true);
        // window.drag_resize_window(winit::window::ResizeDirection::NorthWest);
        window.set_transparent(true);
    }

    // Wgpu实例
    let mut wgpu_instance = WGPUInstance::new(&window);

    window.set_visible(true);
    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } => {
                if window_id == window.id() {
                    match event {
                        WindowEvent::CloseRequested => {
                            elwt.exit();
                        }
                        WindowEvent::Resized(size) => {
                            // 在Windows下修复快捷键退出最小化情况下窗口在后台情况下
                            window.focus_window();
                            // TODO: 窗口更新的本地方法
                            // 刷新WGPU实例
                            wgpu_instance.resize(*size);
                        }
                        WindowEvent::ScaleFactorChanged {
                            ..
                        } => {
                            let size = window.inner_size();
                            // 或许有更好的解决方案
                            println!("{:?}", size);
                            wgpu_instance.resize(size);
                        }
                        _ => {
                            // println!("{:?}", event);
                        }
                    }
                }
            }
            _ => {}
        })
        .unwrap();
}
