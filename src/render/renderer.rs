use glam::{ivec2, vec2, vec3, IVec2, Mat3, Mat4, Vec3};
use glow::*;
use sdl2::video::{GLContext, Window};

use crate::state::ecs::transform::Transform;

use super::{
    camera::Camera,
    mesh::{Mesh, Quad, Vertex},
    model::Model,
    shader::ShaderProgram,
    texture::{CubeMap, GameTexture, Skybox},
};

pub struct Renderer {
    gl: Context,
    _gl_context: GLContext,
    quad_indices: Vec<u32>,
    geometry_shader: ShaderProgram,
    lighting_shader: ShaderProgram,
    skybox_shader: ShaderProgram,
    g_buffer: GBuffer,
    screen_quad: Model,
    dims: IVec2,
}

impl Renderer {
    pub fn new(gl: Context, gl_context: GLContext, window: &Window) -> Self {
        let quad_indices = create_quad_indices(24);
        let geometry_shader = ShaderProgram::new(
            &gl,
            include_str!("geometry.vert"),
            include_str!("geometry.frag"),
        );
        let lighting_shader = ShaderProgram::new(
            &gl,
            include_str!("lighting.vert"),
            include_str!("lighting.frag"),
        );
        let skybox_shader = ShaderProgram::new(
            &gl,
            include_str!("skybox.vert"),
            include_str!("skybox.frag"),
        );

        let drawable_size = window.drawable_size();
        let dims = ivec2(drawable_size.0 as i32, drawable_size.1 as i32);

        let g_buffer = unsafe { create_g_buffer(&gl, &dims) };

        let screen_quad = create_screen_quad(&gl, &quad_indices);

        unsafe {
            // Enable MSAA anti-aliasing
            // gl.enable(MULTISAMPLE);
            gl.disable(BLEND);
            gl.enable(CULL_FACE);
            // gl.polygon_mode(FRONT_AND_BACK, LINE);

            gl.viewport(0, 0, dims.x, dims.y);
        }

        lighting_shader.set_used(&gl);
        lighting_shader.set_int(&gl, "g_position", 0);
        lighting_shader.set_int(&gl, "g_normal", 1);
        lighting_shader.set_int(&gl, "g_albedo_spec", 2);

        Self {
            gl,
            _gl_context: gl_context,
            quad_indices,
            geometry_shader,
            lighting_shader,
            skybox_shader,
            g_buffer,
            screen_quad,
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

    pub fn create_model(&self, mesh: &Mesh) -> Model {
        Model::new(&self.gl, &self.quad_indices, mesh)
    }

    pub fn create_texture(&self, path: &str) -> GameTexture {
        GameTexture::new(&self.gl, path)
    }

    pub fn bind_texture(&self, texture: &GameTexture) {
        texture.bind(&self.gl, 0);
    }

    pub fn quad_indices(&self) -> &Vec<u32> {
        &self.quad_indices
    }

    pub fn prepare(&self, camera: &mut Camera) {
        unsafe {
            self.gl
                .bind_framebuffer(FRAMEBUFFER, Some(self.g_buffer.framebuffer));

            self.clear();

            camera.update_projection_view_matrix(self.dims.x as f32 / self.dims.y as f32);

            self.geometry_shader.set_used(&self.gl);
            self.geometry_shader
                .set_mat4(&self.gl, "projection_view", camera.projection_view());
            self.geometry_shader.set_int(&self.gl, "texture_diffuse", 0);

            self.gl.enable(DEPTH_TEST);
            // self.gl.polygon_mode(FRONT_AND_BACK, LINE);
            // self.gl.disable(CULL_FACE);
        }
    }

    pub fn end(&self, camera: &Camera) {
        unsafe {
            self.gl.bind_framebuffer(FRAMEBUFFER, None);
            // self.gl.polygon_mode(FRONT_AND_BACK, FILL);
            // self.gl.enable(CULL_FACE);

            self.clear();

            self.lighting_shader.set_used(&self.gl);
            self.gl.active_texture(TEXTURE0);
            self.gl
                .bind_texture(TEXTURE_2D, Some(self.g_buffer.position));
            self.gl.active_texture(TEXTURE1);
            self.gl.bind_texture(TEXTURE_2D, Some(self.g_buffer.normal));
            self.gl.active_texture(TEXTURE2);
            self.gl
                .bind_texture(TEXTURE_2D, Some(self.g_buffer.albedo_spec));

            self.gl.disable(DEPTH_TEST);

            self.lighting_shader
                .set_vec3(&self.gl, "view_pos", camera.pos());
            self.render_screen_quad();

            self.gl
                .bind_framebuffer(READ_FRAMEBUFFER, Some(self.g_buffer.framebuffer));
            self.gl.bind_framebuffer(DRAW_FRAMEBUFFER, None);

            self.gl.blit_framebuffer(
                0,
                0,
                self.dims.x,
                self.dims.y,
                0,
                0,
                self.dims.x,
                self.dims.y,
                DEPTH_BUFFER_BIT,
                NEAREST,
            );
            self.gl.bind_framebuffer(FRAMEBUFFER, None);
        }
    }

    fn render_screen_quad(&self) {
        unsafe {
            self.lighting_shader.set_used(&self.gl);

            self.gl.bind_vertex_array(Some(self.screen_quad.vao()));
            self.gl
                .draw_elements(TRIANGLES, self.screen_quad.len() as i32, UNSIGNED_INT, 0);
            self.gl.bind_vertex_array(None);
        }
    }

    pub fn render(&self, model: &Model, transform: &Transform) {
        unsafe {
            self.geometry_shader.set_used(&self.gl);
            self.geometry_shader
                .set_mat4(&self.gl, "model", transform.matrix());

            self.gl.bind_vertex_array(Some(model.vao()));
            self.gl
                .draw_elements(TRIANGLES, model.len() as i32, UNSIGNED_INT, 0);
            self.gl.bind_vertex_array(None);
        }
    }

    pub fn handle_resize(&mut self, width: i32, height: i32) {
        unsafe {
            self.dims.x = width;
            self.dims.y = height;

            self.gl.viewport(0, 0, width, height);
        }
    }

    pub fn create_skybox(&self) -> Skybox {
        let cube = Mesh::from_cube(2.);
        let model = self.create_model(&cube);

        Skybox::new(&self.gl, model)
    }

    pub fn render_skybox(&self, model: &Model, camera: &Camera, cube_map: &CubeMap) {
        unsafe {
            self.gl.depth_func(LEQUAL);
            self.skybox_shader.set_used(&self.gl);
            let view = Mat4::from_mat3(Mat3::from_mat4(camera.view()));
            let projection_view = camera.projection() * view;
            self.skybox_shader
                .set_mat4(&self.gl, "projection_view", projection_view);

            cube_map.bind(&self.gl);

            self.gl.enable(DEPTH_TEST);
            self.gl.cull_face(FRONT);

            self.gl.bind_vertex_array(Some(model.vao()));
            self.gl
                .draw_elements(TRIANGLES, model.len() as i32, UNSIGNED_INT, 0);
            self.gl.bind_vertex_array(None);

            self.gl.depth_func(LESS);
            self.gl.cull_face(BACK);
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

struct GBuffer {
    framebuffer: Framebuffer,
    position: Texture,
    normal: Texture,
    albedo_spec: Texture,
}

unsafe fn create_g_buffer(gl: &Context, dims: &IVec2) -> GBuffer {
    let g_buffer = gl
        .create_framebuffer()
        .expect("Couldn't create framebuffer.");

    gl.bind_framebuffer(FRAMEBUFFER, Some(g_buffer));

    let g_position = create_color_buffer(gl, dims, RGBA16F, FLOAT, COLOR_ATTACHMENT0);
    let g_normal = create_color_buffer(gl, dims, RGBA16F, FLOAT, COLOR_ATTACHMENT1);
    let g_albedo_spec = create_color_buffer(gl, dims, RGBA, UNSIGNED_BYTE, COLOR_ATTACHMENT2);

    let attachments = [COLOR_ATTACHMENT0, COLOR_ATTACHMENT1, COLOR_ATTACHMENT2];

    gl.draw_buffers(&attachments);

    let depth_buffer = gl
        .create_renderbuffer()
        .expect("Couldn't create renderbuffer.");

    gl.bind_renderbuffer(RENDERBUFFER, Some(depth_buffer));

    gl.renderbuffer_storage(RENDERBUFFER, DEPTH_COMPONENT, dims.x, dims.y);

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

unsafe fn create_color_buffer(
    gl: &Context,
    dims: &IVec2,
    format: u32,
    type_: u32,
    attachment: u32,
) -> Texture {
    let texture = gl.create_texture().expect("Couldn't create texture.");
    gl.bind_texture(TEXTURE_2D, Some(texture));

    gl.tex_image_2d(
        TEXTURE_2D,
        0,
        format as i32,
        dims.x,
        dims.y,
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

fn create_screen_quad(gl: &Context, quad_indices: &Vec<u32>) -> Model {
    let mut screen_mesh = Mesh::new();
    let temp_normal = Vec3::ZERO;
    screen_mesh.push_quad(Quad::new(
        Vertex::new(vec3(-1., -1., 0.), temp_normal, vec2(0., 0.)),
        Vertex::new(vec3(1., -1., 0.), temp_normal, vec2(1., 0.)),
        Vertex::new(vec3(1., 1., 0.), temp_normal, vec2(1., 1.)),
        Vertex::new(vec3(-1., 1., 0.), temp_normal, vec2(0., 1.)),
    ));
    Model::new(gl, quad_indices, &screen_mesh)
}

// fn create_skybox(gl: &Context, quad_indices: &Vec<u32>) -> Model {

// }
