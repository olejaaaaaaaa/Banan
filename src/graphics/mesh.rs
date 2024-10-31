

use std::{cell::RefMut, collections::HashMap};

use log::warn;
use rand::Rng;
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BindingType, Buffer, BufferUsages, PrimitiveTopology};

use crate::get_unique_id;

use super::{Entity, GameResource, GameWorld, Id, Position3D, Vertex3D, WebGPUType};

pub struct Mesh3DTexture {
    texture: usize,
}

pub trait Trait3DMesh {
    fn add_mesh(&mut self, vertex: Vec<Vertex3D>, topology: PrimitiveTopology) -> &mut Mesh3D;
}

impl Trait3DMesh for Entity<'_> {

    fn add_mesh(&mut self, vertex: Vec<Vertex3D>, topology: PrimitiveTopology) -> &mut Mesh3D {
        self.add_component(Mesh3D::new(self, vertex, topology));

        use wgpu::*;



        self.components.last_mut().unwrap().downcast_mut::<Mesh3D>().unwrap()


    }

}

#[derive(Debug)]
pub struct Mesh3D {
    pub vertex:             Vec<Vertex3D>,
    pub vertex_count_id:    Id,
    pub vertex_buffer_id:   Id,
    pub index_buffer_id:    Option<Id>,
    pub indeces_id:         Option<Id>,
    pub topology:           PrimitiveTopology
}

impl Mesh3D {

    pub fn new(entity: &Entity, vertex: Vec<Vertex3D>, topology: PrimitiveTopology,) -> Self {

        let buffer = entity.game_resource.borrow().ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: vertex.bytes(),
            usage: BufferUsages::COPY_DST | BufferUsages::VERTEX,
        });

        let vertex_buffer_id = get_unique_id::<Buffer>(entity);
        let vertex_count_id = get_unique_id::<usize>(entity);

        let mut res: RefMut<GameResource> = entity.game_resource.borrow_mut();

        res.vertex_buffer.insert(vertex_buffer_id, buffer);
        res.vertex_count.insert(vertex_count_id, vertex.len());

        Self {
            vertex_count_id:    vertex_count_id,
            vertex:             vertex,
            vertex_buffer_id:   vertex_buffer_id,
            topology:           topology,
            indeces_id:         None,
            index_buffer_id:    None
        }
    }
}



