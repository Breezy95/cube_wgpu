struct uniform_buffer {
    mvpMatrix: mat4x4<f32>,
    //model view perspective
};
@binding(0) @group(0) var<uniform> uniforms: uniform_buffer;
//above line is passing the model view matrix to the shader

struct vertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) vector_color: vec4<f32>,
}

@vertex
fn vs_main(@location(0) pos: vec4<f32>, @location(1) color: vec4<f32>) -> vertexOutput {
    var output: vertexOutput;
    output.position = uniforms.mvpMatrix * pos;
    output.vector_color = color;
    return output;
}

@fragment
fn fs_main(@location(0) vector_color: vec4<f32>) -> @location(0) vec4<f32> {
    return vector_color;
}