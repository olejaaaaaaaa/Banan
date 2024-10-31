

use wgpu::{util::{BufferInitDescriptor, DeviceExt}, Buffer, BufferUsages, PrimitiveTopology};

use super::{Entity, GameWorld, Position3D, Vertex3D, WebGPUType};

pub struct Default3DMesh {
    pub vertex:         Vec<Vertex3D>,
    pub vertex_count:   usize,
    pub vertex_buffer:  usize,
    pub index_buffer:   Option<usize>,
    pub indeces:        Option<usize>,
    pub topology:       wgpu::PrimitiveTopology
}

impl Default3DMesh {

    pub fn new(entity: &Entity, vertex: Vec<Vertex3D>, topology: PrimitiveTopology,) -> Self {

        let buffer = entity.game_resource.borrow().ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: vertex.bytes(),
            usage: BufferUsages::COPY_DST | BufferUsages::VERTEX,
        });

        entity.game_resource.borrow_mut().vertex_buffer.push(buffer);
        let index = entity.game_resource.borrow().vertex_buffer.len();

        Self {
            vertex_count:   vertex.len(),
            vertex:         vertex,
            vertex_buffer:  index,
            topology:       topology,
            indeces:        None,
            index_buffer:   None
        }
    }

    pub fn add_indeces(mut self, entity: &Entity, indeces: &[u16]) -> Self {

        let buffer = entity.game_resource.borrow().ctx.device.create_buffer_init(&BufferInitDescriptor {
            label:      Some(" "),
            contents:   bytemuck::cast_slice(indeces),
            usage:      BufferUsages::INDEX | BufferUsages::COPY_DST,
        });

        entity.game_resource.borrow_mut().indeces.push(indeces.to_vec());
        self.indeces = Some(entity.game_resource.borrow().indeces.len()-1);

        entity.game_resource.borrow_mut().index_buffer.push(buffer);
        self.index_buffer = Some(entity.game_resource.borrow().index_buffer.len()-1);

        self
    }
}



