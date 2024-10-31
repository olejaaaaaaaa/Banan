use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBinding, ShaderStages};

use crate::get_unique_id;
use super::{Entity, Id};

pub struct ComponentBindGroup {
    bind_group_id: Id
}

impl ComponentBindGroup {
    fn new(entity: &Entity) {
        let res = entity.game_resource.borrow();

        let bind_group = res.ctx.device.create_bind_group(&BindGroupDescriptor{
            label: None,
            layout: todo!(),
            entries: todo!(),
        });
    }
}







