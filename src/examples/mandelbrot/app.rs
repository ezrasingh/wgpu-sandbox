use crate::gpu::{
    app::GpuApplication,
    context::{GpuContext, GpuShaderConfig},
    instance::GpuInstance,
};
use encase::ShaderType;
use std::sync::Arc;
use winit::{
    event::{KeyEvent, MouseScrollDelta},
    keyboard,
    window::Window,
};

const ZOOM_INCREMENT_FACTOR: f32 = 0.01;
const CAMERA_POS_INCREMENT_FACTOR: f32 = 0.1;

#[derive(ShaderType, Clone, Copy)]
pub struct AppState {
    pub cursor_pos: glam::Vec2,
    pub zoom: f32,
    pub max_iterations: u32,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            cursor_pos: glam::Vec2::ZERO,
            zoom: 1.0,
            max_iterations: 50,
        }
    }
}

impl AppState {
    fn translate_view(&mut self, increments: i32, axis: usize) {
        self.cursor_pos[axis] += CAMERA_POS_INCREMENT_FACTOR * increments as f32 / self.zoom;
    }

    fn zoom(&mut self, amount: f32) {
        self.zoom += ZOOM_INCREMENT_FACTOR * amount * self.zoom.powf(1.02);
        self.zoom = self.zoom.max(1.1);
    }
}

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

    fn on_keyboard(&mut self, input: KeyEvent, window: Arc<Window>, _gpu: &GpuContext) {
        if let keyboard::Key::Named(key) = input.logical_key {
            match key {
                keyboard::NamedKey::ArrowUp => self.translate_view(1, 1),
                keyboard::NamedKey::ArrowDown => self.translate_view(-1, 1),
                keyboard::NamedKey::ArrowLeft => self.translate_view(-1, 0),
                keyboard::NamedKey::ArrowRight => self.translate_view(1, 0),
                _ => {}
            }
        }

        if let Some(text) = input.text {
            if text == "u" {
                self.max_iterations += 3;
            } else if text == "d" {
                self.max_iterations -= 3;
            }
        };
        window.request_redraw();
    }

    fn on_scroll(&mut self, delta: MouseScrollDelta, window: Arc<Window>, _gpu: &GpuContext) {
        let change = match delta {
            MouseScrollDelta::LineDelta(_, vertical) => vertical,
            MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 20.0,
        };
        self.zoom(change);
        window.request_redraw();
    }
}
