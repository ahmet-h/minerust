use glam::{vec3, Mat4};
use glow::*;

use crate::state::ecs::transform::Transform;

use super::{model::Model, shader::ShaderProgram};

const SHADOW_WIDTH: i32 = 1024;
const SHADOW_HEIGHT: i32 = 1024;

pub struct ShadowMap {
    framebuffer: Framebuffer,
    projection: Mat4,
    view: Mat4,
    projection_view: Mat4,
    shader: ShaderProgram,
}

impl ShadowMap {
    pub fn new(gl: &Context) -> Self {
        unsafe {
            let framebuffer = gl
                .create_framebuffer()
                .expect("Couldn't create framebuffer.");

            let depth_map = gl.create_texture().expect("Couldn't create texture.");
            gl.bind_texture(TEXTURE_2D, Some(depth_map));

            gl.tex_image_2d(
                TEXTURE_2D,
                0,
                DEPTH_COMPONENT as i32,
                SHADOW_WIDTH,
                SHADOW_HEIGHT,
                0,
                DEPTH_COMPONENT,
                FLOAT,
                None,
            );

            gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_WRAP_S, REPEAT as i32);
            gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_WRAP_T, REPEAT as i32);

            gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_MIN_FILTER, LINEAR as i32);
            gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_MAG_FILTER, LINEAR as i32);

            gl.bind_framebuffer(FRAMEBUFFER, Some(framebuffer));

            gl.framebuffer_texture(FRAMEBUFFER, DEPTH_ATTACHMENT, Some(depth_map), 0);
            gl.draw_buffer(NONE);
            gl.read_buffer(NONE);

            if gl.check_framebuffer_status(FRAMEBUFFER) != FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer is not complete.");
            }

            gl.bind_framebuffer(FRAMEBUFFER, None);

            let projection = Mat4::orthographic_rh_gl(-10., 10., -10., 10., 1., 10.);
            let light_dir = vec3(0.5, -1., -0.8).normalize();
            let view = Mat4::look_at_rh(
                vec3(0., 0., 0.) - light_dir * 10.,
                vec3(0., 0., 0.),
                vec3(0., 1., 0.),
            );
            let projection_view = projection * view;

            let shader =
                ShaderProgram::new(gl, include_str!("shadow.vert"), include_str!("shadow.frag"));

            Self {
                framebuffer,
                projection,
                view,
                projection_view,
                shader,
            }
        }
    }

    pub fn prepare(&self, gl: &Context) {
        unsafe {
            gl.viewport(0, 0, SHADOW_WIDTH, SHADOW_HEIGHT);
            gl.bind_framebuffer(FRAMEBUFFER, Some(self.framebuffer));
            gl.clear(DEPTH_BUFFER_BIT);

            self.shader.set_used(gl);
            self.shader
                .set_mat4(gl, "projection_view", self.projection_view);
        }
    }

    pub fn render(&self, gl: &Context, model: &Model, transform: &Transform) {
        unsafe {
            self.shader.set_used(gl);
            self.shader.set_mat4(gl, "model", transform.matrix());

            gl.bind_vertex_array(Some(model.vao()));
            gl.draw_elements(TRIANGLES, model.len() as i32, UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }
    }

    pub fn end(&self, gl: &Context) {
        unsafe {
            gl.bind_framebuffer(FRAMEBUFFER, None);
        }
    }
}
