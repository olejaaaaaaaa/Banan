

use std::collections::HashMap;

use log::warn;
use rand::Rng;
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, Buffer, BufferUsages, PrimitiveTopology};

use super::{Entity, GameWorld, Id, Position3D, Vertex3D, WebGPUType};
use getrandom::getrandom;

fn get_unique_id<T: 'static>(entity: &Entity) -> i64 {

    let mut rng = rand::thread_rng();
    let mut is_unique_id = true;

    loop {
        let id = rng.gen_range(std::i64::MIN..std::i64::MAX);

        for i in &entity.components {
            if let Some(x) = i.downcast_ref::<HashMap<Id, T>>() {

                for j in x.keys() {
                    if *j == id {
                        is_unique_id = false;
                    }
                }
            }
        }

        if is_unique_id { return id }
    }
}

pub struct Mesh3DTexture {
    texture: usize,
}

#[derive(Debug)]
pub struct Mesh3D {
    pub vertex:         Vec<Vertex3D>,
    pub vertex_count:   Id,
    pub vertex_buffer:  Id,
    pub index_buffer:   Option<Id>,
    pub indeces:        Option<Id>,
    pub topology:       wgpu::PrimitiveTopology
}

impl Mesh3D {

    pub fn new(entity: &Entity, vertex: Vec<Vertex3D>, topology: PrimitiveTopology,) -> Self {

        let buffer = entity.game_resource.borrow().ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: vertex.bytes(),
            usage: BufferUsages::COPY_DST | BufferUsages::VERTEX,
        });

        let id = get_unique_id::<Vertex3D>(entity);

        let mut res = entity.game_resource.borrow_mut();

        res.vertex_buffer.insert(id, buffer);
        res.vertex_count.insert(id, vertex.len());

        Self {
            vertex_count:   id,
            vertex:         vertex,
            vertex_buffer:  id,
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

        // entity.game_resource.borrow_mut().indeces.push(indeces.to_vec());
        // self.indeces = Some(entity.game_resource.borrow().indeces.len()-1);

        // entity.game_resource.borrow_mut().index_buffer.push(buffer);
        // self.index_buffer = Some(entity.game_resource.borrow().index_buffer.len()-1);

        self
    }
}



