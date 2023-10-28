use std::time::{SystemTime, UNIX_EPOCH};

use wgpu::SurfaceError;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{WindowBuilder, WindowButtons},
};

use crate::wgpu::{RenderObject, Vertex, WGPUInstance};

// struct LauncherState {
//     // TODO: 用来传递应用启动参数
// }

pub fn launch() {
    let event_loop = EventLoop::new().unwrap();
    let mut window_builder = WindowBuilder::new()
        .with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE)
        .with_inner_size(LogicalSize::new(600, 400))
        .with_min_inner_size(LogicalSize::new(600, 400))
        .with_title("Akiraka - Lota")
        .with_visible(false);

    // TODO: 一些特定平台的函数
    #[cfg(target_os = "macos")]
    {
        // use crate::{platform::macos::apply_empty_tool_bar, wgpu::WGPUInstance};
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
    // event_loop.set_control_flow(ControlFlow::Wait);
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
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
                            wgpu_instance.resize(&window, *size);
                            #[cfg(target_os = "macos")]
                            window.request_redraw();
                        }
                        WindowEvent::ScaleFactorChanged { .. } => {
                            let size = window.inner_size();
                            // 或许有更好的解决方案
                            println!("{:?}", size);
                            wgpu_instance.resize(&window, size);
                        }
                        WindowEvent::KeyboardInput { .. } => {
                            wgpu_instance.input(event);
                            wgpu_instance.update();
                        }
                        WindowEvent::RedrawRequested => {
                            // TODO: Debug
                            let x = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_millis()
                                - start_time;
                            let x: f32 = (x as f32 / 10000.0).to_degrees().sin();
                            wgpu_instance.render_objects.push(RenderObject::new(
                                vec![
                                    Vertex {
                                        position: [-1.0 * x, -1.0 * x, 0.0],
                                        color: [0.0, 0.0, 0.0],
                                        tex_coords: [0.0, 0.0],
                                    },
                                    Vertex {
                                        position: [1.0 * x, -1.0 * x, 0.0],
                                        color: [0.0, 0.0, 0.0],
                                        tex_coords: [1.0, 0.0],
                                    },
                                    Vertex {
                                        position: [1.0 * x, 1.0 * x, 0.0],
                                        color: [0.0, 0.0, 0.0],
                                        tex_coords: [1.0, 1.0],
                                    },
                                    Vertex {
                                        position: [-1.0 * x, 1.0 * x, 0.0],
                                        color: [0.0, 0.0, 0.0],
                                        tex_coords: [0.0, 1.0],
                                    },
                                ],
                                vec![0, 1, 2, 0, 2, 3],
                            ));
                            wgpu_instance.update();
                            match wgpu_instance.render() {
                                Ok(()) => {}
                                Err(SurfaceError::Lost) => {
                                    wgpu_instance.resize(&window, wgpu_instance.size)
                                }
                                Err(SurfaceError::OutOfMemory) => elwt.exit(),
                                Err(e) => eprintln!("{}", e),
                            }

                            if !window.is_minimized().unwrap() && window.is_visible().unwrap() {
                                window.request_redraw();
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
