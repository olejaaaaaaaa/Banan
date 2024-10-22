
use std::{any::Any, rc::Rc};
use log::warn;
use wgpu::*;
use crate::{Vertex, Vertex3D, WebGPUContext};
const default_sahder: &str = include_str!("../shaders/test.wgsl");

pub struct RenderObject<'a> {
    pub webgpu_context: Rc<WebGPUContext<'a>>,
    pub render: wgpu::RenderPipeline,
    pub mesh: Vec<Vec<Vertex3D>>,
    pub buffer: Vec<Buffer>,
    pub count_vertex: Vec<usize>,
    pub properties: Vec<Box<dyn Any>>,
}

impl<'s> RenderObject<'s> {

    pub fn default(ctx: Rc<WebGPUContext<'s>>, _type: PrimitiveTopology) -> Self {
        let pipeline_layout = ctx.create_pipeline_layout(vec![]);
        let shader = ctx.create_shader(default_sahder);

        let binding = vec![Vertex3D{pos: [0.0, 0.0, 0.0], color: [0.0, 0.0, 0.0]}];
        let vertex_layout = binding.layout();

        let pipeline_render = ctx.create_pipeline(_type, pipeline_layout, shader, vertex_layout);

        Self {
            webgpu_context: ctx,
            properties: vec![],
            render: pipeline_render,
            mesh: vec![],
            buffer: vec![],
            count_vertex: vec![]
        }
    }

    pub fn add_mesh(&mut self, vertex: Vec<Vertex3D>) {
        self.mesh.push(vertex.clone());
        let buf = self.webgpu_context.create_vertex_buffer(vertex.bytes());
        self.buffer.push(buf);
        self.count_vertex.push(vertex.len());
    }

    pub fn set_mesh(&mut self,  vertex: Vec<Vertex3D>, index: usize) {
        if index > vertex.len()-1 { warn!("Index > Vertex len"); return; }
        
        self.mesh[index] = vertex;
        self.webgpu_context.update_buffer(&self.buffer[index], self.mesh[index].bytes());
    }

    pub fn get_mesh(&self, index: usize) -> Vec<Vertex3D> {
        self.mesh[index].clone()
    }

    pub fn sub_mesh(&mut self, index: usize) {
        if index > self.mesh.len()-1 { warn!("Index > Vertex len"); return; }

        self.buffer.remove(index);
        self.count_vertex.remove(index);
        self.mesh.remove(index);
    }

}

struct Shader {
    
}


