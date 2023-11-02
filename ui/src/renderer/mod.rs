use wgpu::RenderPass;

use crate::wgpu::{RenderObject, Vertex};

// 个很简单的渲染器
pub struct RenderSystem<'a> {
    pub(crate) size: (u32, u32),
    pub(crate) scale_factor: f64,
    pub(crate) render_objects: &'a mut Vec<RenderObject>,
}

impl<'a> RenderSystem<'a> {
    // TODO: [WIP] 转换X坐标
    fn transform_x(&mut self, value: u32) -> f32 {
        (value as f32) / (self.size.0 as f32) / (self.scale_factor as f32)
    }

    // TODO: [WIP] 转换Y坐标
    fn transform_y(&mut self, value: u32) -> f32 {
        (value as f32) / (self.size.1 as f32) / (self.scale_factor as f32)
    }

    pub fn rect(&mut self, pos: (u32, u32), size: (u32, u32)) {
        let x = self.transform_x(pos.0);
        let y = self.transform_y(pos.1);
        let w = self.transform_x(pos.0 + size.0);
        let h = self.transform_y(pos.1 + size.1);

        self.render_objects.push(RenderObject::new(
            vec![
                // 左上
                Vertex {
                    position: [
                        x,
                        y,
                        0.0,
                    ],
                    color: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                // 右上
                Vertex {
                    position: [
                        x + w,
                        y,
                        0.0,
                    ],
                    color: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                // 左下
                Vertex {
                    position: [
                        x,
                        y + h,
                        0.0,
                    ],
                    color: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                // 右下
                Vertex {
                    position: [
                        x + w,
                        y + h,
                        0.0,
                    ],
                    color: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
            ],
            vec![0, 1, 2, 0, 1, 2],
        ));
    }
}
