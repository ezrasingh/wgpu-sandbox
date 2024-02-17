use crate::gpu::context::setup_device;
use std::sync::Arc;
use winit::window::Window;

pub struct GpuInstance {
    pub surface: wgpu::Surface<'static>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GpuInstance {
    pub async fn new(window: Arc<Window>, instance: wgpu::Instance) -> GpuInstance {
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let size = window.inner_size();
        let surface_config = surface
            .get_default_config(&adapter, size.width | 1, size.height | 1)
            .unwrap();
        let (device, queue) = setup_device(&adapter).await;
        surface.configure(&device, &surface_config);
        GpuInstance {
            adapter,
            surface,
            surface_config,
            device,
            queue,
        }
    }
}
