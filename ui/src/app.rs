use wgpu::SurfaceError;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{WindowBuilder, WindowButtons},
};

use crate::wgpu::WGPUInstance;

// struct LauncherState {
//     // TODO: 用来传递应用启动参数
// }

pub fn launch() {
    let event_loop = EventLoop::new().unwrap();
    let mut window_builder = WindowBuilder::new()
        .with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE)
        .with_inner_size(LogicalSize::new(600, 400))
        .with_min_inner_size(LogicalSize::new(600, 400))
        .with_visible(false);

    // TODO: 一些特定平台的函数
    #[cfg(target_os = "macos")]
    {
        use crate::{platform::macos::apply_empty_tool_bar, wgpu::WGPUInstance};
        use winit::platform::macos::WindowBuilderExtMacOS;

        window_builder = window_builder
            .with_titlebar_transparent(true)
            .with_fullsize_content_view(true)
            .with_title_hidden(true);
    }
    #[cfg(target_os = "windows")]
    {
        use winit::platform::windows::WindowBuilderExtWindows;
        window_builder = window_builder
            .with_undecorated_shadow(true)
            // TODO: 自定义窗口还要等等
            // .with_decorations(false)
            .with_transparent(true);
    }

    let window = window_builder.build(&event_loop).unwrap();

    // let monitor_size = window.current_monitor().unwrap().size();
    // let window_size = window.inner_size();
    // println!("{:?}", monitor_size);
    // println!("{:?}", window_size);

    // TODO: 还是一些特定平台的函数
    #[cfg(target_os = "macos")]
    {
        crate::platform::macos::init_window(&window)
            .expect("Could not configure custom Window Settings for macOS!");
    }
    #[cfg(target_os = "windows")]
    {
        // use winit::platform::windows::WindowExtWindows;
        // window.set_decorations(false);
        // window.set_undecorated_shadow(true);
        // // window.drag_resize_window(winit::window::ResizeDirection::NorthWest);
        // window.set_transparent(true);
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
                        WindowEvent::ScaleFactorChanged { .. } => {
                            let size = window.inner_size();
                            // 或许有更好的解决方案
                            println!("{:?}", size);
                            wgpu_instance.resize(size);
                        }
                        WindowEvent::KeyboardInput { .. } => {
                            wgpu_instance.input(event);
                            wgpu_instance.update();
                        }
                        WindowEvent::RedrawRequested => {
                            wgpu_instance.update();
                            match wgpu_instance.render() {
                                Ok(()) => {}
                                Err(SurfaceError::Lost) => wgpu_instance.resize(wgpu_instance.size),
                                Err(SurfaceError::OutOfMemory) => elwt.exit(),
                                Err(e) => eprintln!("{}", e),
                            }
                            // if window_id == window.id() {
                            //     gl_state.update();
                            //     // 渲染并处理错误
                            //     match gl_state.render() {
                            //         Ok(_) => {}
                            //         Err(SurfaceError::Lost) => gl_state.resize(gl_state.size),
                            //         Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                            //         Err(e) => eprintln!("{}", e),
                            //     };
                            // }
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
