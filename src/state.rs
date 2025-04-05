use cgmath::{Matrix4, Point3, Vector3};
use std::iter;

use cgmath::prelude::*;
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BlendComponent, BufferUsages, ColorTargetState, FragmentState};
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};


use crate::{camera::{self, Camera}, wgpu_helpers};

const ANIMATION_SPEED:f32 = 1.0;
const IS_PERSPECTIVE: bool = true;

pub struct State<'a> {
    #[allow(dead_code)]
    instance: wgpu::Instance,
    #[allow(dead_code)]
    adapter: wgpu::Adapter,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
    use_color: bool,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: &'a Window,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    model_mat: Matrix4<f32>,
    view_mat: Matrix4<f32>,
    project_mat: Matrix4<f32>,
    //challenge_render_pipeline: wgpu::RenderPipeline,
    //line_primitive_render_pipeline: wgpu::RenderPipeline
}

impl<'a> State<'a> {
    pub async fn new(window: &'a winit::window::Window) -> State<'a> {
        let size = window.inner_size();


        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = unsafe {instance.create_surface(window)}.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("ERROR IN CREATING ADAPTER");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    memory_hints: Default::default(),
                },
                None, // Trace path 
            ).await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);


        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);
        let clear_color = wgpu::Color::BLACK;
/* 
    let camera = Camera {
        // remember build_view_projection_matrix focuses the perspectivev
        // on the center using the eye as a "pov"
        // camera is placed one unit "up" and 2 "back"
        eye: (3.0, 1.5, 3.0).into(),
        //the origin of the coord-space but really the focus of where the "eye" is looking 
        target: (1.0, 0.0, 0.0).into(),
        up: cgmath::Vector3::unit_y(),
        aspect: config.width as f32 / config.height as f32,
        fovy: 45.0,
        znear: 0.1,
        zfar: 100.0,
    };

    let mut camera_uniform = camera::CameraUniform::new();
    camera_uniform.update_view_proj(&camera);
////////////////AHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH
    let camera_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor{
           label: Some("First camera buffer"),
           contents: bytemuck::cast_slice(&[camera_uniform]),
           usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        }
    );
*/
    let cam_eye : Point3<f32> = (3.0, 1.5, 3.0).into();
    let look_dir: Point3<f32> = (0.0,0.0,0.0).into();
    let up_dir: Vector3<f32> = cgmath::Vector3::unit_y();

    let model_mat = wgpu_helpers::create_transforms([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let (view_mat, project_mat, view_project_mat) = wgpu_helpers::create_view_projection(cam_eye, look_dir, up_dir, config.width as f32/ config.height as f32, IS_PERSPECTIVE);
    let mvp_mat = view_project_mat * model_mat;

    let mvp_ref: &[f32; 16] = mvp_mat.as_ref();

    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("uniform buffer"),
        contents: bytemuck::cast_slice(mvp_ref),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });
    
    let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
        label: Some("uniform bind group layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
                },
            count: None,
        }]
    });

    let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
        label:  Some("uniform buffer layout"),
        layout: &uniform_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
           binding: 0,
           resource: uniform_buffer.as_entire_binding()
        }],
    });
/* 
    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
        label: Some("uniform buffer"),
        contents: bytemuck::cast_slice(mvp_ref),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
    });

*/
    let buffer_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { 
        label: Some("shader"), 
        source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shader.wgsl").into()),
     });

     let buffer_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("pipeline_layout"),
        bind_group_layouts: &[&uniform_bind_group_layout],
        push_constant_ranges: &[]
    });

    let buffer_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
        label: Some("buffer pipeline"),
        layout: Some(&buffer_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &buffer_shader,
             entry_point: Some("vs_main"),
               buffers: &[wgpu_helpers::Vertex::desc()],
            compilation_options: Default::default(),
            },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(FragmentState {
            module: &buffer_shader,
            entry_point: Some("fs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            targets: &[Some(ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState {
                        color: BlendComponent::REPLACE,
                        alpha: BlendComponent::REPLACE,
                    }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
        cache: None,
    });

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
        label: Some("First vertex buffer"),
        contents: bytemuck::cast_slice(&State::create_vertices()),
        usage: BufferUsages::VERTEX,
    });


    

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window : window,
            instance,
            adapter,
            clear_color,
            render_pipeline: buffer_render_pipeline,
            use_color: true,
            vertex_buffer,
            uniform_buffer,
            uniform_bind_group,
            model_mat,
            view_mat,
            project_mat,
 
        }
    }

    fn window(&self) -> &Window {
        &self.window
    }

    pub fn create_vertices() -> Vec<[f32; 3]> {
    vec![
        [-0.5, -0.5, 0.0], // Bottom-left
        [0.5, -0.5, 0.0],  // Bottom-right
        [0.0, 0.5, 0.0],   // Top-center
    ]
}

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.project_mat = wgpu_helpers::create_projection((new_size.width/new_size.height) as f32, IS_PERSPECTIVE);
            let mvp_mat = self.project_mat * self.view_mat * self.model_mat;
            let mvp_ref: &[f32; 16] = mvp_mat.as_ref();
            self.queue.write_buffer(&self.uniform_buffer,0,bytemuck::cast_slice(mvp_ref));
        }
    }

    #[allow(unused_variables)]
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        
        match event {
            WindowEvent::CursorMoved { position,.. } => {
                println!("SOMETHING HAOPPEND");
                self.clear_color = wgpu::Color::BLUE; /*wgpu::Color {
                 r: position.x as f64 / self.size.width as f64,
                    g: position.y as f64 / self.size.height as f64,
                    b: 1.0,
                    a: 1.0,
                };
                */
                true
            },
            WindowEvent::KeyboardInput { device_id, event: KeyEvent {
                 physical_key: PhysicalKey::Code(KeyCode::Space),
                  state,
                   .. },
                    .. } => {
                        self.use_color = *state == ElementState::Released;
                        true
                    }
            _ => false
        }
    }

    pub fn update(&mut self, dur: std::time::Duration) {
        let dur = ANIMATION_SPEED * dur.as_secs_f32();
        let transf_mat =    wgpu_helpers::create_transforms(
            [0.0, 0.0, 0.0],
             [dur.sin(), dur.cos(), 0.0],
              [1.0,1.0, 1.0]);
        let mvp_mat = self.project_mat * self.view_mat * transf_mat;
        let mvp_ref: &[f32; 16] = mvp_mat.as_ref();
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(mvp_ref));

    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth texture"),
            size: wgpu::Extent3d {
                width: self.config.width,
                height: self.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format:wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],

        });

        let depth_view = depth_texture.create_view( &wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {r: 0.05, g:0.062, b:0.08, a:1.0}),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw(0..36,0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

}