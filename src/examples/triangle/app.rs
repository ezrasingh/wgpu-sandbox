use std::ops::Div;

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
            color: glam::Vec4::ZERO,
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

    fn on_cursor_move(
        &mut self,
        position: winit::dpi::PhysicalPosition<f64>,
        window: std::sync::Arc<winit::window::Window>,
        _gpu: &GpuContext,
    ) {
        let size = window.inner_size();
        leptos::logging::log!("{:?}", position);
        leptos::logging::log!("{:?}", size);
        self.color.x = position.x.div(f64::from(size.width.max(1))) as f32;
        self.color.y = position.y.div(f64::from(size.height.max(1))) as f32;
        leptos::logging::log!("{:?}", self.color);
    }
}
