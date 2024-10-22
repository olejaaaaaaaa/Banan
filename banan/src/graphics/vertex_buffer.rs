use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};
use bytemuck::*;

pub trait Vertex {
    fn bytes(&self) -> &[u8];
    fn layout(&self) -> &VertexBufferLayout<'static>;
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Default, Debug, Clone, Copy)]
pub struct Vertex3D {
    pub pos: [f32; 3],
    pub color: [f32; 3]
}

impl Vertex for Vec<Vertex3D> {

    fn bytes(&self) -> &[u8] {
        &bytemuck::cast_slice(self)
    }

    fn layout(&self) -> &VertexBufferLayout<'static> {

        &VertexBufferLayout {
            
            array_stride: size_of::<Vertex3D>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[ 
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0
                },

                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1
                }
            ]
        }
        
    }
}


