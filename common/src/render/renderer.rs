use glam::{ivec2, vec4, IVec2, Vec4};
use glow::*;
use sdl2::video::{GLContext, Window};

use super::{camera::Camera, mesh::Mesh, model::Model, shader::ShaderProgram, vertex::Vertex};

pub struct Renderer {
    gl: Context,
    _gl_context: GLContext,
    dims: IVec2,
    sky_shader: ShaderProgram,
}

impl Renderer {
    pub fn new(gl: Context, _gl_context: GLContext, window: &Window) -> Self {
        let drawable_size = window.drawable_size();
        let dims = ivec2(drawable_size.0 as i32, drawable_size.1 as i32);

        let sky_shader = ShaderProgram::new(&gl, include_str!("sky.vs"), include_str!("sky.fs"));

        Self {
            gl,
            _gl_context,
            dims,
            sky_shader,
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

    pub fn prepare(&self, camera: &mut Camera) {
        unsafe {
            self.gl.viewport(0, 0, self.dims.x, self.dims.y);

            self.clear();

            camera.update_projection_view_matrix(self.dims.x as f32 / self.dims.y as f32);

            let c = camera.front();
            // println!(
            //     "{:?}",
            //     camera.inv_view_projection() * vec4(c.x, c.y, c.z, 1.0)
            // );

            self.sky_shader.set_used(&self.gl);
            self.sky_shader.set_mat4(
                &self.gl,
                "inv_view_projection",
                camera.inv_view_projection(),
            );

            self.sky_shader.set_used(&self.gl);
            self.sky_shader
                .set_mat4(&self.gl, "projection_view", camera.projection_view());

            self.gl.enable(DEPTH_TEST);
            // self.gl.polygon_mode(FRONT_AND_BACK, LINE);
            // self.gl.disable(CULL_FACE);
        }
    }

    pub fn render_sky<V: Vertex>(&self, model: &Model<V>) {
        unsafe {
            self.gl.depth_func(LEQUAL);
            self.sky_shader.set_used(&self.gl);

            self.gl.bind_vertex_array(Some(model.vao()));
            self.gl
                .draw_elements(TRIANGLES, model.len() as i32, UNSIGNED_INT, 0);
            self.gl.bind_vertex_array(None);
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

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.sky_shader.native_program());
        }
    }
}
