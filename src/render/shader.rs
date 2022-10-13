use glow::*;

pub struct ShaderProgram {
    program: NativeProgram,
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
}
