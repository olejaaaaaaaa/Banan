

trait Shader {
    fn index(&self) -> usize;
}

pub struct DefaultShader {
    pub shader: usize
}

