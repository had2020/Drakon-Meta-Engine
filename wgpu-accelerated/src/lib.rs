use drakon_opcodes::*;
use fitness_parameters::*;
use std::num::NonZeroU64;
use wgpu::util::DeviceExt;

pub struct WgpuWrapper {
    device: wgpu::Device,
    queue: wgpu::Queue,
    module: wgpu::ShaderModule,
}

impl WgpuWrapper {
    pub fn init() -> Self {
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
            module: module,
        }
    }
}

pub fn gpu_raw_search<const TEST_SETS: usize>(
    wgpu_wrapper: &WgpuWrapper,
    training_dataset: &Requirements<TEST_SETS>,
) -> [u8; 8] {
    let dataset_bytes = training_dataset.to_bytes_form();

    let input_data_buffer =
        wgpu_wrapper
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Training Dataset Buffer"),
                contents: dataset_bytes.as_slice(),
                usage: wgpu::BufferUsages::STORAGE,
            });

    let output_data_buffer = wgpu_wrapper.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: input_data_buffer.size(),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let download_buffer = wgpu_wrapper.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: input_data_buffer.size(),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let bind_group_layout =
        wgpu_wrapper
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    // input buffer
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            // the size of a single element in the buffer
                            min_binding_size: Some(NonZeroU64::new(4).unwrap()),
                            has_dynamic_offset: false,
                        },
                        count: None,
                    },
                    // output buffer
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            min_binding_size: Some(NonZeroU64::new(4).unwrap()),
                            has_dynamic_offset: false,
                        },
                        count: None,
                    },
                ],
            });

    let bind_group = wgpu_wrapper
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: input_data_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: output_data_buffer.as_entire_binding(),
                },
            ],
        });

    let pipeline_layout =
        wgpu_wrapper
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[Some(&bind_group_layout)],
                immediate_size: 0,
            });

    let pipeline = wgpu_wrapper
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            module: &wgpu_wrapper.module,
            entry_point: Some("MetaDrakon"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });

    let mut encoder = wgpu_wrapper
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
        label: None,
        timestamp_writes: None,
    });

    compute_pass.set_pipeline(&pipeline);
    compute_pass.set_bind_group(0, &bind_group, &[]);

    //let workgroup_count: usize = 65535;
    //compute_pass.dispatch_workgroups(workgroup_count as u32, 1, 1);

    // Each workgroup will take the max of 65536 invocations,
    // this leaves a range of 256 required attempts, for the search space of 8^8.
    compute_pass.dispatch_workgroups(256, 256, 1);

    drop(compute_pass);

    encoder.copy_buffer_to_buffer(
        &output_data_buffer,
        0,
        &download_buffer,
        0,
        output_data_buffer.size(),
    );

    let command_buffer = encoder.finish();

    wgpu_wrapper.queue.submit([command_buffer]);

    let buffer_slice = download_buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, |_| {});

    wgpu_wrapper
        .device
        .poll(wgpu::PollType::wait_indefinitely())
        .unwrap();

    let data = buffer_slice.get_mapped_range().unwrap();

    let result: Vec<u8> = bytemuck::allocation::pod_collect_to_vec(&data);

    let program: [u8; 8] = [
        result[0], result[1], result[2], result[3], result[4], result[5], result[6], result[7],
    ];

    program // <- TODO:
}
