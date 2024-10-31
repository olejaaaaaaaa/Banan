use std::{borrow::Borrow, cell::{Ref, RefMut}};
use wgpu::BindGroupLayout;
use wgpu::{BindGroupLayoutDescriptor, BindGroupLayoutEntry, PipelineLayout, PipelineLayoutDescriptor, PrimitiveState, RenderPipelineDescriptor, VertexState};

use crate::get_unique_id;

use super::{bind_group, ComponentShader, Entity, GameResource, Id};

trait TraitRenderPipeline {
    fn add_render_pipeline(&mut self);
}

pub struct ComponentRenderPipeline {
    pub render_pipeline_id: Id
}

impl ComponentRenderPipeline {
    fn new(entity: &Entity) {

        let shader = entity.get_component::<ComponentShader>().unwrap();
        let pipeline_layout = entity.get_component::<ComponentPipelineLayout>().unwrap();

        // let pipeline = entity.game_resource.try_borrow().ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        //     label: None,
        //     layout: entity.,
        //     vertex: wgpu::VertexState {
        //         module: &shader.unwrap(),
        //         entry_point: "vs_main",
        //         compilation_options: Default::default(),
        //         buffers: &[Vertex::layout()],
        //     },

        //     fragment: Some(wgpu::FragmentState {
        //         module: self.shader.as_ref().unwrap(),
        //         entry_point: "fs_main",
        //         compilation_options: Default::default(),
        //         targets: &[Some(wgpu::ColorTargetState {
        //             format: TextureFormat::Rgba8UnormSrgb,
        //             blend: Some(wgpu::BlendState {
        //                 color: wgpu::BlendComponent {
        //                     operation: wgpu::BlendOperation::Add,
        //                     src_factor: wgpu::BlendFactor::SrcAlpha,
        //                     dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
        //                 },
        //                 alpha: wgpu::BlendComponent::REPLACE,
        //             }),
        //             write_mask: wgpu::ColorWrites::ALL,
        //         })],
        //     }),

        //     primitive: wgpu::PrimitiveState {
        //         front_face: wgpu::FrontFace::Ccw,
        //         cull_mode: Some(wgpu::Face::Back),
        //         polygon_mode: wgpu::PolygonMode::Fill,
        //         topology: PrimitiveTopology::PointList,
        //         ..Default::default()
        //     },

        //     depth_stencil: None,
        //     multisample: wgpu::MultisampleState::default(),
        //     multiview: None,
        //     cache: None,
        // });


    }
}

pub struct ComponentPipelineLayout {
    pub pipeline_layout_id: Id
}

impl ComponentPipelineLayout {
    fn new(entity: &Entity, bind_group_layout: &[&BindGroupLayout]) -> Self {
        let mut res: RefMut<GameResource> = entity.game_resource.borrow_mut();

        let pipeline_layout = res.ctx.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: bind_group_layout,
            push_constant_ranges: &[]
        });

        let id = get_unique_id::<PipelineLayout>(entity);
        entity.game_resource.borrow_mut().pipeline_layout.insert(id, pipeline_layout);

        Self {
            pipeline_layout_id: id
        }
    }
}

