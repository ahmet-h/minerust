use glam::{vec2, vec3, Mat4, Vec3};
use glow::*;
use sdl2::video::GLContext;

use super::{
    camera::Camera,
    mesh::{Mesh, Quad, Vertex},
    model::Model,
    shader::ShaderProgram,
};

pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;

pub struct Renderer {
    gl: Context,
    _gl_context: GLContext,
    quad_indices: Vec<u32>,
    geometry_shader: ShaderProgram,
    shader: ShaderProgram,
    g_buffer: GBuffer,
    screen_quad: Model,
}

impl Renderer {
    pub fn new(gl: Context, gl_context: GLContext) -> Self {
        let quad_indices = create_quad_indices(24);
        let geometry_shader = ShaderProgram::new(
            &gl,
            include_str!("geometry.vert"),
            include_str!("geometry.frag"),
        );
        let shader = ShaderProgram::new(
            &gl,
            include_str!("lighting.vert"),
            include_str!("lighting.frag"),
        );

        unsafe {
            // Enable MSAA anti-aliasing
            // gl.enable(MULTISAMPLE);
            gl.disable(BLEND);
            gl.enable(CULL_FACE);
            // gl.polygon_mode(FRONT_AND_BACK, LINE);

            gl.viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
        }

        let g_buffer = unsafe { create_g_buffer(&gl) };

        shader.set_used(&gl);
        // shader.set_int(&gl, "g_position", 0);
        // shader.set_int(&gl, "g_normal", 1);
        shader.set_int(&gl, "g_albedo_spec", 2);

        let mut screen_mesh = Mesh::new();
        let temp_normal = Vec3::ZERO;
        screen_mesh.push_quad(Quad::new(
            Vertex::new(vec3(-1., -1., 0.), temp_normal, vec2(0., 0.)),
            Vertex::new(vec3(1., -1., 0.), temp_normal, vec2(1., 0.)),
            Vertex::new(vec3(1., 1., 0.), temp_normal, vec2(1., 1.)),
            Vertex::new(vec3(-1., 1., 0.), temp_normal, vec2(0., 1.)),
        ));
        let screen_quad = Model::new(&gl, &quad_indices, &screen_mesh);

        Self {
            gl,
            _gl_context: gl_context,
            quad_indices,
            geometry_shader,
            shader,
            g_buffer,
            screen_quad,
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

    pub fn create_model(&self, mesh: &Mesh) -> Model {
        Model::new(&self.gl, &self.quad_indices, mesh)
    }

    pub fn quad_indices(&self) -> &Vec<u32> {
        &self.quad_indices
    }

    pub fn prepare(&self, camera: &mut Camera) {
        unsafe {
            self.gl
                .bind_framebuffer(FRAMEBUFFER, Some(self.g_buffer.framebuffer));

            self.clear();

            camera.update_projection_view_matrix(WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32);

            self.geometry_shader.set_used(&self.gl);
            self.geometry_shader
                .set_mat4(&self.gl, "projection_view", camera.projection_view());

            self.gl.enable(DEPTH_TEST);
            // self.gl.polygon_mode(FRONT_AND_BACK, LINE);
            // self.gl.disable(CULL_FACE);
        }
    }

    pub fn end(&self) {
        unsafe {
            self.gl.bind_framebuffer(FRAMEBUFFER, None);
            // self.gl.polygon_mode(FRONT_AND_BACK, FILL);
            // self.gl.enable(CULL_FACE);

            self.clear();

            self.shader.set_used(&self.gl);
            self.gl.active_texture(TEXTURE0);
            self.gl
                .bind_texture(TEXTURE_2D, Some(self.g_buffer.position));
            self.gl.active_texture(TEXTURE1);
            self.gl.bind_texture(TEXTURE_2D, Some(self.g_buffer.normal));
            self.gl.active_texture(TEXTURE2);
            self.gl
                .bind_texture(TEXTURE_2D, Some(self.g_buffer.albedo_spec));

            self.gl.disable(DEPTH_TEST);
            self.render_screen_quad();

            self.gl
                .bind_framebuffer(READ_FRAMEBUFFER, Some(self.g_buffer.framebuffer));
            self.gl.bind_framebuffer(DRAW_FRAMEBUFFER, None);

            self.gl.blit_framebuffer(
                0,
                0,
                WINDOW_WIDTH as i32,
                WINDOW_HEIGHT as i32,
                0,
                0,
                WINDOW_WIDTH as i32,
                WINDOW_HEIGHT as i32,
                DEPTH_BUFFER_BIT,
                NEAREST,
            );
            self.gl.bind_framebuffer(FRAMEBUFFER, None);
        }
    }

    fn render_screen_quad(&self) {
        unsafe {
            self.shader.set_used(&self.gl);

            self.gl.bind_vertex_array(Some(self.screen_quad.vao()));
            self.gl
                .draw_elements(TRIANGLES, self.screen_quad.len() as i32, UNSIGNED_INT, 0);
            self.gl.bind_vertex_array(None);
        }
    }

    pub fn render(&self, model: &Model) {
        unsafe {
            self.geometry_shader.set_used(&self.gl);
            self.geometry_shader.set_mat4(
                &self.gl,
                "model",
                Mat4::from_translation(vec3(0., 0., 0.)),
            );

            self.gl.bind_vertex_array(Some(model.vao()));
            self.gl
                .draw_elements(TRIANGLES, model.len() as i32, UNSIGNED_INT, 0);
            self.gl.bind_vertex_array(None);
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gl
                .delete_program(self.geometry_shader.native_program());
        }
    }
}

fn create_quad_indices(vert_len: usize) -> Vec<u32> {
    let indices = [0, 1, 2, 2, 3, 0]
        .iter()
        .cycle()
        .copied()
        .take((vert_len / 4) * 6)
        .enumerate()
        .map(|(i, v)| ((i / 6) * 4 + v) as u32)
        .collect();

    indices
}

unsafe fn create_color_buffer(gl: &Context, format: u32, type_: u32, attachment: u32) -> Texture {
    let texture = gl.create_texture().expect("Couldn't create texture.");
    gl.bind_texture(TEXTURE_2D, Some(texture));

    gl.tex_image_2d(
        TEXTURE_2D,
        0,
        format as i32,
        WINDOW_WIDTH as i32,
        WINDOW_HEIGHT as i32,
        0,
        RGBA,
        type_,
        None,
    );
    gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);
    gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);

    gl.framebuffer_texture_2d(FRAMEBUFFER, attachment, TEXTURE_2D, Some(texture), 0);

    gl.bind_texture(TEXTURE_2D, None);

    texture
}

struct GBuffer {
    framebuffer: Framebuffer,
    position: Texture,
    normal: Texture,
    albedo_spec: Texture,
}

unsafe fn create_g_buffer(gl: &Context) -> GBuffer {
    let g_buffer = gl
        .create_framebuffer()
        .expect("Couldn't create framebuffer.");

    gl.bind_framebuffer(FRAMEBUFFER, Some(g_buffer));

    let g_position = create_color_buffer(gl, RGBA16F, FLOAT, COLOR_ATTACHMENT0);
    let g_normal = create_color_buffer(gl, RGBA16F, FLOAT, COLOR_ATTACHMENT1);
    let g_albedo_spec = create_color_buffer(gl, RGBA, UNSIGNED_BYTE, COLOR_ATTACHMENT2);

    let attachments = [COLOR_ATTACHMENT0, COLOR_ATTACHMENT1, COLOR_ATTACHMENT2];

    gl.draw_buffers(&attachments);

    let depth_buffer = gl
        .create_renderbuffer()
        .expect("Couldn't create renderbuffer.");

    gl.bind_renderbuffer(RENDERBUFFER, Some(depth_buffer));

    gl.renderbuffer_storage(
        RENDERBUFFER,
        DEPTH_COMPONENT,
        WINDOW_WIDTH as i32,
        WINDOW_HEIGHT as i32,
    );

    gl.framebuffer_renderbuffer(
        FRAMEBUFFER,
        DEPTH_ATTACHMENT,
        RENDERBUFFER,
        Some(depth_buffer),
    );

    if gl.check_framebuffer_status(FRAMEBUFFER) != FRAMEBUFFER_COMPLETE {
        panic!("Framebuffer is not complete.");
    }

    gl.bind_framebuffer(FRAMEBUFFER, None);

    GBuffer {
        framebuffer: g_buffer,
        position: g_position,
        normal: g_normal,
        albedo_spec: g_albedo_spec,
    }
}
