

use wgpu::{util::{BufferInitDescriptor, DeviceExt}, Buffer, BufferUsages};

use super::{GameWorld, Vertex3D};

pub struct Default3DMesh {
    vertex:         Vec<Vertex3D>,
    vertex_count:   usize,
    vertex_buffer:  Vec<Buffer>,
    topology:       wgpu::PrimitiveTopology
}

impl Default3DMesh {
    pub fn new(world: &GameWorld, vertex: Vec<Vertex3D>) {
        world.resource.ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: &[],
            usage: BufferUsages::COPY_DST | BufferUsages::VERTEX,
        });
    }
}



