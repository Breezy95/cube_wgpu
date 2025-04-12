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
        let mut surface_target: SurfaceTarget; 
        let surface_result: Result<Surface, wgpu::CreateSurfaceError> = if canvas.is_some() {
            
            let canvas = canvas.unwrap();
            cfg_if::cfg_if! {
            if #[cfg(target_arch="wasm32")] {
                console::log_1(&format!("using canvas for surface target height {}", canvas.height()).into());
            surface_target= SurfaceTarget::Canvas(canvas);
            }
            else {
                surface_target = SurfaceTarget::from(window);
                 console::log_1(&"not using canvas for surface target".into());
                }
            }
            let variant = matches!(surface_target, SurfaceTarget::Window {..});
            console::log_1(&format!("variant value is Windows if true {}",variant.into_integer()).into());
            
            Ok(instance.create_surface(surface_target).expect(format!("error in surface target creation of  {}", match variant {
            true  => {"type Window"}
            _ => {"type Canvas"}
            }).as_str()))

            } else {
            
            instance.create_surface(window)
            
        };
        let surface = surface_result.unwrap();
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
