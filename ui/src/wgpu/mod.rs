pub(crate) mod cache;
pub mod texture;

use std::borrow::Cow;
use std::collections::HashMap;

use bytemuck::{Pod, Zeroable};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    Adapter, Backends, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendState, BufferUsages,
    ColorTargetState, ColorWrites, CompositeAlphaMode, Device, DeviceDescriptor, Face, Features,
    FragmentState, Instance, InstanceDescriptor, Limits, LoadOp, MultisampleState, Operations,
    PipelineLayoutDescriptor, PowerPreference, PresentMode, PrimitiveState, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    RequestAdapterOptions, SamplerBindingType, ShaderModuleDescriptor, ShaderStages, Surface,
    SurfaceConfiguration, SurfaceError, TextureSampleType, TextureUsages, TextureViewDimension,
    VertexState,
};
use wgpu::{Buffer, BufferAddress, VertexAttribute, VertexBufferLayout};

use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::renderer::RenderSystem;
use crate::widget::colored_block::ColoredBlock;
use crate::widget::Widget;
use crate::{assets::Assets, Exception};

use self::texture::Texture;

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
    pub scale_factor: f64,
    pub size: PhysicalSize<u32>,
    pub render_pipelines: HashMap<String, RenderPipeline>,
    pub texture_bind_group_layout: BindGroupLayout,
    pub surface_size_buffer: Buffer,
    pub surface_size_bind_group: BindGroup,
    pub render_objects: Vec<RenderObject>,
    pub base_widget: Box<dyn Widget>,
    pub textures: Vec<Texture>
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
            alpha_mode: alpha_channel,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

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

        // 屏幕大小Uniform
        let surface_size_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("surface_size_uniform_buffer"),
            contents: bytemuck::bytes_of(&[size.width as f32, size.height as f32, 1.0]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let surface_size_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("surface_size_uniform_layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let surface_size_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("surface_size_bind_group"),
            layout: &surface_size_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: surface_size_buffer.as_entire_binding(),
            }],
        });

        let mut render_pipelines = HashMap::new();

        // Position Color
        fn position_color(
            device: &Device,
            config: &SurfaceConfiguration,
            surface_size_bind_group: &BindGroupLayout,
        ) -> Result<RenderPipeline, Exception> {
            let shader_path: String = "position_color".into();
            let binding =
                Assets::get((format!("shaders/{}.wgsl", shader_path.clone())).as_str()).unwrap();
            let src = std::str::from_utf8(&binding.data).unwrap();
            let shader = device.create_shader_module(ShaderModuleDescriptor {
                label: Some(shader_path.as_str()),
                source: wgpu::ShaderSource::Wgsl(src.into()),
            });
            let pipeline_layout: wgpu::PipelineLayout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some((shader_path.clone() + "_pipeline_layout").as_str()),
                bind_group_layouts: &[surface_size_bind_group],
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
                    // 剔除部分
                    front_face: wgpu::FrontFace::Cw,
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
            position_color(&device, &config, &surface_size_bind_group_layout).unwrap(),
        );

        // Position Texture
        fn position_texture(
            device: &Device,
            config: &SurfaceConfiguration,
            surface_size_bind_group_layout: &BindGroupLayout,
            texture_bind_group_layout: &BindGroupLayout,
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
                bind_group_layouts: &[surface_size_bind_group_layout, texture_bind_group_layout],
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
            position_texture(&device, &config, &surface_size_bind_group_layout, &texture_bind_group_layout).unwrap(),
        );

        let render_objects = Vec::new();

        WGPUInstance {
            instance,
            surface,
            device,
            queue,
            config,
            scale_factor: window.scale_factor(),
            size,
            render_pipelines,
            texture_bind_group_layout,
            surface_size_buffer,
            surface_size_bind_group,
            render_objects,
            base_widget: Box::new(ColoredBlock::new((580, 380), (10, 10))),
            textures: Vec::new(),
        }
    }

    pub fn resize(&mut self, _window: &Window, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = self.size.width;
            self.config.height = self.size.height;
            self.config.present_mode = PresentMode::AutoVsync;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        // TODO: 输入这部分还没做 =w=
        false
    }

    pub fn update(&mut self, event: &WindowEvent) {
        // TODO: 啥也没有 owo
        self.base_widget.update(event);
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        // TODO: 测试
        // let mut font_system = FontSystem::new();
        // let mut cache = SwashCache::new();
        // let mut atlas = TextAtlas::new(&self.device, &self.queue, self.config.format);
        // let mut text_renderer = TextRenderer::new(&mut atlas, &self.device, MultisampleState::default(), None);
        // let mut buffer = glyphon::Buffer::new(&mut font_system, Metrics::new(30.0, 42.0));

        // buffer.set_size(&mut font_system, self.size.width as f32, self.size.height as f32);
        // buffer.set_text(&mut font_system, "Hello Akiraka!", Attrs::new().family(glyphon::Family::Serif), glyphon::Shaping::Advanced);
        // buffer.shape_until_scroll(&mut font_system);

        // let mut texture = crate::wgpu::texture::Texture::from_bytes(
        //     &Assets::get("textures/happy-tree.png").unwrap().data,
        //     &self.texture_bind_group_layout,
        //     &self,
        // );
        let data: Cow<'_, [u8]> = Assets::get("textures/happy-tree.png").unwrap().data.clone();
        // 啥也不是 o.0
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

            let mut render_system = RenderSystem {
                size: (self.size.width, self.size.height),
                scale_factor: self.scale_factor,
                render_objects: &mut self.render_objects,
            };
            self.base_widget.render(&mut render_system);

            // // 清除上一循环的缓冲
            // unsafe {
            //     for buffer in &VERTEX_BUFFERS {
            //         buffer.destroy();
            //     }
            //     VERTEX_BUFFERS.clear();

            //     for buffer in &INDEX_BUFFERS {
            //         buffer.destroy();
            //     }
            //     INDEX_BUFFERS.clear();
            // }

            // println!("{:?}", self.render_objects.len());

            Texture::from_bytes(&data, &self.texture_bind_group_layout, self).bind(&mut render_pass);
            // 然后渲染这一循环
            for (i, obj) in self.render_objects.iter().enumerate() {
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
                                contents: bytemuck::cast_slice(cache::get_index(
                                    obj.index_location,
                                )),
                                usage: wgpu::BufferUsages::INDEX,
                            });
                    // 所有权转移
                    VERTEX_BUFFERS.push(vertex_buffer);
                    INDEX_BUFFERS.push(index_buffer);

                    // render_pass.set_pipeline(&self.render_pipelines.get("position_color").unwrap());
                    render_pass.set_pipeline(&self.render_pipelines.get("position_texture").unwrap());
                    // let _ = &texture.bind(&mut render_pass);

                    // Uniform来了！
                    self.queue.write_buffer(
                        &self.surface_size_buffer,
                        0,
                        bytemuck::bytes_of(&[self.size.width as f32, self.size.height as f32, 1.0]),
                    );
                    render_pass.set_bind_group(0, &self.surface_size_bind_group, &[]);

                    render_pass.set_vertex_buffer(0, VERTEX_BUFFERS.last().unwrap().slice(..));
                    render_pass.set_index_buffer(
                        INDEX_BUFFERS.last().unwrap().slice(..),
                        wgpu::IndexFormat::Uint16,
                    );

                    render_pass.draw_indexed(
                        0..cache::get_index(obj.index_location).len() as u32,
                        0,
                        0..1,
                    ); // 2.
                }
            }

            // TODO: 测试
            // text_renderer.prepare(
            //     &self.device,
            //     &self.queue,
            //     &mut font_system,
            //     &mut atlas,
            //     Resolution {
            //         width: self.size.width,
            //         height: self.config.height,
            //     },
            //     [TextArea {
            //         buffer: &buffer,
            //         left: 10.0,
            //         top: 10.0,
            //         scale: 1.0,
            //         bounds: TextBounds {
            //             left: 0,
            //             top: 0,
            //             right: 600,
            //             bottom: 160,
            //         },
            //         default_color: glyphon::Color::rgb(255, 255, 255)
            //     }],
            //     &mut cache).unwrap();
            // text_renderer.render(&atlas, &mut render_pass).unwrap();
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();
        self.render_objects.clear();
        cache::clear_vertex();
        cache::clear_index();
        // let _ = &texture.destroy();
        Ok(())
    }
}
