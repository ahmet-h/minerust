use glam::Vec2;

use crate::render::vertex::{Vertex, VertexFormat, VertexLayout, VertextAttribute};

#[repr(C)]
pub struct SkyVertex {
    pos: Vec2,
}

impl SkyVertex {
    pub fn new(pos: Vec2) -> Self {
        Self { pos }
    }
}

impl Vertex for SkyVertex {
    fn layout() -> crate::render::vertex::VertexLayout {
        VertexLayout {
            attributes: vec![VertextAttribute {
                format: VertexFormat::Float32x2,
            }],
        }
    }
}
