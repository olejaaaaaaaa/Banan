use wgpu::VertexAttribute;
use wgpu::VertexBufferLayout;
use wgpu::VertexStepMode;
use bytemuck::{Pod, Zeroable};




#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex3D {
    pub pos:   [f32; 3],
    pub color: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex2D {
    pub pos:   [f32; 2],
    pub color: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Position3D {
    pub pos: [f32; 3]
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex3DTexture {
    pub pos:        [f32; 3],
    pub tex_pos:    [f32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Time {
    pub time: [f32; 3] // hours, secs, millisecs
}

pub trait WebGPUType {
    fn layout(&self) -> VertexBufferLayout<'static>;
    fn bytes(&self)  -> &[u8];
}

impl WebGPUType for Vec<Vertex3D> {

    fn bytes(&self)  -> &[u8] {
        bytemuck::cast_slice(self)
    }

    fn layout(&self) -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride:   24,
            step_mode:      VertexStepMode::Vertex,
            attributes:     &[

                VertexAttribute {
                    format:             wgpu::VertexFormat::Float32x3,
                    offset:             0 as u64,
                    shader_location:    0,
                },

                VertexAttribute {
                    format:             wgpu::VertexFormat::Float32x3,
                    offset:             12,
                    shader_location:    1,
                }

            ],
        }
    }
}


impl WebGPUType for Vec<Vertex2D> {

    fn bytes(&self)  -> &[u8] {
        bytemuck::cast_slice(self)
    }

    fn layout(&self) -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride:   24,
            step_mode:      VertexStepMode::Vertex,
            attributes:     &[

                VertexAttribute {
                    format:             wgpu::VertexFormat::Float32x3,
                    offset:             0 as u64,
                    shader_location:    0,
                },

                VertexAttribute {
                    format:             wgpu::VertexFormat::Float32x2,
                    offset:             size_of::<[f32; 2]>() as u64,
                    shader_location:    1,
                }

            ],
        }
    }
}