
use std::{any::Any, collections::HashMap, rc::Rc};

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


struct Entity<'s> {
    pub game_resource: Rc<GameResource<'s>>,
    pub components: HashMap<usize, Box<dyn Any>>
}

impl<'s> Entity<'s> {

    pub fn add_component<T: 'static>(&mut self, component: T) {
        let id = self.components.len();
        self.components.insert(id, Box::new(component));
    }

    pub fn get_component<T: 'static>(&self, id: usize) -> Option<&T> {
        self.components.get(&id)?.downcast_ref::<T>()
    }

    pub fn remove_component(&mut self, id: usize) {
        self.components.remove(&id);
    }

    pub fn new(game_resource: Rc<GameResource<'s>>) -> Self {
        Self {
            game_resource,
            components: HashMap::new()
        }
    }
}

pub struct GameResource<'s> {
    pub ctx:                        Rc<WebGPUContext<'s>>,
    pub render_pipeline:            Vec<RenderPipeline>,
    pub count_vertex:               Vec<usize>,
    pub vertex_buffer:              Vec<Buffer>,
}

impl<'s> GameResource<'s> {
    async fn new(ctx: WebGPUContext<'s>, window: &'s Window) -> Rc<Self> {
        Rc::new(Self {
            ctx: ctx.into(),
            render_pipeline: vec![],
            count_vertex: vec![],
            vertex_buffer: vec![],
        })
    }
}

pub struct GameWorld<'s> {
    pub resource:   Rc<GameResource<'s>>,
    pub entity:     Vec<Entity<'s>>
}

impl<'s> GameWorld<'s> {
    pub async fn new(ctx: WebGPUContext<'s>, window: &'s Window) -> Self {
        Self {
            resource: GameResource::new(ctx, window).await,
            entity: vec![]
        }
    }

    pub fn create_entity(&self) -> Entity {
        Entity::new(self.resource.clone())
    }
}


