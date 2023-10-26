use std::collections::HashMap;

use bytemuck::{Pod, Zeroable};
use wgpu::{Buffer, VertexBufferLayout, BufferAddress, VertexAttribute, VertexFormat};
use wgpu::util::DeviceExt;
use wgpu::{Adapter, Backends, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits, PowerPreference, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, SurfaceError, TextureUsages, RenderPipeline, ShaderModuleDescriptor, PipelineLayoutDescriptor, RenderPipelineDescriptor, VertexState, ColorTargetState, BlendState, ColorWrites, FragmentState, PrimitiveState, Face, MultisampleState, RenderPassDescriptor, RenderPassColorAttachment, Operations, LoadOp, CompositeAlphaMode, util::BufferInitDescriptor, BufferUsages};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::{Exception, assets::Assets};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub uv: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [VertexAttribute; 3] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x3];

    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0], uv: [0.0, 0.0, 0.0], },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0], uv: [0.0, 0.0, 0.0], },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0], uv: [0.0, 0.0, 0.0], },
];

// 好哎！
// https://jinleili.github.io/learn-wgpu-zh/beginner/tutorial2-surface/
pub struct WGPUInstance {
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
    pub render_pipelines: HashMap<String, RenderPipeline>,
    pub vertex_buffer: Buffer,
    pub num_vertices: u32,
}

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
                    power_preference: PowerPreference::HighPerformance,
                    // TODO: 设置强制使用CPU渲染
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

        // println!("{:?}", caps.alpha_modes);
        let alpha_channel = if caps.alpha_modes.contains(&CompositeAlphaMode::PostMultiplied) {
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

        let mut render_pipelines = HashMap::new();

        fn position_color(device: &Device, config: &SurfaceConfiguration) -> Result<RenderPipeline, Exception> {
            let shader_path: String = "position_color".into();
            let binding = Assets::get((format!("shaders/{}.wgsl", shader_path.clone())).as_str()).unwrap();
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
                    buffers: &[
                        Vertex::desc()
                    ],
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
        render_pipelines.insert("position_color".into(), position_color(&device, &config).unwrap());

        let vertex_buffer = device.create_buffer_init(
            &BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: BufferUsages::VERTEX,
            }
        );

        let num_vertices = VERTICES.len() as u32;

         WGPUInstance {
            surface,
            device,
            queue,
            config,
            size,
            render_pipelines,
            vertex_buffer,
            num_vertices,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size.clone();
            self.config.width = self.size.width;
            self.config.height = self.size.height;
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
        // TODO: 还是啥也没有 0.o
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
                color_attachments: &[
                    Some(RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 0.0
                                }
                            ),
                            store: true,
                        }
                    })
                ],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipelines.get("position_color").unwrap());
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_vertices, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();
        Ok(())
    }

    // 好像没啥用
    // pub fn load_pipeline(&mut self, shader_path: String) -> Result<RenderPipeline, Exception> {
    //     todo!()
    // }
}
