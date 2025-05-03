
use bytemuck::{Pod, Zeroable};
use cgmath::*;
use lazy_static::lazy_static;
use web_sys::console;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BlendComponent, Buffer, BufferUsages, ColorTargetState, FragmentState, PipelineLayout, RenderPipeline, RenderPipelineDescriptor, ShaderModule, SurfaceConfiguration, TextureFormat};
use std::{f32::consts::PI, sync::Mutex};
use std::{iter, mem, option};

use crate::wasm_driver::Driver;


const ANIMATION_SPEED: f32 = 0.001;


#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 4],
    color: [f32; 4]
}

impl Vertex {
    // each elem has a given shader loc, 0,1 = shader loc at 0 (vec2 format) and 1 (vec3 format)
    // correlates with whats in shader.wgsl
    const ATTRIBUTES : [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4];
    //render pipeline needs this to map gpu buffer to shader code
    // wgpu uses this layout to tell the remder pipeline how to read the data from the gpu buffer
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout{
            // reps the amount of bytes the gpu will pass after a vertex
            // a vertex of (x,y) will be 8 bytes, rgb will be 12
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            // tells the rende pipeline 
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }

    }
}



pub enum AnimationAttrs {
    Speed,
    Angle,
    Direction,

}


// used rn to "initialize" project view
pub fn create_view(camera_position: Point3<f32>, look_direction: Point3<f32>, up_direction: Vector3<f32>) -> Matrix4<f32> {
    Matrix4::look_at_rh(camera_position, look_direction, up_direction)
}

pub fn create_transforms(translation_matrix: [f32;3],rotation:[f32;3], scaling:[f32;3]) -> Matrix4<f32>{
    let trans_mat = Matrix4::from_translation(Vector3::new(translation_matrix[0], translation_matrix[1],translation_matrix[2]));
    let rot_x_mat = Matrix4::from_angle_x(Rad(rotation[0]));
    let rot_y_mat = Matrix4::from_angle_y(Rad(rotation[1]));
    let rot_z_mat = Matrix4::from_angle_z(Rad(rotation[2]));
    let scale_mat = Matrix4::from_nonuniform_scale(scaling[0], scaling[1], scaling[2]);

    return trans_mat * rot_x_mat * rot_y_mat * rot_z_mat * scale_mat;

}


#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub fn create_view_projection(camera_position: Point3<f32>, look_direction: Point3<f32>, up_direction: Vector3<f32>,
    aspect:f32, is_perspective:bool) -> (Matrix4<f32>, Matrix4<f32>, Matrix4<f32>) {
    
    // construct view matrix
    let view_mat = Matrix4::look_at_rh(camera_position, look_direction, up_direction);     

    // construct projection matrix
    let project_mat:Matrix4<f32>;
    if is_perspective {
        project_mat = OPENGL_TO_WGPU_MATRIX * perspective(Rad(2.0*PI/5.0), aspect, 0.1, 100.0);
    } else {
        project_mat = OPENGL_TO_WGPU_MATRIX * ortho(-4.0, 4.0, -3.0, 3.0, -1.0, 6.0);
    }
    
    // contruct view-projection matrix
    let view_project_mat = project_mat * view_mat;
   
    // return various matrices
    (view_mat, project_mat, view_project_mat)
} 

pub fn create_projection(aspect:f32, is_perspective:bool) -> Matrix4<f32> {
    let project_mat: Matrix4<f32>;
    if is_perspective {
        project_mat = OPENGL_TO_WGPU_MATRIX * perspective(Rad(2.0*PI/5.0), aspect, 0.1, 100.0);
    } else {
        project_mat = OPENGL_TO_WGPU_MATRIX * ortho(-4.0, 4.0, -3.0, 3.0, -1.0, 6.0);
    }
    project_mat

}

pub struct Cube {
    pub(crate) config: SurfaceConfiguration,
    pub model_mat: Matrix4<f32>,
    pub view_mat: Matrix4<f32>,
    pub project_mat: Matrix4<f32>,
    pub uniform_bg: BindGroup,
    pub uniform_buffer: Buffer,
    pub uniform_bgl: BindGroupLayout,
    pub vertex_buffer: Buffer,
    pub(crate) buffer_render_pipeline: RenderPipeline,

}
impl Cube {
    pub fn new(
        driver: &Driver,
        config: &SurfaceConfiguration,
        camera_position: Point3<f32>,
        look_direction: Point3<f32>,
        up_direction: Vector3<f32>,
        aspect: f32,
        is_perspective: bool,
    ) -> Self {
        // create perspective and view mats
        let (view_mat, project_mat, view_project_mat) = create_view_projection(
            camera_position,
            look_direction,
            up_direction,
            aspect,
            is_perspective,
        );
        let model_mat = create_transforms([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        let mvp_matrix = project_mat * model_mat * view_mat;
        let mvp_ref: &[f32; 16] = mvp_matrix.as_ref();

        // create uniform buffer
        let uniform_buffer = Cube::create_buffer(driver, mvp_ref);

        // create bind group layout
        let uniform_bgl = Cube::create_bgl(
            driver,
            &[BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            Some("uniform_bgl"),
        );

        // create bind group
        let uniform_bg = Cube::create_bg(
            driver,
            &uniform_bgl,
            &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            Some("uniform_bg"),
        );

        // create shader module
        let buffer_shader = Cube::create_shader(driver, Some("buffer_shader"));

        // create render pipeline layout
        let buffer_render_pipeline_layout =
            Cube::create_buffer_render_pipeline_layout(driver, &[&uniform_bgl]);

        // create render pipeline
        let buffer_render_pipeline = Cube::create_buffer_render_pipeline(
            driver,
            Some(&buffer_render_pipeline_layout),
            &buffer_shader,
            config.format,
        );

        // create vertex buffer
        let vertex_buffer = Cube::create_vertex_buffer(driver);

        Cube {
            config: config.clone(),
            model_mat,
            view_mat,
            project_mat,
            uniform_bg,
            uniform_buffer,
            uniform_bgl,
            vertex_buffer,
            buffer_render_pipeline,
        }
    }
}


impl Cube{
    

    //create uniform buffer for cube
    pub fn create_buffer(driver: &Driver, mvp_ref: &[f32;16]) -> Buffer{
        driver.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("uniform buffer"),
        contents: bytemuck::cast_slice(mvp_ref),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    })
}
    //for cube
    pub fn create_buffer_render_pipeline_layout(driver: &Driver, bgls: &[&BindGroupLayout]) -> PipelineLayout{
        driver.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
            label: Some("Cube pipeline layout"),
            bind_group_layouts: bgls,
            push_constant_ranges: &[],
        })
    }
    // for cube
   pub fn create_buffer_render_pipeline(driver: &Driver, layout: Option<&PipelineLayout>, buffer_shader: &ShaderModule, texture_format: TextureFormat) -> RenderPipeline {
        driver.device.create_render_pipeline(&RenderPipelineDescriptor{
            label: Some("Cube render pipeline"),
            layout: layout,
            vertex: wgpu::VertexState {
                module: buffer_shader,
                 entry_point: Some("vs_main"),
                   buffers: &[Vertex::desc()],
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
                format: texture_format,
                blend: Some(wgpu::BlendState {
                        color: BlendComponent::REPLACE,
                        alpha: BlendComponent::REPLACE,
                    }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
        cache: None,
        
            
        })
    }
    
    pub fn update_cube_render(&mut self, driver: &Driver ,mut dur: f32){

        dur *= ANIMATION_SPEED;
        
        let transf_matrix = create_transforms(
            //     left/right     up/down,  in/out/forward
            [0.0 , 0.0, 0.0],
            // cube pitch  yaw,  roll
             [dur, 0.0, 0.0],
              [1.0, 1.0, 1.0]);

              
               let camera_position = Point3::new(
        3.0 * dur.cos(), // X position (orbiting)
        2.0,                        // Y position (fixed height)
        3.0 * dur.sin(), // Z position (orbiting)

        );

    // Update the view matrix to look at the cube's center
    self.view_mat = create_view(
        camera_position,            // Camera position
        Point3::new(0.0, 0.0, 0.0), // Look at the cube's center
        Vector3::unit_y(),          // Up direction
    );

    // Combine the matrices: projection -> view -> model
    let mvp_matrix = self.project_mat * self.view_mat * self.model_mat * transf_matrix;
        //let mvp_matrix = self.project_mat * transf_matrix * self.view_mat * self.model_mat;
        let mvp_ref: &[f32; 16] = mvp_matrix.as_ref();
        driver.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(mvp_ref));
        
    }
    // for cube vertices
   pub fn create_vertex_buffer(driver: &Driver) ->Buffer{
        driver.device.create_buffer_init(&BufferInitDescriptor{
            label: Some("Cube vertex Buffer"),
            contents: bytemuck::cast_slice(&Cube::create_cube_vertices()),
            usage: BufferUsages::VERTEX,
        })
    }

    pub fn create_bgl(driver: &Driver, bgls: &[BindGroupLayoutEntry], label: Option<&str>) -> BindGroupLayout{
        driver.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: label,
            entries: bgls
    })
    }

    pub fn create_bg(driver: &Driver, layout: &BindGroupLayout, entries: &[BindGroupEntry], label: Option<&str>) -> BindGroup{
        driver.device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: label,
            layout: layout,
            entries: entries,
        })
    }


    pub fn create_shader(driver: &Driver, label: Option<&str>) ->ShaderModule{
        driver.device.create_shader_module(wgpu::ShaderModuleDescriptor{
            label: label,
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shader.wgsl").into()),
        })
    }

    pub fn create_pll(){

    }

    pub fn create_rpl(){}



   pub fn cube_data() -> (Vec<[i8; 3]>, Vec<[i8; 3]>, Vec<[i8; 2]>, Vec<[i8; 3]>){
        // we have to build the face with two triangle primitives cant be easy ig
    // using 4 vertices and indices will not let us have a typical cube with unique face colors

    let vertex_positions = [
        // front (0, 0, 1)
        [-1, -1, 1], [1, -1, 1], [-1, 1, 1], [-1, 1, 1], [ 1, -1, 1], [ 1, 1, 1],
        // right (1, 0, 0)
        [ 1, -1, 1], [1, -1, -1], [ 1, 1, 1], [ 1, 1, 1], [ 1, -1, -1], [ 1, 1, -1],
        // back (0, 0, -1)
        [ 1, -1, -1], [-1, -1, -1], [1, 1, -1], [ 1, 1, -1], [-1, -1, -1], [-1, 1, -1],
        // left (-1, 0, 0)
        [-1, -1, -1], [-1, -1, 1], [-1, 1, -1], [-1, 1, -1], [-1, -1, 1], [-1, 1, 1],
        // top (0, 1, 0)
        [-1, 1, 1], [ 1, 1, 1], [-1, 1, -1], [-1, 1, -1], [ 1, 1, 1], [ 1, 1, -1],
        // bottom (0, -1, 0)
        [-1, -1, -1], [ 1, -1, -1], [-1, -1, 1], [-1, -1, 1], [ 1, -1, -1], [ 1, -1, 1],
    ];


    let colors = [
        // front - blue
        [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1],
        // right - red
        [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0],
        // back - yellow
        [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0],
        // left - aqua
        [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1],
        // top - green
        [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0],
        // bottom - fuchsia
        [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1],
    ];

    let uvs = [
        // front
[0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
// right
[0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
// back
[0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
// left
[0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
// top
[0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],
// bottom
[0, 0], [1, 0], [0, 1], [0, 1], [1, 0], [1, 1],

    ];

    let normalized_coords = [
// front
[0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1],
// right
[1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0],
// back
[0, 0, -1], [0, 0, -1], [0, 0, -1], [0, 0, -1], [0, 0, -1], [0, 0, -1],
// left
[-1, 0, 0], [-1, 0, 0], [-1, 0, 0], [-1, 0, 0], [-1, 0, 0], [-1, 0, 0],
// top
[0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0],
// bottom
[0, -1, 0], [0, -1, 0], [0, -1, 0], [0, -1, 0], [0, -1, 0], [0, -1, 0],
    ];

    (vertex_positions.to_vec(), colors.to_vec(), uvs.to_vec(), normalized_coords.to_vec())
    }

    fn create_vertex(p:[i8;3], c:[i8;3]) -> Vertex{
        Vertex {
            position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
            color: [c[0] as f32, c[1] as f32, c[2] as f32, 1.0],
            }
    }

    fn create_cube_vertices() -> Vec<Vertex>{
        let (pos, col, _uv, _normal) = Cube::cube_data();
        let mut data: Vec<Vertex> = Vec::with_capacity(pos.len());
        for i in 0..pos.len() {
            data.push(Cube::create_vertex(pos[i],col[i]));
        }
        data.to_vec()
    }

    pub fn render(&mut self, driver: &Driver<'_>) -> Result<(), wgpu::SurfaceError> {
        let output = driver.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        console::log_1(&format!("config.width: {}, config.height: {}", self.config.width, self.config.height).into());

        let depth_texture = driver.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth texture"),
            size: wgpu::Extent3d{
                width: self.config.width,
                height: self.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor{
            label: Some("depth view texture"),
            ..Default::default()
        });
        let mut encoder = driver.device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Cube encoder")
        });
       console::log_1(&"before render pass".into()); 
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

            console::log_1(&"After render pass".into());
            render_pass.set_pipeline(&self.buffer_render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.uniform_bg, &[]);
            render_pass.draw(0..36,0..1);
        } 
        driver.queue.submit(iter::once(encoder.finish()));
        output.present();
    
                console::log_1(&"leaving func".into());
        Ok(())
    }

}