use std::borrow::Borrow;

use wgpu::{PipelineLayoutDescriptor, PrimitiveState, RenderPipelineDescriptor, VertexState};

use super::{ComponentShader, Entity};

struct DefaultRenderPipeline {
    render_pipeline: usize
}

impl DefaultRenderPipeline {
    fn new(entity: &Entity) {

        let shader = entity.get_component::<ComponentShader>().unwrap();
        let pipeline_layout = entity.get_component::<DefaultPipelineLayout>().unwrap();


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

struct DefaultPipelineLayout {
    pipeline_layout: usize
}