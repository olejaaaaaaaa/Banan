use std::{cell::RefCell, collections::HashMap, iter, rc::Rc};
use wgpu::{Color, CommandEncoderDescriptor};
use super::{ComponentRenderPipeline3DMesh, Entity, GameResource, GameWorld, Mesh3D};

pub trait RenderSystem {
    fn draw(&self, entity: Vec<&Entity>);
}

impl<'s> RenderSystem for GameWorld<'s> {
    fn draw(&self, entity: Vec<&Entity>) {

        let res = &self.resource.borrow();

        if !res.ctx.resized.get() { return; }

        let mut encoder = res.ctx.device.create_command_encoder(&CommandEncoderDescriptor { label: Some("Default Command Encoder") });
        let mut output =  res.ctx.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Default Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color { r: 0.0, g: 0.2, b: 0.1, a: 1.0 }),
                        store: wgpu::StoreOp::Discard,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            for i in entity {
                let pipeline = i.get_component::<ComponentRenderPipeline3DMesh>().unwrap();
                let mesh = i.get_component::<Mesh3D>().unwrap();

                rpass.set_pipeline(&res.render_pipeline[&pipeline.render_pipeline_id]);
                rpass.set_vertex_buffer(0, res.vertex_buffer[&mesh.vertex_buffer_id].slice(..));
                rpass.draw(0..res.vertex_count[&mesh.vertex_count_id] as u32, 0..1);
            }

        }

        res.ctx.queue.submit(iter::once(encoder.finish()));
        output.present();
    }
}