#![allow(warnings)]
mod utils;

use dpi::PhysicalSize;
use event::WindowEvent;
use wasm_bindgen::prelude::*;
use web_sys::*;
use banan::*;
use winit::*;
use pollster;
use wgpu::*;
use log::log;
use crate::js_sys::Date;

extern crate console_log;
extern crate console_error_panic_hook;

extern crate log;
use log::{debug, error, info, warn};

use banan::vertex_buffer::*;

#[wasm_bindgen]
pub fn main() {
    pollster::block_on(run());
}

const test: &str = include_str!("shaders/test.wgsl");

pub async fn run() {

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Info);

    let main_loop = winit::event_loop::EventLoop::new().unwrap();
    let mut window = winit::window::WindowBuilder::new().with_inner_size(PhysicalSize::new(640, 640)).build(&main_loop).unwrap();
    window.request_inner_size(PhysicalSize::new(640, 640));
    
    let mut ctx = WebGPUContext::new(&window).await;
    let shader = ctx.create_shader(test);

    let layout = ctx.create_pipeline_layout(vec![]);

    let mut c: Vec<Vertex3D> = vec![
        Vertex3D{pos: [ 0.0,  0.5, 0.0 ], color: [1.0, 0.0, 0.0]},
        Vertex3D{pos: [-0.5, -0.5, 0.0 ], color: [0.0, 1.0, 0.0]},
        Vertex3D{pos: [ 0.5, -0.5, 0.0 ], color: [0.0, 0.0, 1.0]},
        Vertex3D{pos: [ 0.0,  0.5, 0.0 ], color: [1.0, 0.0, 1.0]},
    ];

    let b = ctx.create_vertex_buffer(c.content()); 
    let pipeline = ctx.create_pipeline(PrimitiveTopology::LineStrip, layout, shader, c.layout());
    

    main_loop.run(move |event, event_loop_window_target| {

        match event {

            winit::event::Event::AboutToWait => {
                ctx.window.request_redraw();
            }

            winit::event::Event::WindowEvent { window_id, event } => {
                match event {

                    WindowEvent::Destroyed => {
                        
                    }

                    WindowEvent::RedrawRequested => {

                        for i in &mut c {
                            i.color[0] -= 0.001; i.pos[0] += (Date::new_0().get_utc_seconds() as f32).cos() / 100.0;
                            i.color[1] -= 0.001; i.pos[1] += (Date::new_0().get_utc_seconds() as f32).sin() / 100.0;
                            i.color[2] -= 0.001;
                        }

                        ctx.update_buffer(&b, c.content());
                        ctx.draw(&pipeline, &b);
                        
                    }

                    WindowEvent::Resized(size) => {
                        ctx.resize(size);
                    }

                    _ => ()
                }
            }

            _ => ()
        }

    });


}