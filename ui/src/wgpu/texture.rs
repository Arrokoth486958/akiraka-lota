use std::vec;

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, BufferUsages,
    CommandEncoderDescriptor, Extent3d, ImageCopyTexture, ImageDataLayout, Origin3d, RenderPass,
    SamplerDescriptor, TextureAspect, TextureFormat, TextureUsages, TextureViewDescriptor, Buffer, Queue,
};

use super::WGPUInstance;

pub struct Texture {
    pub size: Extent3d,
    pub texture: wgpu::Texture,
    // 很生草的玩意
    pub data: Vec<u8>,
    pub buffer: Buffer,
}

impl Texture {
    pub fn from_bytes(
        bytes: &[u8],
        instance: &WGPUInstance,
    ) -> Texture {
        let image = image::load_from_memory(bytes).unwrap();
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

        let data = Vec::from(bytes);

        Texture {
            size: texture_size,
            texture,
            data,
            buffer,
        }
    }

    pub fn bind(&mut self, queue: &mut Queue) {
        // let boxed_slice = self.data.into_boxed_slice();
        // queue.write_buffer(&self.buffer, 0, self.data.collec)
    }

    // 一定要调用！！！
    // 不然内存泄漏把你搞哭
    pub fn destroy(&mut self) {
        self.texture.destroy();
    }
}
