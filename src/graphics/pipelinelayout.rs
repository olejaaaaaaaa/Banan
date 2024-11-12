use std::cell::RefMut;

use wgpu::{BindGroupLayout, PipelineLayout, PipelineLayoutDescriptor};

use crate::get_unique_id;

use super::{ComponentBindGrouplayout, Entity, GameResource, Id};

pub trait TraitPipelineLayout {
    fn add_pipeline_layout(&mut self);
}

impl TraitPipelineLayout for Entity<'_> {
    fn add_pipeline_layout(&mut self) {
        let component = ComponentPipelineLayout::new(self);
        self.add_component(component);
    }
}

pub struct ComponentPipelineLayout {
    pub pipeline_layout_id: Id
}

impl ComponentPipelineLayout {
    fn new(entity: &Entity) -> Self {
        let mut res: RefMut<GameResource> = entity.game_resource.borrow_mut();

        match entity.get_components::<ComponentBindGrouplayout>() {

            Some(bind) => {

                let bind_group_layout = bind.iter().map(|x| {
                    &res.bind_group_layout[&x.bind_group_layout_id]
                }).collect::<Vec<_>>();

                let pipeline_layout = res.ctx.device.create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &bind_group_layout,
                    push_constant_ranges: &[]
                });

                let id = get_unique_id::<PipelineLayout>(entity);
                res.pipeline_layout.insert(id, pipeline_layout);

                Self {
                    pipeline_layout_id: id
                }

            },

            None => {

                let pipeline_layout = res.ctx.device.create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[],
                    push_constant_ranges: &[]
                });

                let id = get_unique_id::<PipelineLayout>(entity);
                entity.game_resource.borrow_mut().pipeline_layout.insert(id, pipeline_layout);

                Self {
                    pipeline_layout_id: id
                }

            }
        }

    }
}