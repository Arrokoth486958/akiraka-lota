pub mod colored_block;

use wgpu::RenderPass;
use winit::event::WindowEvent;

pub trait Widget {
    fn set_size(&mut self, size: (u32, u32));

    fn get_size(&mut self) -> (u32, u32);

    fn set_pos(&mut self, pos: (u32, u32));

    fn get_pos(&mut self) -> (u32, u32);

    fn render(&mut self, render_pass: &RenderPass);

    fn update(&mut self, event: &WindowEvent);
}
