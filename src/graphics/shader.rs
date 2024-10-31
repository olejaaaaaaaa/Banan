use log::warn;
use wgpu::{ShaderModule, ShaderModuleDescriptor};
use crate::{get_unique_id, load_shader};
use super::{Entity, Id};

pub trait TraitShader {
    fn add_shader(&mut self, file_name: &str);
}

impl TraitShader for Entity<'_> {
    fn add_shader(&mut self, file_name: &str) {
        let component_shader = ComponentShader::new(self, file_name);
        self.add_component(component_shader);
    }
}

#[derive(Debug)]
pub struct ComponentShader {
    pub shader_id: Id,
}

impl ComponentShader {
    fn new(entity: &Entity, file_name: &str) -> Self {

        let mut res = entity.game_resource.borrow_mut();
        let shader = res.ctx.device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(load_shader(file_name))),
        });

        let id = get_unique_id::<ShaderModule>(entity);
        res.shader.insert(id, shader);

        ComponentShader {
            shader_id: id
        }
    }
}

impl Drop for ComponentShader{
    fn drop(&mut self) {
        warn!("Drop Component Shader {}", self.shader_id);
    }
}