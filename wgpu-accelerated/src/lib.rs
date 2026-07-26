use drakon_opcodes::*;
use fitness_parameters::*;
use std::{num::NonZeroU64, str::FromStr};
use wgpu::util::DeviceExt;

pub struct WgpuWrapper {
    device: wgpu::Device,
    queue: wgpu::Queue,
    shader_module: wgpu::ShaderModule,
}

impl WgpuWrapper {
    pub fn init(self) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());

        let adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
                .expect("Failed to create adapter on GPU device, with Wgpu");

        let downlevel_capabilities = adapter.get_downlevel_capabilities();
        if !downlevel_capabilities
            .flags
            .contains(wgpu::DownlevelFlags::COMPUTE_SHADERS)
        {
            panic!("Your GPU device Adapter does not support compute shaders");
        }

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_defaults(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: wgpu::MemoryHints::MemoryUsage,
            trace: wgpu::Trace::Off,
        }))
        .expect("Failed to create device");

        let module = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        Self {
            device: device,
            queue: queue,
            shader_module: module,
        }
    }
}

pub fn gpu_raw_search(wgpu_wrapper: WgpuWrapper, training_dataset: TrainingDataset) -> [u8; 8] {
    let input_data_buffer =
        wgpu_wrapper
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: training_dataset.to_bytes().as_slice(),
                usage: wgpu::BufferUsages::STORAGE,
            });

    [0_u8; 8]
}
