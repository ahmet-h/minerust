use glow::*;
use sdl2::video::GLContext;

pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;

pub struct Renderer {
    gl: Context,
    _gl_context: GLContext,
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

        Self {
            gl,
            _gl_context: gl_context,
        }
    }

    pub fn clear(&self) {
        unsafe {
            self.gl.clear_color(0.5, 0.9, 1.0, 1.0);
            self.gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        }
    }
}
