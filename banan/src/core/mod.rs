
use std::any::Any;
use wgpu::*;
use crate::{Vertex, Vertex3D, WebGPUContext};
const default_sahder: &str = include_str!("../shaders/test.wgsl");

pub struct RenderObject {
    pub render: wgpu::RenderPipeline,
    pub mesh: Vec<Vec<Vertex3D>>,
    pub buffer: Vec<Buffer>,
    pub count_vertex: Vec<usize>,
    pub properties: Vec<Box<dyn Any>>,
}

impl RenderObject {

    pub fn default(ctx: &WebGPUContext, _type: PrimitiveTopology) -> Self {
        let pipeline_layout = ctx.create_pipeline_layout(vec![]);
        let shader = ctx.create_shader(default_sahder);

        let binding = vec![Vertex3D{pos: [0.0, 0.0, 0.0], color: [0.0, 0.0, 0.0]}];
        let vertex_layout = binding.layout();

        let pipeline_render = ctx.create_pipeline(_type, pipeline_layout, shader, vertex_layout);

        Self {
            properties: vec![],
            render: pipeline_render,
            mesh: vec![vec![]],
            buffer: vec![],
            count_vertex: vec![]
        }
    }

    pub fn add_mesh(&mut self, ctx: &WebGPUContext, vertex: Vec<Vertex3D>) {
        self.mesh.push(vertex.clone());
        let buf = ctx.create_vertex_buffer(vertex.bytes());
        self.buffer.push(buf);
        self.count_vertex.push(vertex.len());
    }

    pub fn sub_mesh(&mut self, index: usize) {
        // ..todo
    }

}



