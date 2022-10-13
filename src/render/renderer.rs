use glow::*;
use sdl2::video::GLContext;

use super::shader::ShaderProgram;

pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;

pub struct Renderer {
    gl: Context,
    _gl_context: GLContext,
    quad_indices: Vec<u32>,
    shader_program: ShaderProgram,
}

impl Renderer {
    pub fn new(gl: Context, gl_context: GLContext) -> Self {
        unsafe {
            // Enable MSAA anti-aliasing
            // gl.enable(MULTISAMPLE);
            gl.enable(DEPTH_TEST);
            gl.enable(CULL_FACE);
            gl.polygon_mode(FRONT_AND_BACK, LINE);

            gl.viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            gl.clear_color(0.5, 0.9, 1.0, 1.0);
            gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        }

        let quad_indices = create_quad_indices(24);
        let shader_program = ShaderProgram::new(
            &gl,
            include_str!("default.vert"),
            include_str!("default.frag"),
        );

        Self {
            gl,
            _gl_context: gl_context,
            quad_indices,
            shader_program,
        }
    }

    /// # Safety
    pub unsafe fn gl(&self) -> &Context {
        &self.gl
    }

    pub fn clear(&self) {
        unsafe {
            self.gl.clear_color(0.5, 0.9, 1.0, 1.0);
            self.gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        }
    }

    pub fn quad_indices(&self) -> &Vec<u32> {
        &self.quad_indices
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.shader_program.native_program());
        }
    }
}

fn create_quad_indices(vert_len: usize) -> Vec<u32> {
    let indices = [0, 1, 2, 2, 3, 0]
        .iter()
        .copied()
        .cycle()
        .take((vert_len / 4) * 6)
        .enumerate()
        .map(|(i, v)| ((i / 6) * 4 + v) as u32)
        .collect();

    indices
}
