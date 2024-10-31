use super::{Entity, Id};

trait TraitBindGroupEntry {
    fn add_bind_group_entry(&mut self);
}

impl TraitBindGroupEntry for ComponentBindGroupEntry {
    fn add_bind_group_entry(&mut self) {

    }
}

pub struct ComponentBindGroupEntry {
    pub bind_group_entry_id: Id,
}

impl ComponentBindGroupEntry {
    pub fn new(entity: &Entity, binding: u32) {
        let res = entity.game_resource.borrow();

    }
}