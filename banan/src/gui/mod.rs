use std::rc::Rc;

use winit::event::WindowEvent;

use crate::WebGPUContext;

pub struct Button {
    width: usize,
    height: usize,
    x: f32,
    y: f32,
}

pub struct Text {
    size: usize,
    x: f32,
    y: f32,
}

struct Image {

}

struct InputText {

}

pub struct UI<'s> {
    context: Rc<WebGPUContext<'s>>
}

impl<'s> UI<'s> {

    pub fn new(ctx: Rc<WebGPUContext<'s>>) -> Self {
        Self {
            context: ctx
        }
    }

    pub fn input_update(&self, event: WindowEvent) {
        match event {

            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {

            },
            
            _ => ()
        }
    }
}
