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
use banan::*;

#[wasm_bindgen]
pub fn main() {
    pollster::block_on(run());
}

pub async fn run() {

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn);

    let main_loop = winit::event_loop::EventLoop::new().unwrap();
    let mut window = winit::window::WindowBuilder::new().build(&main_loop).unwrap();
    window.request_inner_size(PhysicalSize::new(640, 640));
    
    debug_play();

    let mut ctx = WebGPUContext::new(&window).await;
    let mut ui = UI::new(ctx.clone());

    let mut rb = RenderObject::default(ctx.clone(), PrimitiveTopology::LineStrip);

    /*
        rb.set_shader()
        rb.add_uniform()
        rb.add_indices()
        rb.update_uniform()
    */

    rb.add_mesh(create_circle_2D(0.3, 0.0, 0.0));
    rb.add_mesh(create_rectangle_2D(0.3, 0.6, 0.0, 0.0));

    let mut all_obj = vec![rb];

    main_loop.run(move |event, event_loop_window_target| {

        match event {

            winit::event::Event::AboutToWait => {
                ctx.window.request_redraw();
            }

            winit::event::Event::WindowEvent { window_id, event } => {

                ui.input_update(event.clone());

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