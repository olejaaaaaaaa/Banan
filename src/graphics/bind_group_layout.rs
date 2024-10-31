use wgpu::BindGroupLayoutDescriptor;
use wgpu::BindGroupLayout;

use crate::get_unique_id;

use super::{ComponentBindGroupLayoutEntry, Entity, Id};


trait TraitBindGroupLayout {
    fn add_bind_group_layout(&self);
}

impl TraitBindGroupLayout for Entity<'_> {
    fn add_bind_group_layout(&self) {
        let component = ComponentBindGrouplayout::new(self);
    }
}

pub struct ComponentBindGrouplayout {
    pub bind_group_layout_id: Id
}

impl ComponentBindGrouplayout {
    fn new(entity: &Entity) -> ComponentBindGrouplayout {

        let bind_group_layout_entry = entity.get_components::<ComponentBindGroupLayoutEntry>().unwrap().clone();
        let res = entity.game_resource.borrow();
        let entry = bind_group_layout_entry.iter().map(|x| res.bind_group_layout_entry[&x.bind_group_layout_entry_id]).collect::<Vec<_>>();

        let bind_group_layout_entry = res.ctx.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: entry.as_slice()
        });

        let id = get_unique_id::<BindGroupLayout>(entity);
        res.bind_group_layout.insert(id, bind_group_layout);

        Self {
            bind_group_layout_id: id
        }
    }
}