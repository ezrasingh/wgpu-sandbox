use crate::gpu::{GpuApplication, GpuContext};
use encase::ShaderType;
use std::{future::Future, sync::Arc};
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    window::Window,
};

const ZOOM_INCREMENT_FACTOR: f32 = 1.1;
const CAMERA_POS_INCREMENT_FACTOR: f32 = 0.1;

#[derive(ShaderType, Clone, Copy, Debug)]
pub struct AppState {
    pub cursor_pos: glam::Vec2,
    pub zoom: f32,
    pub max_iterations: u32,
}

impl AppState {
    // Here though, we use the encase crate which makes translating potentially
    // complex Rust structs easy through combined use of the [`ShaderType`] trait
    // / derive macro and the buffer structs which hold data formatted for WGSL
    // in either the storage or uniform spaces.
    fn as_wgsl_bytes(&self) -> encase::internal::Result<Vec<u8>> {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(self)?;
        Ok(buffer.into_inner())
    }

    fn translate_view(&mut self, increments: i32, axis: usize) {
        self.cursor_pos[axis] += CAMERA_POS_INCREMENT_FACTOR * increments as f32 / self.zoom;
    }

    fn zoom(&mut self, amount: f32) {
        self.zoom += ZOOM_INCREMENT_FACTOR * amount * self.zoom.powf(1.02);
        self.zoom = self.zoom.max(1.1);
    }
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

impl GpuApplication for AppState {
    fn render(self, window: Arc<Window>, event_loop: EventLoop<()>) -> impl Future {
        let main_window_id = window.clone().id();
        let mut state = Some(self.clone());
        async move {
            let mut gpu: Option<GpuContext> =
                Some(GpuContext::new::<AppState>(window.clone()).await);
            event_loop
                .run(move |event, target| {
                    match event {
                        Event::LoopExiting => {
                            gpu = None;
                            state = None;
                        }
                        Event::WindowEvent { window_id, event } if window_id == main_window_id => {
                            match event {
                                WindowEvent::CloseRequested => {
                                    target.exit();
                                }
                                WindowEvent::KeyboardInput {
                                    event:
                                        KeyEvent {
                                            logical_key, text, ..
                                        },
                                    ..
                                } => {
                                    let state_mut = state.as_mut().unwrap();

                                    if let Key::Named(key) = logical_key {
                                        match key {
                                            NamedKey::Escape => target.exit(),
                                            NamedKey::ArrowUp => state_mut.translate_view(1, 1),
                                            NamedKey::ArrowDown => state_mut.translate_view(-1, 1),
                                            NamedKey::ArrowLeft => state_mut.translate_view(-1, 0),
                                            NamedKey::ArrowRight => state_mut.translate_view(1, 0),
                                            _ => {}
                                        }
                                    }

                                    if let Some(text) = text {
                                        if text == "u" {
                                            state_mut.max_iterations += 3;
                                        } else if text == "d" {
                                            state_mut.max_iterations -= 3;
                                        }
                                    };

                                    window.request_redraw();
                                }
                                WindowEvent::MouseWheel { delta, .. } => {
                                    let change = match delta {
                                        winit::event::MouseScrollDelta::LineDelta(_, vertical) => {
                                            vertical
                                        }
                                        winit::event::MouseScrollDelta::PixelDelta(pos) => {
                                            pos.y as f32 / 20.0
                                        }
                                    };
                                    let state_mut = state.as_mut().unwrap();
                                    // (7b)
                                    state_mut.zoom(change);
                                    window.request_redraw();
                                }
                                WindowEvent::Resized(new_size) => {
                                    let gpu_mut = gpu.as_mut().unwrap();
                                    gpu_mut.resize(new_size);
                                    window.request_redraw();
                                }
                                WindowEvent::RedrawRequested => {
                                    let gpu_ref = gpu.as_ref().unwrap();
                                    let state_ref = state.as_ref().unwrap();
                                    let frame = gpu_ref.surface.get_current_texture().unwrap();
                                    let view = frame
                                        .texture
                                        .create_view(&wgpu::TextureViewDescriptor::default());

                                    // (8)
                                    gpu_ref.queue.write_buffer(
                                        &gpu_ref.uniform_buffer,
                                        0,
                                        &state_ref.as_wgsl_bytes().expect(
                                            "Error in encase translating AppState \
                    struct to WGSL bytes.",
                                        ),
                                    );
                                    let mut encoder = gpu_ref.device.create_command_encoder(
                                        &wgpu::CommandEncoderDescriptor { label: None },
                                    );
                                    {
                                        let mut render_pass = encoder.begin_render_pass(
                                            &wgpu::RenderPassDescriptor {
                                                label: None,
                                                color_attachments: &[Some(
                                                    wgpu::RenderPassColorAttachment {
                                                        view: &view,
                                                        resolve_target: None,
                                                        ops: wgpu::Operations {
                                                            load: wgpu::LoadOp::Clear(
                                                                wgpu::Color::GREEN,
                                                            ),
                                                            store: wgpu::StoreOp::Store,
                                                        },
                                                    },
                                                )],
                                                depth_stencil_attachment: None,
                                                occlusion_query_set: None,
                                                timestamp_writes: None,
                                            },
                                        );
                                        render_pass.set_pipeline(&gpu_ref.pipeline);
                                        // (9)
                                        render_pass.set_bind_group(0, &gpu_ref.bind_group, &[]);
                                        render_pass.draw(0..3, 0..1);
                                    }
                                    gpu_ref.queue.submit(Some(encoder.finish()));
                                    frame.present();
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                })
                .unwrap();
        }
    }
}
