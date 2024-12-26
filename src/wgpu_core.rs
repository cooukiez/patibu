use std::time::{Duration, Instant};
use pollster::block_on;
use wgpu::{Adapter, Backends, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, InstanceFlags, Limits, PowerPreference, Queue, RequestAdapterOptions, Surface};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct FrameInfo {
    last_frame: Instant,
    
    // since last frame
    frame_count: u32,
    accumulated_time: f32,

    pub fps: f32,
    pub frame_time: f32,
}

impl FrameInfo {
    pub fn fetch(&mut self) -> Duration {
        let delta_time = self.last_frame.elapsed();
        let delta_seconds = delta_time.as_secs_f32();
        self.last_frame = Instant::now();

        self.accumulated_time += delta_seconds;
        
        if self.accumulated_time >= 1.0 {
            self.fps = self.frame_count as f32 / self.accumulated_time;
            self.frame_time = (self.accumulated_time * 1000.0) / self.frame_count as f32;
            
            self.accumulated_time = 0.0;
            self.frame_count = 0;
        }
        
        self.frame_count += 1;

        delta_time
    }
}

impl Default for FrameInfo {
    fn default() -> Self {
        Self {
            last_frame: Instant::now(),
            
            frame_count: 0,
            accumulated_time: 0.0,

            fps: 0.0,
            frame_time: 0.0,
        }
    }
}

pub async fn init_wgpu(window: &Window) -> (PhysicalSize<u32>, Instance, Surface, f64, Adapter, Device, Queue) {
    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);

    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::VULKAN,
        flags: InstanceFlags::empty(),
        ..Default::default()
    });

    let surf = instance.create_surface(window).unwrap();

    let hidpi_factor = window.scale_factor();

    let adapter = block_on(instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        compatible_surface: Some(&surf),
        force_fallback_adapter: false,
    })).unwrap();

    let (dev, queue) = adapter.request_device(
        &DeviceDescriptor {
            label: None,
            required_features: Features::SPIRV_SHADER_PASSTHROUGH,
            required_limits: Limits {
                max_storage_buffers_per_shader_stage: 1,

                ..Limits::downlevel_webgl2_defaults()
            }.using_resolution(adapter.limits()),
        },
        None,
    ).await.expect("failed to create device.");

    (size, instance, surf, hidpi_factor, adapter, dev, queue)
}