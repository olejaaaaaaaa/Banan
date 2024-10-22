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
use banan::core::*;

#[wasm_bindgen]
pub fn main() {
    pollster::block_on(run());
}

pub async fn run() {

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Info);

    let main_loop = winit::event_loop::EventLoop::new().unwrap();
    let mut window = winit::window::WindowBuilder::new().build(&main_loop).unwrap();
    window.request_inner_size(PhysicalSize::new(640, 640));
    
    let mut ctx = WebGPUContext::new(&window).await;
    let mut rb = RenderObject::default(&ctx, PrimitiveTopology::LineStrip);

    rb.add_mesh(&ctx, vec![
        Vertex3D{ pos: [ 0.0,   0.5,  0.0],   color: [1.0, 0.0, 0.0] },
        Vertex3D{ pos: [-0.5,  -0.5,  0.0],   color: [0.0, 1.0, 0.0] },
        Vertex3D{ pos: [ 0.5,  -0.5,  0.0],   color: [0.0, 0.0, 1.0] },
        Vertex3D{ pos: [ 0.0,   0.5,  0.0],   color: [1.0, 0.0, 0.0] }
    ]);

    let all_obj = vec![rb];

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
                        ctx.draw_debug(&all_obj);                
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