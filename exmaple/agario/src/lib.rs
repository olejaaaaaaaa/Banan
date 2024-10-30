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

use hecs::*;

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

    let ctx = WebGPUContextBuilder::new().await.build();
    let mut world = GameWorld::new(ctx, &window).await;
    let player = world.create_entity();

    player.add_component([0.0, 0.0, 0.0]);


    main_loop.run(move |event, event_loop_window_target| {

        match event {

            winit::event::Event::AboutToWait => {

            }

            winit::event::Event::WindowEvent { window_id, event } => {

                match event {

                    WindowEvent::Destroyed => {

                    }

                    WindowEvent::RedrawRequested => {

                    }

                    WindowEvent::Resized(size) => {

                    }

                    _ => ()
                }
            }
            _ => ()
        }
    });

}