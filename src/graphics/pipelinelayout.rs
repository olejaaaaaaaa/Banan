use std::cell::RefMut;

use wgpu::{BindGroupLayout, PipelineLayout, PipelineLayoutDescriptor};

use crate::get_unique_id;

use super::{Entity, GameResource, Id};

pub struct ComponentPipelineLayout {
    pub pipeline_layout_id: Id
}

impl ComponentPipelineLayout {
    fn new(entity: &Entity, bind_group_layout: &[&BindGroupLayout]) -> Self {
        let mut res: RefMut<GameResource> = entity.game_resource.borrow_mut();

        let pipeline_layout = res.ctx.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: bind_group_layout,
            push_constant_ranges: &[]
        });

        let id = get_unique_id::<PipelineLayout>(entity);
        entity.game_resource.borrow_mut().pipeline_layout.insert(id, pipeline_layout);

        Self {
            pipeline_layout_id: id
        }
    }
}