use glam::{ivec2, IVec2};
use glow::*;
use sdl2::video::{GLContext, Window};

use super::{mesh::Mesh, model::Model, vertex::Vertex};

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

    /// # Safety
    pub unsafe fn gl(&self) -> &Context {
        &self.gl
    }

    pub fn clear(&self) {
        unsafe {
            self.gl.clear_color(0., 0., 0., 1.);
            self.gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        }
    }

    pub fn create_model<V: Vertex>(&self, mesh: &Mesh<V>) -> Model<V> {
        Model::new(&self.gl, mesh)
    }

    pub fn handle_resize(&mut self, width: i32, height: i32) {
        unsafe {
            self.dims.x = width;
            self.dims.y = height;

            self.gl.viewport(0, 0, width, height);
        }
    }
}
