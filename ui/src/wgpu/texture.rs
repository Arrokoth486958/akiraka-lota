use std::{vec, borrow::Cow};

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, BufferUsages,
    CommandEncoderDescriptor, Extent3d, ImageCopyTexture, ImageDataLayout, Origin3d, RenderPass,
    SamplerDescriptor, TextureAspect, TextureFormat, TextureUsages, TextureViewDescriptor, Buffer, Queue,
};

use super::WGPUInstance;

pub struct Texture<'a> {
    pub size: Extent3d,
    pub texture: wgpu::Texture,
    pub data: Cow<'a, [u8]>,
    pub buffer: Buffer,
}

impl<'a> Texture<'a> {
    pub fn from_bytes(
        bytes: Cow<'a, [u8]>,
        instance: &WGPUInstance,
    ) -> Texture<'a> {
        let image = image::load_from_memory(&bytes).unwrap();
        let rgba = image.to_rgba8();

        use image::GenericImageView;
        let dimensions = image.dimensions();

        let texture_size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = instance.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            label: Some("texture"),
            view_formats: &[],
        });

        instance.queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &rgba,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        let buffer = instance.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Temp Buffer"),
            contents: &rgba,
            usage: BufferUsages::COPY_SRC,
        });

        Texture {
            size: texture_size,
            texture,
            data: bytes,
            buffer,
        }
    }

    pub fn bind(&mut self, queue: &mut Queue) {
        queue.write_buffer(&self.buffer, 0, &self.data);
    }

    // 一定要调用！！！
    // 不然内存泄漏把你搞哭
    pub fn destroy(&mut self) {
        self.texture.destroy();
        self.buffer.destroy();
    }
}
