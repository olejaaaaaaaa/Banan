#![allow(warnings)]
mod utils;

use wasm_bindgen::prelude::*;

pub mod context;
pub use context::WebGPUContext;

pub mod vertex_buffer;
pub use vertex_buffer::*;

pub mod uniform_buffer;
pub use uniform_buffer::*;

pub mod render_object;
pub use render_object::*;