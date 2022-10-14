use glam::{Vec2, Vec3};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Vertex {
    position: Vec3,
    normal: Vec3,
    tex_coords: Vec2,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, tex_coords: Vec2, ao: f32) -> Self {
        Self {
            position,
            normal,
            tex_coords,
        }
    }
}

pub struct Quad {
    a: Vertex,
    b: Vertex,
    c: Vertex,
    d: Vertex,
}

impl Quad {
    pub fn new(a: Vertex, b: Vertex, c: Vertex, d: Vertex) -> Self {
        Self { a, b, c, d }
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    vertices: Vec<Vertex>,
}

impl Mesh {
    pub fn new() -> Self {
        let vertices = Vec::<Vertex>::new();
        Mesh { vertices }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let vertices = Vec::<Vertex>::with_capacity(capacity);
        Mesh { vertices }
    }

    pub fn with_vertices(vertices: Vec<Vertex>) -> Self {
        Mesh { vertices }
    }

    pub fn push(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    pub fn push_quad(&mut self, quad: Quad) {
        self.vertices.push(quad.a);
        self.vertices.push(quad.b);
        self.vertices.push(quad.c);
        self.vertices.push(quad.d);
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}
