use wgpu::{BindGroupLayoutEntry, BindingType, ShaderStages};

use crate::get_unique_id;

use super::{Entity, Id};


pub trait TraitBindGroupLayoutEntry {
    fn add_bind_group_layout_entry(&mut self, binding: u32, visible: ShaderStages, binding_type: BindingType);
}

impl TraitBindGroupLayoutEntry for Entity<'_> {
    fn add_bind_group_layout_entry(&mut self, binding: u32, visible: ShaderStages, binding_type: BindingType) {
        let component = ComponentBindGroupLayoutEntry::new(self, binding, visible, binding_type);
        self.add_component(component);
    }
}

pub struct ComponentBindGroupLayoutEntry {
    pub bind_group_layout_entry_id: Id
}

impl ComponentBindGroupLayoutEntry {
    pub fn new(entity: &Entity, binding: u32, visible: ShaderStages, binding_type: BindingType) -> ComponentBindGroupLayoutEntry {
        let res = entity.game_resource.borrow();

        let bind = BindGroupLayoutEntry {
            binding,
            visibility: visible,
            ty:         binding_type,
            count:      None,
        };

        let id = get_unique_id::<BindGroupLayoutEntry>(entity);
        entity.game_resource.borrow_mut().bind_group_layout_entry.insert(id, bind);

        Self {
            bind_group_layout_entry_id: id
        }

    }
}