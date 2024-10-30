#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex3D {
    pub color: [f32; 3],
    pub pos:   [f32; 3]
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex2D {
    pub color: [f32; 3],
    pub pos:   [f32; 2]
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub time: [f32; 3] // hours, secs, millisecs
}