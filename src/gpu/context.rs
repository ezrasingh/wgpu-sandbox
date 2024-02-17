use crate::gpu::helpers::GpuInstance;

pub struct GpuContext {
    pub surface: wgpu::Surface<'static>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
}

pub struct GpuShaderConfig<'a> {
    pub src: &'static str,
    pub vertex_entry: &'a str,
    pub fragment_entry: &'a str,
}

impl GpuContext {
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn new<S>(instance: GpuInstance, shader_config: GpuShaderConfig) -> GpuContext {
        let shader = instance
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(shader_config.src)),
            });
        let uniform_buffer = instance.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: std::mem::size_of::<S>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let (bind_group, bind_group_layout) = setup_bind_group(
            &instance.device,
            wgpu::BufferBindingType::Uniform,
            &uniform_buffer,
        );
        let pipeline = setup_pipeline(
            &instance.device,
            &bind_group_layout,
            &shader,
            (shader_config.vertex_entry, shader_config.fragment_entry),
            instance.surface.get_capabilities(&instance.adapter),
        );
        GpuContext {
            surface: instance.surface,
            surface_config: instance.surface_config,
            device: instance.device,
            queue: instance.queue,
            pipeline,
            bind_group,
            uniform_buffer,
        }
    }
}

pub async fn setup_device(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
            },
            None,
        )
        .await
        .unwrap()
}

pub fn setup_bind_group(
    device: &wgpu::Device,
    buffer_type: wgpu::BufferBindingType,
    buffer: &wgpu::Buffer,
) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: buffer_type,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer,
                offset: 0,
                size: None,
            }),
        }],
    });
    (bind_group, bind_group_layout)
}

pub fn setup_pipeline(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
    shader: &wgpu::ShaderModule,
    shader_entrypoints: (&str, &str),
    swapchain_capabilities: wgpu::SurfaceCapabilities,
) -> wgpu::RenderPipeline {
    let swapchain_format = swapchain_capabilities.formats[0];
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: shader_entrypoints.0,
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: shader_entrypoints.1,
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    })
}
