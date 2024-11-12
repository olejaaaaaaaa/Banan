use wgpu::BindGroupLayoutDescriptor;
use wgpu::BindGroupLayout;

use crate::get_unique_id;

use super::{ComponentBindGroupLayoutEntry, Entity, Id};


pub trait TraitBindGroupLayout {
    fn add_bind_group_layout(&mut self);
}

impl TraitBindGroupLayout for Entity<'_> {
    fn add_bind_group_layout(&mut self) {
        let component = ComponentBindGrouplayout::new(self);
        self.add_component(component);
    }
}

pub struct ComponentBindGrouplayout {
    pub bind_group_layout_id: Id
}

impl ComponentBindGrouplayout {
    fn new(entity: &Entity) -> ComponentBindGrouplayout {

        match entity.get_components::<ComponentBindGroupLayoutEntry>() {

            Some(bind) => {

                let mut res = entity.game_resource.borrow_mut();
                let entry = bind.iter().map(|x| res.bind_group_layout_entry[&x.bind_group_layout_entry_id]).collect::<Vec<_>>();

                let bind_group_layout = res.ctx.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: entry.as_slice()
                });

                let id = get_unique_id::<BindGroupLayout>(entity);
                res.bind_group_layout.insert(id, bind_group_layout);


                Self {
                    bind_group_layout_id: id
                }
            }

            None => {
                let mut res = entity.game_resource.borrow_mut();

                let bind_group_layout = res.ctx.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[]
                });

                let id = get_unique_id::<BindGroupLayout>(entity);
                res.bind_group_layout.insert(id, bind_group_layout);

                Self {
                    bind_group_layout_id: id
                }
            }
        }
    }
}