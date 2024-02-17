use crate::gpu::{context::GpuContext, helpers::GpuInstance};
use std::borrow::{Borrow, BorrowMut};
use std::future::Future;
use std::sync::Arc;
use web_sys::{HtmlCanvasElement, HtmlDivElement};
use winit::{
    event::{Event, KeyEvent, MouseScrollDelta, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

pub trait GpuApplication {
    fn as_wgsl_bytes(&self) -> encase::internal::Result<Vec<u8>>
    where
        Self: encase::ShaderType + encase::internal::WriteInto,
    {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(self)?;
        Ok(buffer.into_inner())
    }

    fn build(self, instance: GpuInstance) -> GpuContext;

    fn on_close(
        &mut self,
        target: &EventLoopWindowTarget<()>,
        _window: Arc<Window>,
        _gpu: &GpuContext,
    ) where
        Self: Sized,
    {
        target.exit();
    }

    fn on_resize(
        &mut self,
        new_size: winit::dpi::PhysicalSize<u32>,
        window: Arc<Window>,
        gpu: &mut GpuContext,
    ) where
        Self: Sized,
        Self: encase::internal::WriteInto,
    {
        gpu.resize(new_size);
        window.request_redraw();
    }

    fn on_redraw(&mut self, _window: Arc<Window>, gpu: &GpuContext)
    where
        Self: Sized,
        Self: encase::ShaderType,
        Self: encase::internal::WriteInto,
    {
        let frame = gpu.surface.get_current_texture().unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        gpu.queue.write_buffer(
            &gpu.uniform_buffer,
            0,
            &self
                .as_wgsl_bytes()
                .expect("Error in encase translating AppState struct to WGSL bytes."),
        );

        let mut encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&gpu.pipeline);
            render_pass.set_bind_group(0, &gpu.bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }
        gpu.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    fn on_keyboard(&mut self, _input: KeyEvent, _window: Arc<Window>, _gpu: &GpuContext) {}

    fn on_scroll(&mut self, _delta: MouseScrollDelta, _window: Arc<Window>, _gpu: &GpuContext)
    where
        Self: Sized,
    {
    }

    fn render(&mut self, window: Arc<Window>, event_loop: EventLoop<()>) -> impl Future
    where
        Self: Copy,
        Self: Sized,
        Self: encase::ShaderType,
        Self: encase::internal::WriteInto,
    {
        async {
            let instance = GpuInstance::new(window.clone(), wgpu::Instance::default()).await;
            let mut gpu = self.build(instance);

            event_loop
                .run(move |event, target| match event {
                    Event::WindowEvent { window_id, event } if window_id == window.clone().id() => {
                        match event {
                            WindowEvent::CloseRequested => {
                                self.on_close(target, window.to_owned(), gpu.borrow())
                            }
                            WindowEvent::KeyboardInput { event, .. } => {
                                self.on_keyboard(event, window.to_owned(), gpu.borrow())
                            }
                            WindowEvent::MouseWheel { delta, .. } => {
                                self.on_scroll(delta, window.to_owned(), gpu.borrow())
                            }
                            WindowEvent::Resized(new_size) => {
                                self.on_resize(new_size, window.to_owned(), gpu.borrow_mut())
                            }
                            WindowEvent::RedrawRequested => {
                                self.on_redraw(window.to_owned(), gpu.borrow())
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                })
                .unwrap();
        }
    }

    fn setup(&self, container: HtmlDivElement) -> (Window, EventLoop<()>)
    where
        Self: Sized,
    {
        use winit::platform::web::WindowExtWebSys;
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let canvas: HtmlCanvasElement = window.canvas().unwrap();

        canvas
            .style()
            .set_css_text("display: block; width: 100%; height: 100%");

        let _ = container.append_child(&canvas);

        (window, event_loop)
    }
}
