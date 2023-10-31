use wgpu::{BindGroup, Extent3d, TextureFormat, TextureUsages, ImageCopyTexture, TextureAspect, Origin3d, ImageDataLayout, util::{BufferInitDescriptor, DeviceExt}, BufferUsages, CommandEncoderDescriptor, SamplerDescriptor, TextureViewDescriptor, BindGroupLayoutDescriptor, BindGroupLayoutEntry, ShaderStages, BindingType, TextureViewDimension, TextureSampleType, SamplerBindingType, BindGroupDescriptor, BindGroupEntry, BindingResource, RenderPass};

use super::WGPUInstance;

static mut BIND_GROUPS: Vec<BindGroup> = Vec::new();

pub struct Texture {
    // bind_group_layout: Layout,
    // bind_group_ptr: *mut u8,
    // bind_group: &'static BindGroup,
    bind_group_position: usize,
    size: Extent3d,
    texture: wgpu::Texture
}

impl Texture {
    pub fn from_bytes(bytes: &[u8], instance: &WGPUInstance) -> Texture {
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

        // 这位更是重量级
        // 手动分配内存真有你的
        // 这不也是没办法吗？
        // let bind_group_layout = Layout::new::<BindGroup>();
        // let bind_group_layout = Layout::for_value::<BindGroup>(&bind_group);
        // let bind_group_ptr = unsafe {
        //     let ptr = std::alloc::alloc(bind_group_layout);
        //     if ptr.is_null() {
        //         handle_alloc_error(bind_group_layout);
        //     }
        //     println!("{:?}", ptr);
        //     // println!("{:?}", *(ptr as *mut BindGroup));
        //     *(ptr as *mut BindGroup) = bind_group;
        //     // &*(ptr as *mut BindGroup)
        //     ptr
        // };

        // let bind_group_ptr: *mut u8 = std::ptr::null_mut();

        // let bind_group_position = unsafe {
            // bind_groups.(bind_group);
            // bind_groups.len() - 1
        // };

        let bind_group_position = unsafe {
            let mut x: usize = 0;
            for i in 0..BIND_GROUPS.len() {
                if BIND_GROUPS.get(i).is_none() {
                    x = i
                }
            }
            BIND_GROUPS.insert(x, bind_group);
            x
        };

        Texture {
            // bind_group_ptr,
            // bind_group_layout,
            // bind_group,
            bind_group_position,
            size: texture_size,
            texture,
        }
    }

    pub fn bind(&mut self, render_pass: &mut RenderPass) {
        // render_pass.set_bind_group(0, &self.bind_group, &[]);
        // render_pass.set_bind_group(0, unsafe { &*(self.bind_group_ptr as *mut BindGroup) }, &[]);
        render_pass.set_bind_group(0, unsafe { BIND_GROUPS.get(self.bind_group_position.clone()).unwrap() }, &[]);
    }

    // 一定要调用！！！
    // 不然内存泄漏把你搞哭
    pub fn destroy(&mut self) {
        self.texture.destroy();
        unsafe {
            // std::alloc::dealloc(self.bind_group_ptr, self.bind_group_layout);
            BIND_GROUPS.remove(self.bind_group_position);
        }
    }
}