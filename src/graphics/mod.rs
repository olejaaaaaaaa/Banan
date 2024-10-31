
use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

use wgpu::*;
pub use winit::window::Window;

mod context;
pub use context::{WebGPUContext, WebGPUContextBuilder};

mod mesh;
pub use mesh::*;

mod render;
pub use render::*;

mod types;
pub use types::*;

mod uniform;
pub use uniform::*;

mod pipeline;
pub use pipeline::*;

mod shader;
use shader::*;


pub struct Entity<'s,> {
    pub game_resource: Rc<RefCell<GameResource<'s,>>>,
    pub components: HashMap<usize, Box<dyn Any>>
}

impl<'s,> Entity<'s,> {

    pub fn add_component<T: 'static>(&mut self, component: T) {
        let id = self.components.len();
        self.components.insert(id, Box::new(component));
    }

    pub fn get_component<T: 'static>(&self) -> Option<&T> {
        for i in &self.components {
            if let Some(x) = i.1.downcast_ref::<T>() {
                return Some(x);
            }
        }
        None
    }

    pub fn get_mut_component<T: 'static>(&mut self, id: usize, component: T) -> Option<&mut T>{
       self.components.get_mut(&id)?.downcast_mut::<T>()
    }

    pub fn remove_component(&mut self, id: usize) {
        self.components.remove(&id);
    }

    pub fn new(game_resource: Rc<RefCell<GameResource<'s,>>>) -> Self {
        Self {
            game_resource,
            components: HashMap::new()
        }
    }
}

pub struct GameResource<'s> {
    pub ctx:                        Rc<WebGPUContext<'s>>,
    pub render_pipeline:            Vec<RenderPipeline>,
    pub pipeline_layout:            Vec<PipelineLayout>,
    pub count_vertex:               Vec<usize>,
    pub vertex_buffer:              Vec<Buffer>,
    pub index_buffer:               Vec<Buffer>,
    pub indeces:                    Vec<Vec<u16>>,
    pub bind_group:                 Vec<BindGroup>,
    pub vertex_buffer_layout:       Vec<VertexBufferLayout<'static>>,
    pub uniform_buffer:             Vec<Buffer>,
    pub shader:                     Vec<ShaderModule>,
}

impl<'s> GameResource<'s> {
    async fn new(ctx: WebGPUContext<'s>, window: &'s Window) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            ctx:                    ctx.into(),
            render_pipeline:        vec![],
            count_vertex:           vec![],
            vertex_buffer:          vec![],
            bind_group:             vec![],
            vertex_buffer_layout:   vec![],
            uniform_buffer:         vec![],
            indeces:                vec![],
            index_buffer:           vec![],
            pipeline_layout:        vec![],
            shader:                 vec![],
        }))
    }
}

pub struct GameWorld<'s,> {
    pub resource:   Rc<RefCell<GameResource<'s,>>>,
    pub entity:     Vec<Entity<'s,>>
}

impl<'s,> GameWorld<'s,> {
    pub async fn new(ctx: WebGPUContext<'s>, window: &'s Window) -> Self {
        Self {
            resource: GameResource::new(ctx, window).await,
            entity: vec![]
        }
    }

    pub fn create_entity(&self) -> Entity<'s,> {
        Entity::new(self.resource.clone())
    }
}


