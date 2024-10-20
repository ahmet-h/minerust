use super::vertex::Vertex;

pub struct Mesh<V: Vertex> {
    vertices: Vec<V>,
}

impl<V: Vertex> Mesh<V> {
    pub fn new() -> Self {
        Self {
            vertices: Vec::<V>::new(),
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
    }

    pub fn vertices(&self) -> &[V] {
        &self.vertices
    }

    pub fn vertices_mut(&mut self) -> &mut [V] {
        &mut self.vertices
    }

    pub fn vertices_mut_vec(&mut self) -> &mut Vec<V> {
        &mut self.vertices
    }

    pub fn push(&mut self, vert: V) {
        self.vertices.push(vert);
    }

    pub fn push_quad(&mut self, quad: Quad<V>) {
        self.vertices.push(quad.a);
        self.vertices.push(quad.b);
        self.vertices.push(quad.c);
        self.vertices.push(quad.d);
    }
}

impl<V: Vertex> Default for Mesh<V> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Quad<V: Vertex> {
    a: V,
    b: V,
    c: V,
    d: V,
}

impl<V: Vertex> Quad<V> {
    pub fn new(a: V, b: V, c: V, d: V) -> Self {
        Self { a, b, c, d }
    }
}
