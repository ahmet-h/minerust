use glam::Vec3;
use glow::*;

use super::mesh::{Mesh, Vertex};

#[derive(Clone)]
pub struct Model {
    vao: VertexArray,
    vbo: Buffer,
    ebo: Buffer,
    len: usize,
}

impl Model {
    pub fn new(gl: &Context, indices: &Vec<u32>, mesh: &Mesh) -> Self {
        let mut model = unsafe {
            let vao = gl
                .create_vertex_array()
                .expect("Couldn't create vertex array.");

            let vbo = gl.create_buffer().expect("Couldn't create buffer.");
            let ebo = gl.create_buffer().expect("Couldn't create buffer.");

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));

            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
            let ebo_data = core::slice::from_raw_parts(
                indices.as_ptr() as *const u8,
                indices.len() * std::mem::size_of::<u32>(),
            );
            gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, ebo_data, STATIC_DRAW);

            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(
                0,
                3,
                FLOAT,
                false,
                std::mem::size_of::<Vertex>() as i32,
                0,
            );

            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(
                1,
                3,
                FLOAT,
                false,
                std::mem::size_of::<Vertex>() as i32,
                std::mem::size_of::<Vec3>() as i32,
            );

            gl.enable_vertex_attrib_array(2);
            gl.vertex_attrib_pointer_f32(
                2,
                2,
                FLOAT,
                false,
                std::mem::size_of::<Vertex>() as i32,
                (std::mem::size_of::<Vec3>() * 2) as i32,
            );

            gl.bind_vertex_array(None);
            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, None);
            gl.bind_buffer(ARRAY_BUFFER, None);

            Self {
                vao,
                vbo,
                ebo,
                len: (mesh.vertices().len() / 4) * 6,
            }
        };

        model.update(gl, mesh);

        model
    }

    pub fn vao(&self) -> VertexArray {
        self.vao
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn update(&self, gl: &Context, mesh: &Mesh) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));

            gl.bind_buffer(ARRAY_BUFFER, Some(self.vbo));
            let vertices = mesh.vertices();
            let vbo_data = core::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * std::mem::size_of::<Vertex>(),
            );
            gl.buffer_data_u8_slice(ARRAY_BUFFER, vbo_data, STATIC_DRAW);

            gl.bind_vertex_array(None);
            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, None);
            gl.bind_buffer(ARRAY_BUFFER, None);
        }
    }
}

// impl Drop for Model {
//     fn drop(&mut self) {
//         unsafe {
//             gl::DeleteBuffers(1, &self.ebo);
//             gl::DeleteBuffers(1, &self.vbo);
//             gl::DeleteVertexArrays(1, &self.vao);
//         }
//     }
// }
