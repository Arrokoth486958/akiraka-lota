pub mod colored_block;

use winit::event::WindowEvent;

use crate::{wgpu::WGPUInstance, renderer::RenderSystem};

pub trait Widget {
    fn set_size(&mut self, size: (u32, u32));

    fn get_size(&mut self) -> (u32, u32);

    fn set_pos(&mut self, pos: (u32, u32));

    fn get_pos(&mut self) -> (u32, u32);

    fn render(&mut self, render_system: &mut RenderSystem);

    fn update(&mut self, event: &WindowEvent);
}
