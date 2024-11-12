
use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

use log::warn;
use wgpu::*;
use winit::dpi::PhysicalSize;
pub use winit::window::Window;

mod context;
pub use context::{WebGPUContext, WebGPUContextBuilder};

mod mesh;
pub use mesh::*;

mod render2d;
pub use render2d::*;

mod types;
pub use types::*;

mod uniform;
pub use uniform::*;

mod pipeline;
pub use pipeline::*;

mod shader;
pub use shader::*;

mod bind_group;
pub use bind_group::*;

mod bind_group_entry;
pub use bind_group_entry::*;

mod bind_group_layout;
pub use bind_group_layout::*;

mod bind_group_layout_entry;
pub use bind_group_layout_entry::*;

mod pipelinelayout;
pub use pipelinelayout::*;

type Id = i64;

pub struct Entity<'s, 'p> {
    pub game_resource: Rc<RefCell<GameResource<'s, 'p>>>,
    pub components: Vec<Box<dyn Any>>
}

impl Drop for Entity<'_, '_> {
    fn drop(&mut self) {
        warn!("DROP {}", self.components.len());
    }
}

impl<'s, 'p> Entity<'s, 'p> {

    pub fn add_component<T: 'static>(&mut self, component: T) {
        self.components.push(Box::new(component));
    }

    pub fn get_component<T: 'static>(&self) -> Option<&T> {
        for i in &self.components {
            if let Some(x) = i.downcast_ref::<T>() {
                return Some(x);
            }
        }
        None
    }

    pub fn get_mut_component<T: 'static>(&mut self) -> Option<&mut T>{
        for i in &mut self.components {
            if let Some(x) = i.downcast_mut::<T>() {
                return Some(x);
            }
        }
        None
    }

    pub fn get_mut_components<T: 'static>(&mut self) -> Option<Vec<&mut T>>{

        let mut v: Option<Vec<&mut T>> = None;

        for i in &mut self.components {
            if let Some(x) = i.downcast_mut::<T>() {
                match &mut v {
                    None => { v = Some(vec![x]) }
                    Some(n) => { n.push(x) }
                }
            }
        }

        v
    }

    pub fn get_components<T: 'static>(&self) -> Option<Vec<&T>>{

        let mut v: Option<Vec<&T>> = None;

        for i in &self.components {
            if let Some(x) = i.downcast_ref::<T>() {
                match &mut v {
                    None => { v = Some(vec![x]) }
                    Some(n) => { n.push(x) }
                }
            }
        }

        v
    }

    pub fn remove_component<T: 'static>(&mut self) {
        for i in 0..self.components.len() {
            if let Some(x) = self.components[i].downcast_ref::<T>() {
                self.components.remove(i);
                return;
            }
        }
    }

    pub fn remove_components<T: 'static>(&mut self) {
        for i in 0..self.components.len() {
            if let Some(x) = self.components[i].downcast_ref::<T>() {
                self.components.remove(i);
            }
        }
    }

    pub fn new(game_resource: Rc<RefCell<GameResource<'s, 'p>>>) -> Self {
        Self {
            game_resource,
            components: vec![]
        }
    }
}

pub struct GameResource<'s, 'p> {
    pub ctx:                        Rc<WebGPUContext<'s>>,
    pub render_pipeline:            HashMap<Id, RenderPipeline>,
    pub pipeline_layout:            HashMap<Id, PipelineLayout>,
    pub vertex_count:               HashMap<Id, usize>,
    pub vertex_buffer:              HashMap<Id, Buffer>,
    pub index_buffer:               HashMap<Id, Buffer>,
    pub indeces:                    HashMap<Id, Vec<u16>>,
    pub bind_group:                 HashMap<Id, BindGroup>,
    pub bind_group_layout:          HashMap<Id, BindGroupLayout>,
    pub bind_group_entry:           HashMap<Id, BindGroupEntry<'p>>,
    pub binding_resource:           HashMap<Id, BindingResource<'p>>,
    pub vertex_buffer_layout:       HashMap<Id, VertexBufferLayout<'static>>,
    pub uniform_buffer:             HashMap<Id, Buffer>,
    pub shader:                     HashMap<Id, ShaderModule>,
    pub bind_group_layout_entry:    HashMap<Id, BindGroupLayoutEntry>,
    pub texture_buffer:             HashMap<Id, Buffer>,
}

impl<'s, 'p> GameResource<'s, 'p> {
    async fn new(ctx: WebGPUContext<'s>, window: &'s Window) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            ctx:                        ctx.into(),
            vertex_count:               HashMap::new(),
            vertex_buffer:              HashMap::new(),
            vertex_buffer_layout:       HashMap::new(),
            uniform_buffer:             HashMap::new(),
            indeces:                    HashMap::new(),
            index_buffer:               HashMap::new(),
            shader:                     HashMap::new(),
            render_pipeline:            HashMap::new(),
            pipeline_layout:            HashMap::new(),
            bind_group:                 HashMap::new(),
            bind_group_layout:          HashMap::new(),
            bind_group_entry:           HashMap::new(),
            bind_group_layout_entry:    HashMap::new(),
            texture_buffer:             HashMap::new(),
            binding_resource:           HashMap::new(),
        }))
    }
}

pub struct GameWorld<'s, 'p> {
    pub resource:   Rc<RefCell<GameResource<'s, 'p>>>,
    pub entity:     Vec<Entity<'s, 'p>>
}

impl<'s, 'p> GameWorld<'s, 'p> {
    pub async fn new(ctx: WebGPUContext<'s>, window: &'s Window) -> Self {
        Self {
            resource: GameResource::new(ctx, window).await,
            entity: vec![]
        }
    }

    pub fn create_entity(&self) -> Entity<'s, 'p> {
        Entity::new(self.resource.clone())
    }

    pub fn resize(&self, size: PhysicalSize<u32>) {
        self.resource.borrow().ctx.resize(size);
    }
}


