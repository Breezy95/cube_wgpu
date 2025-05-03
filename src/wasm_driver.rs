use std::io::Error;

use bytemuck::Contiguous;
use web_sys::{console, HtmlCanvasElement};
use wgpu::{ rwh::HasWindowHandle, Surface, SurfaceTarget};
use winit::raw_window_handle;
pub struct Driver<'a>{
     pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface<'a>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl<'a> Driver<'a>{
    pub async fn new(window: &'a winit::window::Window, canvas: Option<HtmlCanvasElement>) -> Driver<'a>{
        let size = window.inner_size();
        let instance = wgpu::Instance::default();

 console::log_1(&"before new surface".into());
        #[cfg(target_arch="wasm32")]
        let surface = {
            if canvas.is_some(){
                let surface_target = wgpu::SurfaceTarget::Canvas(canvas.unwrap());
                instance.create_surface(surface_target).expect("Failed to create surface for wasm32 target")
            }
            else{
                let surface_target = SurfaceTarget::from(window);
                instance.create_surface(surface_target).expect("could not create surface with native target")
            }
};

        #[cfg(not(target_arch="wasm32"))]
        let surface = {
            let surface_target = SurfaceTarget::from(window);
            instance.create_surface(surface_target).expect("could not create surface with native target")
        };
         console::log_1(&"create surface".into());
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
        console::log_1(&"driver created".into());
    Self{ 
        size, 
        surface, 
        adapter, 
        device, 
        queue
    }
    }
    }   
