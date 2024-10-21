use std::any::Any;
use wgpu::*;

#[path="vertex_buffer.rs"]
mod vertex_buffer;
use vertex_buffer::*;

use crate::WebGPUContext;


struct RenderObject {
    properties: Vec<Box<dyn Any>>,
    pipeline: RenderPipeline,
    mesh: Vec<Vertex3D>,
    buffer: Vec<Buffer>,
}


impl RenderObject {
    fn new(ctx: &WebGPUContext) {
        
    }
}


