use glam::{Mat4, Vec3};
use glow::*;

pub struct ShaderProgram {
    program: Program,
}

impl ShaderProgram {
    pub fn new(gl: &Context, vertex_shader_source: &str, fragment_shader_source: &str) -> Self {
        unsafe {
            let program = gl.create_program().expect("Couldn't create program.");

            let shader_sources = [
                (glow::VERTEX_SHADER, vertex_shader_source),
                (glow::FRAGMENT_SHADER, fragment_shader_source),
            ];

            let mut shaders = Vec::with_capacity(shader_sources.len());

            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Couldn't create shader.");

                gl.shader_source(shader, *shader_source);
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    panic!(
                        "{}\n{}",
                        "Couldn't compile shader.",
                        gl.get_shader_info_log(shader)
                    );
                }

                gl.attach_shader(program, shader);

                shaders.push(shader);
            }

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!(
                    "{}\n{}",
                    "Couldn't link program.",
                    gl.get_program_info_log(program)
                );
            }

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            Self { program }
        }
    }

    pub fn native_program(&self) -> NativeProgram {
        self.program
    }

    pub fn set_used(&self, gl: &Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }

    pub fn set_mat4(&self, gl: &Context, name: &str, value: Mat4) {
        unsafe {
            if let Some(location) = gl.get_uniform_location(self.program, name) {
                let data_array = value.to_cols_array();
                let data = core::slice::from_raw_parts(data_array.as_ptr(), data_array.len());

                gl.uniform_matrix_4_f32_slice(Some(&location), false, data);
            } else {
                panic!("Couldn't find uniform: {}", name);
            }
        }
    }

    pub fn set_vec3(&self, gl: &Context, name: &str, value: Vec3) {
        unsafe {
            if let Some(location) = gl.get_uniform_location(self.program, name) {
                let data_array = value.to_array();
                let data = core::slice::from_raw_parts(data_array.as_ptr(), data_array.len());

                gl.uniform_3_f32_slice(Some(&location), data);
            } else {
                panic!("Couldn't find uniform: {}", name);
            }
        }
    }

    pub fn set_int(&self, gl: &Context, name: &str, value: i32) {
        unsafe {
            if let Some(location) = gl.get_uniform_location(self.program, name) {
                gl.uniform_1_i32(Some(&location), value);
            } else {
                panic!("Couldn't find uniform: {}", name);
            }
        }
    }
}
