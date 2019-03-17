use std::fs::read_to_string;
use std::io::Read;

#[allow(dead_code)]
pub enum ShaderStage {
  Vertex,
  Fragment,
  Compute,
}

pub fn load_glsl(path: &str, stage: ShaderStage) -> Vec<u8> {
  let shader_stage = match stage {
    ShaderStage::Vertex => glsl_to_spirv::ShaderType::Vertex,
    ShaderStage::Fragment => glsl_to_spirv::ShaderType::Fragment,
    ShaderStage::Compute => glsl_to_spirv::ShaderType::Compute,
  };

  let code = read_to_string(&path)
    .unwrap_or_else(|e| panic!("Unable to read {:?}: {:?}", path, e));

  let mut output = glsl_to_spirv::compile(&code, shader_stage).expect("Shader compile error");
  let mut spv = vec![];
  output.read_to_end(&mut spv).expect("Shader read error");
  spv
}
