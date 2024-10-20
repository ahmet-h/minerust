use glow::{
    Buffer, Context, HasContext, VertexArray, ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER, FLOAT,
    STATIC_DRAW,
};

use super::{mesh::Mesh, vertex::Vertex};

#[derive(Clone, Copy)]
pub struct Model<V: Vertex> {
    vao: VertexArray,
    vbo: Buffer,
    ebo: Buffer,
    len: usize,
    phantom_data: std::marker::PhantomData<V>,
}

impl<V: Vertex> Model<V> {
    pub fn new(gl: &Context, mesh: &Mesh<V>) -> Self {
        let model = unsafe {
            let vao = gl
                .create_vertex_array()
                .expect("Couldn't create vertex array.");

            let vbo = gl.create_buffer().expect("Couldn't create buffer.");
            let ebo = gl.create_buffer().expect("Couldn't create buffer.");

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));

            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
            let indices = create_quad_indices(24);
            let ebo_data = core::slice::from_raw_parts(
                indices.as_ptr() as *const u8,
                indices.len() * std::mem::size_of::<u32>(),
            );
            gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, ebo_data, STATIC_DRAW);

            let layout = V::layout();
            let mut offset = 0;
            for (i, attribute) in layout.attributes.iter().enumerate() {
                gl.enable_vertex_attrib_array(i as u32);
                let size = attribute.format.size();
                gl.vertex_attrib_pointer_f32(
                    i as u32,
                    size.0,
                    attribute.format.data_type(),
                    false,
                    std::mem::size_of::<V>() as i32,
                    offset,
                );

                offset += size.1;
            }

            gl.bind_vertex_array(None);
            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, None);
            gl.bind_buffer(ARRAY_BUFFER, None);

            Self {
                vao,
                vbo,
                ebo,
                len: (mesh.vertices().len() / 4) * 6,
                phantom_data: std::marker::PhantomData,
            }
        };

        model
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
