


#![allow(warnings)]

use crate::Vertex3D;
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn create_circle_2D(r: f32, x: f32, y: f32) -> Vec<Vertex3D> {
    let mut v: Vec<Vertex3D> = vec![];
    let angle = (3.14/180.0) * 5.0;
    for i in 0..360 {
        v.push(Vertex3D { pos: [(angle * i as f32).cos() * r, (angle * i as f32).sin() * r, 0.0], color: [1.0, 0.0, 0.0] });
    }

    v
}

pub fn create_rectangle_2D(width: f32, heigth: f32, x: f32, y: f32) -> Vec<Vertex3D> {
    let mut v: Vec<Vertex3D> = vec![];
    
    v.push(
        Vertex3D { pos: [x, y, 0.0], color: [0.0, 0.0, 1.0] },
    );

    v.push(
        Vertex3D { pos: [x, y + heigth, 0.0], color: [0.0, 0.0, 1.0] },
    );

    v.push(
        Vertex3D { pos: [x + width, y + heigth, 0.0], color: [0.0, 0.0, 1.0] },
    );

    v.push(
        Vertex3D { pos: [x + width, y, 0.0], color: [0.0, 0.0, 1.0] },
    );

    v.push(
        Vertex3D { pos: [x, y, 0.0], color: [0.0, 0.0, 1.0] },
    );

    v
}