extern crate vulkano_shaders;

fn main() {
    // building the shaders used in the examples
    vulkano_shaders::build_glsl_shaders([
        ("src/compute.glsl", vulkano_shaders::ShaderType::Compute),
        ("src/triangle_vs.glsl", vulkano_shaders::ShaderType::Vertex),
        ("src/triangle_fs.glsl", vulkano_shaders::ShaderType::Fragment)
    ].iter().cloned());
}
