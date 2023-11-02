use winit::event::WindowEvent;

use crate::{wgpu::WGPUInstance, renderer::{self, RenderSystem}};

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

    fn render(&mut self, render_system: &mut RenderSystem) {
        // TODO: 颜色块渲染
        render_system.rect(self.pos, self.size)
    }

    fn update(&mut self, event: &WindowEvent) {
        // 不用更新（也许？
    }
}
