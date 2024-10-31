

trait Shader {
    fn index(&self) -> usize;
}

pub struct ComponentShader {
    pub shader: usize
}

