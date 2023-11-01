use winit::event::WindowEvent;

use super::Widget;

pub struct ColoredBlock {
    pos: (u32, u32),
    size: (u32, u32),
}

impl ColoredBlock {
    pub fn new(size: (u32, u32), pos: (u32, u32)) -> ColoredBlock {
        ColoredBlock { pos, size }
    }
}

impl Widget for ColoredBlock {
    fn set_size(&mut self, size: (u32, u32)) {
        self.size = size;
    }

    fn get_size(&mut self) -> (u32, u32) {
        self.size
    }

    fn set_pos(&mut self, pos: (u32, u32)) {
        self.pos = pos;
    }

    fn get_pos(&mut self) -> (u32, u32) {
        self.pos
    }

    fn render(&mut self, render_pass: &wgpu::RenderPass) {
        // TODO: 颜色块渲染
    }

    fn update(&mut self, event: &WindowEvent) {
        // 不用更新（也许？
    }
}
