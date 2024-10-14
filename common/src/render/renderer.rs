use glam::{ivec2, IVec2};
use glow::*;
use sdl2::video::{GLContext, Window};

pub struct Renderer {
    gl: Context,
    _gl_context: GLContext,
    dims: IVec2,
}

impl Renderer {
    pub fn new(gl: Context, _gl_context: GLContext, window: &Window) -> Self {
        let drawable_size = window.drawable_size();
        let dims = ivec2(drawable_size.0 as i32, drawable_size.1 as i32);

        Self {
            gl,
            _gl_context,
            dims,
        }
    }

    pub fn handle_resize(&mut self, width: i32, height: i32) {
        unsafe {
            self.dims.x = width;
            self.dims.y = height;

            self.gl.viewport(0, 0, width, height);
        }
    }
}
