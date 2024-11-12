use std::{borrow::Borrow, cell::{Ref, RefMut}};
use wgpu::{BindGroupLayout, PrimitiveTopology, RenderPipeline, TextureFormat};
use wgpu::{BindGroupLayoutDescriptor, BindGroupLayoutEntry, PipelineLayout, PipelineLayoutDescriptor, PrimitiveState, RenderPipelineDescriptor, VertexState};

use crate::get_unique_id;

use super::{bind_group, ComponentPipelineLayout, ComponentShader, Entity, GameResource, Id, Mesh3D, WebGPUType};

pub trait TraitRenderPipeline {
    fn add_render_pipeline(&mut self);
}

impl TraitRenderPipeline for Entity<'_> {
    fn add_render_pipeline(&mut self) {
        let component = ComponentRenderPipeline3DMesh::new(self);
        self.add_component(component);
    }
}

pub struct ComponentRenderPipeline3DMesh {
    pub render_pipeline_id: Id
}

impl ComponentRenderPipeline3DMesh {
    fn new(entity: &Entity) -> ComponentRenderPipeline3DMesh {

        let shader = entity.get_component::<ComponentShader>().unwrap();
        let pipeline_layout = entity.get_component::<ComponentPipelineLayout>().unwrap();
        let vertex_buffer_layout = entity.get_component::<Mesh3D>().unwrap().vertex.layout();

        let mut res = entity.game_resource.borrow_mut();

        let pipeline = res.ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            //layout: Some(&res.pipeline_layout[&pipeline_layout.pipeline_layout_id]),
            vertex: wgpu::VertexState {
                module: &res.shader[&shader.id],
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[vertex_buffer_layout],
            },

            fragment: Some(wgpu::FragmentState {
                module: &res.shader[&shader.id],
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            operation: wgpu::BlendOperation::Add,
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        },
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),

            primitive: wgpu::PrimitiveState {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                topology: PrimitiveTopology::TriangleList,
                ..Default::default()
            },

            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let id = get_unique_id::<RenderPipeline>(entity);
        res.render_pipeline.insert(id, pipeline);

        Self {
            render_pipeline_id: id
        }

    }
}



