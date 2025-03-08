use std::mem;
use bytemuck::{Pod, Zeroable};

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

return (vertex_positions.to_vec(), colors.to_vec(), uvs.to_vec(), normalized_coords.to_vec());

}

fn vertex(p:[i8;3], c:[i8; 3]) -> Vertex {
Vertex {
position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
color: [c[0] as f32, c[1] as f32, c[2] as f32, 1.0],
}
}
pub fn create_vertices() -> Vec<Vertex> {
let (pos, col, _uv, _normal) = cube_data();
let mut data:Vec<Vertex> = Vec::with_capacity(pos.len());
for i in 0..pos.len() {
data.push(vertex(pos[i], col[i]));
}
data.to_vec()
}