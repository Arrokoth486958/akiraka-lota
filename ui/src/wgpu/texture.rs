use std::borrow::BorrowMut;

use wgpu::{BindGroup, Extent3d, TextureFormat, TextureUsages, ImageCopyTexture, TextureAspect, Origin3d, ImageDataLayout, util::{BufferInitDescriptor, DeviceExt}, BufferUsages, CommandEncoderDescriptor, SamplerDescriptor, TextureViewDescriptor, BindGroupLayoutDescriptor, BindGroupLayoutEntry, ShaderStages, BindingType, TextureViewDimension, TextureSampleType, SamplerBindingType, BindGroupDescriptor, BindGroupEntry, BindingResource};

use crate::Exception;

use super::WGPUInstance;

pub struct Texture {
    bind_group: BindGroup,
    size: Extent3d,
    texture: wgpu::Texture
}

impl Texture {
    fn read_from_bytes(bytes: &[u8], instance: &WGPUInstance) -> Texture {
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

        let mut encoder = instance.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("texture_buffer_copy_encoder"),
        });

        encoder.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * dimensions.0),
                    rows_per_image: Some(dimensions.1),
                },
            },
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                aspect: TextureAspect::All,
                origin: Origin3d::ZERO,
            },
            texture_size,
        );

        instance.queue.submit(Some(encoder.finish()));

        let texture_view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = instance.device.create_sampler(&SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let texture_bind_group_layout =
            instance.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D2,
                            sample_type: TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let bind_group = instance.device.create_bind_group(&BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("bind_group"),
        });

        Texture {
            bind_group,
            size: texture_size,
            texture,
        }
    }

    pub fn destroy(&mut self) {
        self.texture.destroy();
    }
}