use glow::*;
use sdl2::video::GLContext;

use super::{model::Model, shader::ShaderProgram};

pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;

pub struct Renderer {
    gl: Context,
    _gl_context: GLContext,
    quad_indices: Vec<u32>,
    geometry_shader: ShaderProgram,
}

impl Renderer {
    pub fn new(gl: Context, gl_context: GLContext) -> Self {
        unsafe {
            // Enable MSAA anti-aliasing
            // gl.enable(MULTISAMPLE);
            gl.enable(DEPTH_TEST);
            gl.enable(CULL_FACE);
            // gl.polygon_mode(FRONT_AND_BACK, LINE);

            gl.viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            let g_buffer = create_g_buffer(&gl);
        }

        let quad_indices = create_quad_indices(24);
        let geometry_shader = ShaderProgram::new(
            &gl,
            include_str!("default.vert"),
            include_str!("default.frag"),
        );

        Self {
            gl,
            _gl_context: gl_context,
            quad_indices,
            geometry_shader,
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

    pub fn quad_indices(&self) -> &Vec<u32> {
        &self.quad_indices
    }

    pub fn render(&self, model: &Model) {
        unsafe {
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

    let g_position = create_color_buffer(&gl, RGBA16F, FLOAT, COLOR_ATTACHMENT0);
    let g_normal = create_color_buffer(&gl, RGBA16F, FLOAT, COLOR_ATTACHMENT1);
    let g_albedo_spec = create_color_buffer(&gl, RGBA, UNSIGNED_BYTE, COLOR_ATTACHMENT2);

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
