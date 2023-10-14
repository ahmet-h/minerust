use glow::*;
use sdl2::video::{GLContext, Window};

pub struct Renderer {
    gl: Context,
    _gl_context: GLContext,
}

impl Renderer {
    pub fn new(gl: Context, _gl_context: GLContext, window: &Window) -> Self {
        Self { gl, _gl_context }
    }
}
