use std::{sync::Arc, time::{self, Instant}};

use cgmath::{Point3, Vector3};
use wasm_bindgen::prelude::*;
use wasm_driver::Driver;
use web_sys::{console, HtmlCanvasElement, Node};
use wgpu::{rwh::HasWindowHandle, SurfaceConfiguration};
use wgpu_helpers::Cube;
use winit::{dpi::{LogicalSize, PhysicalSize}, event::{Event, StartCause, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::Window};
pub mod wasm_driver;
pub mod wgpu_helpers;



const IS_PERSPECTIVE: bool = true;

pub async fn run_wasm(event_loop: EventLoop<()>, window:Arc<Window>, location: (u32,u32,f32), ratio: f32, canvas: Option<HtmlCanvasElement>) {
    console::log_1(&"in run wasm".into());
    let driver = Driver::new(&window, canvas).await;
    let surface_capabilities = driver.surface.get_capabilities(&driver.adapter);
    let surface_format = surface_capabilities.formats.iter().copied().
    find(|f| f
        .is_srgb())
        .unwrap_or(surface_capabilities.formats[0]);
    let inner_size = window.inner_size();
    let mut driver_config = SurfaceConfiguration{
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: inner_size.width,
        height: inner_size.height,
        present_mode: surface_capabilities.present_modes[0],
        desired_maximum_frame_latency: 2,
        alpha_mode: surface_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    if driver_config.width == 0 || driver_config.height == 0 {
    console::log_1(&format!("inner window dimensions are {} and {}!", inner_size.width,inner_size.height).into());
    console::log_1(&format!("Driver config dimensions are {} and {}!", driver_config.width, driver_config.height).into());
    panic!();

}

    let camera_eye: Point3<f32> = (3.0, 1.5, 3.0).into();
    let look_dir: Point3<f32> = (0.0, 0.0, 0.0).into();
    let up_dir: Vector3<f32> = cgmath::Vector3::unit_y();
    let model_mat = wgpu_helpers::create_transforms([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let (view_mat, project_mat, view_project_mat) = wgpu_helpers::create_view_projection(camera_eye, look_dir, up_dir, driver_config.width as f32/ driver_config.height as f32, IS_PERSPECTIVE);
    let mvp_mat = view_project_mat * model_mat;
    let mvp_ref = mvp_mat.as_ref();
    let bgls = &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
            },
        count: None,
    }];
    let uniform_buffer = Cube::create_buffer(&driver, mvp_ref);
    let bge = &[wgpu::BindGroupEntry {
        binding: 0,
        resource: uniform_buffer.as_entire_binding()
     }];

     let uniform_bgl = Cube::create_bgl(&driver, bgls, None);
     let uniform_bg = Cube::create_bg(&driver, &uniform_bgl, bge, None);
     let buffer_pll = Cube::create_buffer_render_pipeline_layout(&driver, &[&uniform_bgl]);
     let buffer_shader = &Cube::create_shader(&driver, None);
     let driver_cfg_clone = driver_config.clone();
    
    let vertex_buffer =Cube::create_vertex_buffer(&driver);
    let buffer_render_pipeline = Cube::create_buffer_render_pipeline(&driver, Some(&buffer_pll), buffer_shader, driver_config.format);
     let mut cube_render = Cube {
        config: driver_config,
        model_mat: model_mat,
        uniform_bg,
        uniform_buffer: uniform_buffer,
        uniform_bgl,
        vertex_buffer,
        buffer_render_pipeline, 
        view_mat,
        project_mat,
    };
    
    console::log_1(&"cube render created".into());
    let win_clone = window.clone();
    console::log_1(&"window render".into());
    driver.surface.configure(&driver.device, &driver_cfg_clone);
    console::log_1(&"surface configure".into());
    let render_start_time = web_sys::window()
    .unwrap()
    .performance()
    .unwrap()
    .now() as f32 / 1000.0;
    //console::log_1(String::from(render_start_time).as_str());
    //cube_render.render(&driver);
    event_loop.run(move |event, control_flow| {
        control_flow.set_control_flow(ControlFlow::Poll);
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                cube_render.config.width = size.width;
                cube_render.config.height = size.height;
                driver.surface.configure(&driver.device, &cube_render.config);
                win_clone.request_redraw();
            }
            Event::AboutToWait => {win_clone.request_redraw()}

            Event::WindowEvent { window_id, event: WindowEvent::RedrawRequested, ..} => {
                let curr_time:f32 = web_sys::window().unwrap().performance().unwrap().now() as f32 / 1000.0;
                let time_diff = curr_time - render_start_time;
                console::log_1(&format!("curr_time: {}, time diff: {}, render_start_time: {}", curr_time, time_diff,time_diff).into());
                cube_render.update_cube_render(&driver, time_diff);
                cube_render.render(&driver);
            }
            Event::NewEvents(start_cause) => {

                console::log_1(&format!("event loop new events {0:#?}", start_cause).into());
            }
            _ => {console::log_1(&"random event".into());}
        }
    });

}

fn calculate_dimensions(res: i32, width: u32, height: u32) -> (u32, u32, f32){
    let aspect_ratio = height as f64/ width as f64;
    let x_pixels = (res as isize as f64/aspect_ratio).sqrt().floor() as u32;
    let pixel_size = width as f64/ x_pixels as f64;
    let y_pixels = (height as f64/pixel_size).floor() as u32;
    (x_pixels, y_pixels, pixel_size as f32)
}



#[wasm_bindgen]
pub fn run(pixel_ratio: f32, width: u32, height: u32,canvas: Option<HtmlCanvasElement>) {
    let res = 1000;
    let event_loop = EventLoop::new().unwrap();
    let dimensions = calculate_dimensions(res, width, height);
    
    if !(dimensions.0 == 0 || dimensions.1 == 0) {
        console::log_1(&format!("Calculated dimensions are res:{0}\twidth:{1}\theight:{2}",dimensions.0,dimensions.1, dimensions.2).into());
    }

    
    
    #[cfg(target_arch = "wasm32")] //, target_os = "unknown"))]
    use winit::platform::web::WindowExtWebSys;

    use wasm_bindgen_futures;
    use web_sys;
    console_log::init().expect("could not initialize logger");
    console::log_1(&"Hello from before run".into());
    let window_builder = {
    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowBuilderExtWebSys;
        if let Some(ref canvas_element) = canvas {
            winit::window::WindowBuilder::new().with_canvas(Some(canvas_element.clone()))
        } else {
            winit::window::WindowBuilder::new()
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        winit::window::WindowBuilder::new()
    }
};

    let window = Arc::new(window_builder
        .with_inner_size(PhysicalSize::new(450, 400))
        .build(&event_loop)
        .unwrap());

#[cfg(target_arch="wasm32")]
    if canvas.is_none() {
                let document = web_sys::window()
                    .and_then(|win| win.document())
                    .expect("could not get document");

                let body = document.body().expect("could not get body");

                // Create a div element for the canvas container
                let mut cube_cont = document.get_element_by_id("cube_container");
                if cube_cont.is_none() {
                    cube_cont = document.create_element("div").ok();
                    cube_cont.clone().unwrap().set_id("cube-container");
                }
                let cube_cont_div = cube_cont.unwrap();
                cube_cont_div
                    .set_attribute(
                        "style",
                        "position: relative; width: 100vw; height: 100vh; display: flex; justify-content: center; align-items: center; overflow: hidden;",
                    )
                    .unwrap();

                // Create the canvas element and append it to the container
                let elem = &web_sys::Element::from(window.canvas().unwrap());
                elem.set_id("cube_canvas");
                cube_cont_div.append_child(elem).unwrap();


                body.append_child(&cube_cont_div).unwrap();
               } 

                wasm_bindgen_futures::spawn_local(run_wasm(
                    event_loop,
                    window,
                    dimensions,
                    pixel_ratio * dimensions.2,
                    canvas,
                ));
            
        }
