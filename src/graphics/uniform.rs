
type Id = i64;

pub trait TraitUniform {
    fn add_uniform(&mut self);
}

impl TraitUniform for ComponentUniform {
    fn add_uniform(&mut self) {

    }
}

pub struct ComponentUniform {
    id: Id
}



