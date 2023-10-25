use wgpu::{
    Adapter, Backends, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits,
    PowerPreference, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, SurfaceError,
    TextureUsages,
};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

pub struct WGPUInstance {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,
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
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        WGPUInstance {
            surface,
            device,
            queue,
            config,
            size,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size.clone();
            self.config.width = self.size.width;
            self.config.height = self.size.height;
            self.surface.configure(&self.device, &self.config);
            println!("{:?}", new_size);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        todo!()
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        todo!()
    }
}
