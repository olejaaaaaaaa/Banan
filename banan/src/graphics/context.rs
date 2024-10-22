#![allow(warnings)]
#![allow(clippy::all)]
use std::{iter, rc::Rc};

use dpi::PhysicalSize;
use event::Event;
use event_loop::{EventLoop, EventLoopWindowTarget};
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer, BufferBinding, BufferBindingType, BufferUsages, Color, CommandEncoderDescriptor, DeviceDescriptor, Features, Limits, MemoryHints, PipelineLayout, PipelineLayoutDescriptor, PrimitiveTopology, RenderPipeline, RequestAdapterOptions, RequestAdapterOptionsBase, ShaderModule, ShaderModuleDescriptor, ShaderStages, Surface, TextureFormat, VertexBufferLayout};
use window::Window;
use winit::*;
use pollster::*;

#[path ="vertex_buffer.rs"]
mod vertex_buffer;
use vertex_buffer::*;

use crate::RenderObject;

pub struct WebGPUContext<'s> {
    pub window: &'s Window,
    surface: Surface<'s>,
    adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    instance: wgpu::Instance,
    surface_config: wgpu::SurfaceConfiguration,
    surface_caps: wgpu::SurfaceCapabilities,
    surface_format: wgpu::TextureFormat,
    queue: wgpu::Queue,
}

static mut resized: bool = false;

impl<'s> WebGPUContext<'s> {

    pub async fn new(window: &'s Window ) -> Rc<Self> {
        Self::create_canvas(&window);

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window).expect("Error create surface");
        let adapter = instance.request_adapter(&RequestAdapterOptions { 
            power_preference: wgpu::PowerPreference::HighPerformance, 
            force_fallback_adapter: false, 
            compatible_surface: Some(&surface) 
        }).await.expect("Error create adapter");

        let (device, queue) = adapter.request_device(&DeviceDescriptor { 
            label: Some("Main Adapter"), 
            required_features: Features::empty(), 
            required_limits: Limits::downlevel_webgl2_defaults(), 
            memory_hints: MemoryHints::Performance
        }, None).await.expect("Error create device or queue");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Rc::new(Self { 
            window,
            adapter,
            device,
            instance,
            surface_config,
            surface_caps,
            surface_format,
            queue,
            surface,
        })
    }

   fn create_canvas(window: &Window) {
        use winit::platform::web::WindowExtWebSys;
        let win = web_sys::window().unwrap();
        let doc = win.document().unwrap();
        let body = doc.get_element_by_id("main-body").unwrap();
        let canvas = web_sys::Element::from(window.canvas().unwrap());
        body.append_child(&canvas);       
    }

    pub fn create_shader(&self, source: &str) -> ShaderModule {
        self.device.create_shader_module(ShaderModuleDescriptor {
            label: Some(source),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(source))
        })
    }

    pub fn create_pipeline_layout(&self, bind_group_layout: Vec<&BindGroupLayout>) -> PipelineLayout {
        let pipeline_layout = self.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &bind_group_layout,
            push_constant_ranges: &[],
        });

        pipeline_layout
    }

    pub fn create_vertex_buffer(&self, content: &[u8]) -> wgpu::Buffer{

        let buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some(format!("Vertex Buffer with size: {}", content.len()).as_str()),
            contents: content,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        buffer
    }

    pub fn laod_texture(&self) {

    }

    pub fn create_bind_group(&self, bind_group_layout: &BindGroupLayout, buffer: &Buffer) {
        self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(BufferBinding{
                    buffer,
                    offset: 16,
                    size: None,
                }),
            }],
        });
    }

    pub fn create_bindg_group_layout(&self) -> BindGroupLayout {
        let bind = self.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer { 
                    ty: BufferBindingType::Uniform, 
                    has_dynamic_offset: false, 
                    min_binding_size: None,
                },
                count: None,
            }]
        });

        bind
    }

    pub fn create_uniform_buffer(&self, content: &[u8]) -> wgpu::Buffer {

        let buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some(format!("Uniform Buffer with size: {}", content.len()).as_str()),
            contents: content,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        buffer
    }

    pub fn create_pipeline(&self, topology: PrimitiveTopology, pipeline_layout: PipelineLayout, shader: ShaderModule, vertex_layout: &VertexBufferLayout) -> RenderPipeline{

        let pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[vertex_layout.clone()],
            },

            fragment: Some(wgpu::FragmentState {
                module: &shader,
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
                topology,
                ..Default::default()
            },

            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        pipeline
    }

    pub fn resize(&self, size: PhysicalSize<u32>) {

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            width: size.width.min(640), 
            height: size.height.min(640), 
            present_mode: self.surface_caps.present_modes[0],
            alpha_mode: self.surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        self.surface.configure(&self.device, &surface_config);
        unsafe { resized = true };
    }

    pub fn update_buffer(&self, buffer: &Buffer, content: &[u8]) {
        self.queue.write_buffer(buffer, 0, content);
    }

    pub fn draw(&mut self, pipeline: &RenderPipeline, vertex: &Buffer) {

        if !unsafe { resized } { return; }

        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { label: Some("Main Encoder") });
        let mut output = self.surface.get_current_texture().unwrap();

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }),
                        store: wgpu::StoreOp::Discard,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            rpass.set_pipeline(pipeline);
            rpass.set_vertex_buffer(0, vertex.slice(..));
            rpass.draw(0..3, 0..1);
        }    
      
        self.queue.submit(iter::once(encoder.finish()));
        output.present();

    }

    pub fn draw_debug(&self, obj: &Vec<RenderObject>) {
        if !unsafe { resized } { return; }

        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { label: Some("Main Encoder") });
        let mut output = self.surface.get_current_texture().unwrap();

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }),
                        store: wgpu::StoreOp::Discard,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            for i in obj {
                rpass.set_pipeline(&i.render);
                for v in 0..i.buffer.len() {
                    rpass.set_vertex_buffer(0, i.buffer[v].slice(..));
                    rpass.draw(0..i.count_vertex[v] as u32, 0..1);
                }
            }
        }    
      
        self.queue.submit(iter::once(encoder.finish()));
        output.present();
    }

}




