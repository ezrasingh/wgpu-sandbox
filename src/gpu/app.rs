use crate::gpu::{context::GpuContext, instance::GpuInstance};
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
        gpu.render(
            &self
                .as_wgsl_bytes()
                .expect("Error in encase translating AppState struct to WGSL bytes."),
        )
    }

    fn on_keyboard(&mut self, _input: KeyEvent, _window: Arc<Window>, _gpu: &GpuContext) {}

    fn on_cursor_move(
        &mut self,
        _position: winit::dpi::PhysicalPosition<f64>,
        _window: Arc<Window>,
        _gpu: &GpuContext,
    ) {
    }

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
            let instance = GpuInstance::new(window.clone()).await;
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
                            WindowEvent::CursorMoved { position, .. } => {
                                self.on_cursor_move(position, window.to_owned(), gpu.borrow())
                            }
                            WindowEvent::Resized(new_size) => {
                                self.on_resize(new_size, window.to_owned(), gpu.borrow_mut())
                            }
                            WindowEvent::RedrawRequested => {
                                self.on_redraw(window.to_owned(), gpu.borrow_mut())
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
