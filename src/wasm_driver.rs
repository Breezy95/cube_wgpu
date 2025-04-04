pub struct Driver<'a>{
     pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface<'a>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl<'a> Driver<'a>{
    pub async fn new(window: &'a winit::window::Window) -> Driver<'a>{
        let size = window.inner_size();
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window).unwrap();
        let adapter = instance.
        request_adapter( &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }).await.expect("Failed to find an appropriate adapter");
        let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("driver device"),
                required_features: adapter.features(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
            },
            None,
        )
        .await
        .expect("Failed to create device");

    Self{ 
        size, 
        surface, 
        adapter, 
        device, 
        queue
    }
    }
    }   
