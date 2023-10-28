pub(crate) mod cache;
pub mod texture;

use std::collections::HashMap;

use bytemuck::{Pod, Zeroable};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    Adapter, Backends, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BlendState,
    BufferUsages, ColorTargetState, ColorWrites, CommandEncoderDescriptor, CompositeAlphaMode,
    Device, DeviceDescriptor, Extent3d, Face, Features, FragmentState, ImageCopyTexture,
    ImageDataLayout, Instance, InstanceDescriptor, Limits, LoadOp, MultisampleState, Operations,
    Origin3d, PipelineLayoutDescriptor, PowerPreference, PresentMode, PrimitiveState, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    RequestAdapterOptions, SamplerBindingType, SamplerDescriptor, ShaderModuleDescriptor,
    ShaderStages, Surface, SurfaceConfiguration, SurfaceError, TextureAspect, TextureDescriptor,
    TextureFormat, TextureSampleType, TextureUsages, TextureViewDescriptor, TextureViewDimension,
    VertexState,
};
use wgpu::{Buffer, BufferAddress, VertexAttribute, VertexBufferLayout};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::{assets::Assets, Exception};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2];

    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0, 0.0],
        color: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        color: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        color: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
        color: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
];

const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

pub struct RenderObject {
    vertex_location: usize,
    index_location: usize,
}

impl RenderObject {
    pub fn new(vertex: Vec<Vertex>, index: Vec<u16>) -> RenderObject {
        RenderObject {
            vertex_location: cache::alloc_vertex(vertex),
            index_location: cache::alloc_index(index),
        }
    }
}

// 好哎！
// https://jinleili.github.io/learn-wgpu-zh/beginner/tutorial2-surface/
pub struct WGPUInstance {
    pub instance: Instance,
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
    pub render_pipelines: HashMap<String, RenderPipeline>,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub num_indices: u32,
    pub diffuse_bind_group: BindGroup,
    pub render_objects: Vec<RenderObject>,
}

static mut VERTEX_BUFFERS: Vec<Buffer> = Vec::new();
static mut INDEX_BUFFERS: Vec<Buffer> = Vec::new();

impl WGPUInstance {
    pub fn new(window: &Window) -> Self {
        println!("Scalefactor: {:?}", window.scale_factor());
        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(window) }.unwrap();

        async fn request_adapter(instance: &Instance, surface: &Surface) -> Adapter {
            instance
                .request_adapter(&RequestAdapterOptions {
                    compatible_surface: Some(surface),
                    power_preference: PowerPreference::default(),
                    // 强制使用软件渲染，但是有些设备上用不了
                    // 而且会占用大量内存
                    force_fallback_adapter: false,
                })
                .await
                .unwrap()
        }

        let adapter = pollster::block_on(request_adapter(&instance, &surface));
        println!("{:?}", adapter.get_info());
        for i in instance.enumerate_adapters(Backends::all()) {
            println!("{:?}", i.get_info());
        }

        async fn request_device(adapter: &Adapter) -> (Device, Queue) {
            adapter
                .request_device(
                    &DeviceDescriptor {
                        features: Features::default(),
                        label: None,
                        limits: Limits::downlevel_defaults(),
                    },
                    None,
                )
                .await
                .unwrap()
        }
        let (device, queue) = pollster::block_on(request_device(&adapter));

        let caps = surface.get_capabilities(&adapter);

        let alpha_channel = if caps
            .alpha_modes
            .contains(&CompositeAlphaMode::PostMultiplied)
        {
            CompositeAlphaMode::PostMultiplied
        } else {
            caps.alpha_modes[0]
        };
        println!("{:?}", alpha_channel);
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            // alpha_mode: caps.alpha_modes[0],
            alpha_mode: alpha_channel,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // TODO: 纹理部分
        let diffuse_bytes = Assets::get("textures/happy-tree.png").unwrap().data;
        let diffuse_image = image::load_from_memory(&diffuse_bytes).unwrap();
        let diffuse_rgba = diffuse_image.to_rgba8();

        use image::GenericImageView;
        let dimensions = diffuse_image.dimensions();

        let texture_size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let diffuse_texture = device.create_texture(&TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            label: Some("diffuse_texture"),
            view_formats: &[],
        });

        queue.write_texture(
            ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &diffuse_rgba,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Temp Buffer"),
            contents: &diffuse_rgba,
            usage: BufferUsages::COPY_SRC,
        });

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
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
                texture: &diffuse_texture,
                mip_level: 0,
                aspect: TextureAspect::All,
                origin: Origin3d::ZERO,
            },
            texture_size,
        );

        queue.submit(Some(encoder.finish()));

        let diffuse_texture_view = diffuse_texture.create_view(&TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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

        let diffuse_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&diffuse_texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&diffuse_sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });
        // 好家伙纹理的东西真不少

        let mut render_pipelines = HashMap::new();

        // Position Color
        fn position_color(
            device: &Device,
            config: &SurfaceConfiguration,
        ) -> Result<RenderPipeline, Exception> {
            let shader_path: String = "position_color".into();
            let binding =
                Assets::get((format!("shaders/{}.wgsl", shader_path.clone())).as_str()).unwrap();
            let src = std::str::from_utf8(&binding.data).unwrap();
            let shader = device.create_shader_module(ShaderModuleDescriptor {
                label: Some(shader_path.as_str()),
                source: wgpu::ShaderSource::Wgsl(src.into()),
            });
            let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some((shader_path.clone() + "_pipeline_layout").as_str()),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
            let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some((shader_path.clone() + "_pipeline").as_str()),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(ColorTargetState {
                        format: config.format,
                        blend: Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                        write_mask: ColorWrites::all(),
                    })],
                }),
                primitive: PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });
            Ok(pipeline)
        }
        render_pipelines.insert(
            "position_color".into(),
            position_color(&device, &config).unwrap(),
        );

        // Position Texture
        fn position_texture(
            device: &Device,
            config: &SurfaceConfiguration,
            texture_bind_group_layout: BindGroupLayout,
        ) -> Result<RenderPipeline, Exception> {
            let shader_path: String = "position_texture".into();
            let binding =
                Assets::get((format!("shaders/{}.wgsl", shader_path.clone())).as_str()).unwrap();
            let src = std::str::from_utf8(&binding.data).unwrap();
            let shader = device.create_shader_module(ShaderModuleDescriptor {
                label: Some(shader_path.as_str()),
                source: wgpu::ShaderSource::Wgsl(src.into()),
            });
            let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some((shader_path.clone() + "_pipeline_layout").as_str()),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });
            let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some((shader_path.clone() + "_pipeline").as_str()),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(ColorTargetState {
                        format: config.format,
                        blend: Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                        write_mask: ColorWrites::all(),
                    })],
                }),
                primitive: PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });
            Ok(pipeline)
        }
        render_pipelines.insert(
            "position_texture".into(),
            position_texture(&device, &config, texture_bind_group_layout).unwrap(),
        );

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        let render_objects = Vec::new();

        WGPUInstance {
            instance,
            surface,
            device,
            queue,
            config,
            size,
            render_pipelines,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            render_objects,
        }
    }

    pub fn resize(&mut self, _window: &Window, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = self.size.width;
            self.config.height = self.size.height;
            self.config.present_mode = PresentMode::AutoVsync;
            // self.surface.get_current_texture().unwrap().texture.destroy();
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        // TODO: 输入这部分还没做 =w=
        false
    }

    pub fn update(&mut self) {
        // TODO: 啥也没有 owo
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        // TODO: 啥也不是 o.0
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            // render_pass.set_pipeline(&self.render_pipelines.get("position_color").unwrap());

            // render_pass.set_pipeline(&self.render_pipelines.get("position_texture").unwrap());
            // render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            // render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            // render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // 1.
            // render_pass.draw_indexed(0..self.num_indices, 0, 0..1); // 2.

            // 清除上一循环的缓冲
            unsafe {
                for buffer in &VERTEX_BUFFERS {
                    buffer.destroy();
                }
                VERTEX_BUFFERS.clear();

                for buffer in &INDEX_BUFFERS {
                    buffer.destroy();
                }
                INDEX_BUFFERS.clear();
            }

            // 然后渲染这一循环
            // println!("{:?}", self.render_objects);
            for (i, obj) in self.render_objects.iter().enumerate() {
                // println!("{:?}", i);
                // println!("{:?}", obj.vertex);

                // 因为涉及到全局变量所以需要unsafe
                unsafe {
                    let vertex_buffer =
                        self.device
                            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: Some(format!("Vertex Buffer: {}", i).as_str()),
                                // contents: bytemuck::cast_slice(obj.vertex),
                                contents: bytemuck::cast_slice(cache::get_vertex(
                                    obj.vertex_location,
                                )),
                                usage: wgpu::BufferUsages::VERTEX,
                            });

                    let index_buffer =
                        self.device
                            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: Some(format!("Index Buffer: {}", i).as_str()),
                                // contents: bytemuck::cast_slice(obj.indices),
                                contents: bytemuck::cast_slice(cache::get_index(
                                    obj.index_location,
                                )),
                                usage: wgpu::BufferUsages::INDEX,
                            });
                    // 所有权转移
                    VERTEX_BUFFERS.push(vertex_buffer);
                    INDEX_BUFFERS.push(index_buffer);

                    render_pass
                        .set_pipeline(&self.render_pipelines.get("position_texture").unwrap());
                    render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
                    render_pass.set_vertex_buffer(0, VERTEX_BUFFERS.last().unwrap().slice(..));
                    render_pass.set_index_buffer(
                        INDEX_BUFFERS.last().unwrap().slice(..),
                        wgpu::IndexFormat::Uint16,
                    ); // 1.
                    render_pass.draw_indexed(0..self.num_indices, 0, 0..1); // 2.
                }
            }
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();
        self.render_objects.clear();
        cache::clear_vertex();
        cache::clear_index();
        Ok(())
    }
}
