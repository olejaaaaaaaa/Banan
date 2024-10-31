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


pub struct WebGPUContext<'s> {
    pub window: &'s Window,
    pub surface: Surface<'s>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub instance: wgpu::Instance,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface_caps: wgpu::SurfaceCapabilities,
    pub surface_format: wgpu::TextureFormat,
    pub queue: wgpu::Queue,
}

pub static mut resized: bool = false;

///
///
/// All
impl<'s> WebGPUContext<'s> {

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

}

pub struct WebGPUContextBuilder<'s> {
    pub window: &'s Window,
    pub surface: Surface<'s>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub instance: wgpu::Instance,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface_caps: wgpu::SurfaceCapabilities,
    pub surface_format: wgpu::TextureFormat,
    pub queue: wgpu::Queue,
}


impl<'s> WebGPUContextBuilder<'s> {

    fn create_canvas(window: &Window) {
        use winit::platform::web::WindowExtWebSys;
        let win = web_sys::window().unwrap();
        let doc = win.document().unwrap();
        let body = doc.get_element_by_id("main-body").unwrap();
        let canvas = web_sys::Element::from(window.canvas().unwrap());
        body.append_child(&canvas);
    }

    pub async fn new(window: &'s Window) -> Self {

        Self::create_canvas(window);
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

        Self {
            window,
            adapter,
            device,
            instance,
            surface_config,
            surface_caps,
            surface_format,
            queue,
            surface,
        }

    }


    pub fn required_limits(mut self) {

    }

    pub fn with_webgl_limits(mut self) {
        // for webgl
        // let limits = wgpu::Limits {
        //     max_compute_workgroups_per_dimension: 0,
        //     max_compute_workgroup_size_z: 0,
        //     max_compute_workgroup_size_y: 0,
        //     max_compute_workgroup_size_x: 0,
        //     max_compute_invocations_per_workgroup: 0,
        //     max_compute_workgroup_storage_size: 0,
        //     max_storage_buffer_binding_size: 0,
        //     max_storage_textures_per_shader_stage: 0,
        //     max_storage_buffers_per_shader_stage: 0,
        //     max_dynamic_storage_buffers_per_pipeline_layout: 0,
        //     ..Default::default()
        // };
    }

    pub fn required_features(mut self) {

    }

    pub fn memory_hints(mut self) {

    }

    pub fn power_preference(mut self) {

    }

    pub fn build(self) -> WebGPUContext<'s> {
        WebGPUContext {
            window:         &self.window,
            surface:        self.surface,
            adapter:        self.adapter,
            device:         self.device,
            instance:       self.instance,
            surface_config: self.surface_config,
            surface_caps:   self.surface_caps,
            surface_format: self.surface_format,
            queue:          self.queue
        }
    }


}
