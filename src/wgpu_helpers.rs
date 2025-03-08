use std::f32::consts::PI;
use winit::window::Window;
use cgmath::*;


#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

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