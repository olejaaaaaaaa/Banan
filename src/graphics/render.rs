use std::{collections::HashMap, iter};

use wgpu::{Color, CommandEncoderDescriptor};

use super::{context::resized, Mesh3D, Entity, GameWorld};

pub trait RenderSystem {
    fn draw(&self, entity: Vec<&Entity>);
}
pub struct DefaultRenderSystem;
impl DefaultRenderSystem {
    pub fn new() -> Self {
        Self
    }
}


///
/// Universal Draw for any entity
/// coming soon
///
impl RenderSystem for DefaultRenderSystem {
    fn draw(&self, entity: Vec<&Entity>) {
        // draw entity
    }
}

struct DefaultRender3DMesh;
impl RenderSystem for DefaultRender3DMesh {
    fn draw(&self, entity: Vec<&Entity>) {

        if unsafe { !resized } { return; }

        let mut encoder =   entity[0].game_resource.borrow().ctx.device.create_command_encoder(&CommandEncoderDescriptor { label: None });
        let mut output = entity[0].game_resource.borrow().ctx.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color { r: 0.0, g: 0.2, b: 0.0, a: 1.0 }),
                        store: wgpu::StoreOp::Discard,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

        }

        entity[0].game_resource.borrow().ctx.queue.submit(iter::once(encoder.finish()));
        output.present();
    }
}