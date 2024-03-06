use crate::gpu::{
    app::GpuApplication,
    context::{GpuContext, GpuShaderConfig},
    instance::GpuInstance,
};
use encase::ShaderType;

#[derive(ShaderType, Clone, Copy)]
pub struct AppState {
    pub color: glam::Vec4,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            color: glam::Vec4::new(0.0, 1.0, 0.0, 1.0),
        }
    }
}

impl AppState {}

impl GpuApplication for AppState {
    fn build(self, resources: GpuInstance) -> GpuContext {
        GpuContext::new::<AppState>(
            resources,
            GpuShaderConfig {
                src: include_str!("shader.wgsl"),
                vertex_entry: "vs_main",
                fragment_entry: "fs_main",
            },
        )
    }
}
