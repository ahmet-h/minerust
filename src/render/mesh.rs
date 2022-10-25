use glam::{vec2, vec3, Vec2, Vec3};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Vertex {
    position: Vec3,
    normal: Vec3,
    tex_coords: Vec2,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, tex_coords: Vec2) -> Self {
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

    pub fn from_cube(cube_size: f32) -> Self {
        let (x, y, z) = (0., 0., 0.);

        let points = vec![
            vec3(x, y, z),                                     // 0
            vec3(x + cube_size, y, z),                         // 1
            vec3(x + cube_size, y + cube_size, z),             // 2
            vec3(x, y + cube_size, z),                         // 3
            vec3(x, y, z + cube_size),                         // 4
            vec3(x + cube_size, y, z + cube_size),             // 5
            vec3(x + cube_size, y + cube_size, z + cube_size), // 6
            vec3(x, y + cube_size, z + cube_size),             // 7
        ];

        let center = vec3(x + cube_size / 2., y + cube_size / 2., z + cube_size / 2.);

        let mut vertices = Vec::new();
        for i in 0..6 {
            let n = CUBE_NORMALS[i];
            let quad_normal = vec3(n.0, n.1, n.2);

            for j in 0..4 {
                let point = points[CUBE_POINTS[i * 4 + j]] - center;

                vertices.push(Vertex::new(
                    point,
                    quad_normal,
                    vec2(CUBE_TEX_COORDS[j].0, CUBE_TEX_COORDS[j].1),
                ));
            }
        }

        Self { vertices }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

const CUBE_POINTS: [usize; 24] = [
    0, 4, 7, 3, // -X
    0, 1, 5, 4, // -Y
    1, 0, 3, 2, // -Z
    5, 1, 2, 6, // +X
    7, 6, 2, 3, // +Y
    4, 5, 6, 7, // +Z
];

const CUBE_NORMALS: [(f32, f32, f32); 6] = [
    (-1., 0., 0.), // -X
    (0., -1., 0.), // -Y
    (0., 0., -1.), // -Z
    (1., 0., 0.),  // +X
    (0., 1., 0.),  // +Y
    (0., 0., 1.),  // +Z
];

const CUBE_TEX_COORDS: [(f32, f32); 4] = [(0., 0.), (1., 0.), (1., 1.), (0., 1.)];
