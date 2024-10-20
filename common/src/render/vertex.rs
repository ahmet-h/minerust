use glow::FLOAT;

pub trait Vertex {
    fn layout() -> VertexLayout;
}

pub struct VertexLayout {
    pub attributes: Vec<VertextAttribute>,
}

pub struct VertextAttribute {
    pub format: VertexFormat,
}

pub enum VertexFormat {
    Float32,
    Float32x2,
    Float32x3,
    Float32x4,
}

impl VertexFormat {
    pub const fn size(&self) -> (i32, i32) {
        match self {
            VertexFormat::Float32 => (1, 4),
            VertexFormat::Float32x2 => (2, 8),
            VertexFormat::Float32x3 => (3, 12),
            VertexFormat::Float32x4 => (4, 16),
        }
    }

    pub const fn data_type(&self) -> u32 {
        match self {
            VertexFormat::Float32
            | VertexFormat::Float32x2
            | VertexFormat::Float32x3
            | VertexFormat::Float32x4 => FLOAT,
        }
    }
}
