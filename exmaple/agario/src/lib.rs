#![allow(warnings)]
mod utils;

use dpi::PhysicalSize;
use event::WindowEvent;
use wasm_bindgen::prelude::*;
use web_sys::*;
use banan::*;
use winit::*;
use wgpu::*;
use log::log;
use crate::js_sys::Date;

extern crate console_log;
extern crate console_error_panic_hook;

extern crate log;
use log::{debug, error, info, warn};
use wasm_bindgen_futures::spawn_local;

use banan::Entity;

#[wasm_bindgen]
pub fn main() {
    spawn_local(run());
}

pub async fn run() {

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn);

    let main_loop = winit::event_loop::EventLoop::new().unwrap();
    let mut window = winit::window::WindowBuilder::new().build(&main_loop).unwrap();
    window.request_inner_size(PhysicalSize::new(640, 640));

    debug_play();

    let ctx = WebGPUContextBuilder::new(&window).await.build().await;
    let mut world = GameWorld::new(ctx, &window).await;

    let mut player = world.create_entity();

    player.add_mesh(
    vec![
        Vertex3D{ pos: [ 0.0,   0.5,  0.0], color: [1.0, 0.0, 0.0] },
        Vertex3D{ pos: [-0.5,  -0.5,  0.0], color: [0.0, 1.0, 0.0] },
        Vertex3D{ pos: [ 0.5,  -0.5,  0.0], color: [0.0, 0.0, 1.0] }
    ], PrimitiveTopology::TriangleList);

    player.add_shader("test.wgsl");
    player.add_bind_group_layout();
    player.add_pipeline_layout();
    player.add_render_pipeline();

    let mut player2 = world.create_entity();

    player2.add_shader("test.wgsl");
    player2.add_mesh(
    vec![
        Vertex3D{ pos: [ 0.0,   0.2,  0.0], color: [0.0, 0.0, 0.0] },
        Vertex3D{ pos: [-0.2,  -0.2,  0.0], color: [0.0, 0.0, 0.0] },
        Vertex3D{ pos: [ 0.2,  -0.2,  0.0], color: [0.0, 0.0, 0.0] },
        Vertex3D{ pos: [ 0.0,   0.2,  0.0], color: [0.0, 0.0, 0.0] },
    ], None);

    player2.add_bind_group_layout();
    player2.add_pipeline_layout();
    player2.add_render_pipeline();

    main_loop.run(move |event, event_loop_window_target| {

        match event {

            winit::event::Event::AboutToWait => {

            }

            winit::event::Event::WindowEvent { window_id, event } => {

                match event {

                    WindowEvent::Destroyed => {

                    }

                    WindowEvent::RedrawRequested => {
                        world.draw(vec![&player, &player2]);
                    }

                    WindowEvent::Resized(size) => {
                        world.resize(size);
                    }

                    _ => ()
                }
            }
            _ => ()
        }
    });

}