use crate::uniform::Uniform;
use crate::vertex::Vertex;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Device,
    ShaderStages,
};
use winit::dpi::PhysicalSize;

pub fn create_uniform_buffer(dev: &Device, size: PhysicalSize<u32>) -> (Uniform, Buffer) {
    let mut uniform = Uniform::default();
    uniform.res = [size.width, size.height];

    let uniform_buffer = dev.create_buffer_init(&BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[uniform]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    (uniform, uniform_buffer)
}

pub fn create_polygon_buffers(
    dev: &Device,
    vertices: &Vec<Vertex>,
    indices: &Vec<u32>,
) -> (Buffer, Buffer, u32) {
    // TODO: add vertex staging buffer
    let vertex_buffer = dev.create_buffer_init(&BufferInitDescriptor {
        label: Some("vertex_buffer"),
        contents: bytemuck::cast_slice(vertices.as_slice()),
        usage: BufferUsages::VERTEX,
    });

    // TODO: add index staging buffer
    let num_indices = indices.len() as u32;
    let index_buffer = dev.create_buffer_init(&BufferInitDescriptor {
        label: Some("index_buffer"),
        contents: bytemuck::cast_slice(indices.as_slice()),
        usage: BufferUsages::INDEX,
    });

    (vertex_buffer, index_buffer, num_indices)
}

/// Creates bind group layout and bind group for uniform and SVO buffer .
///
/// # Arguments
///
/// * `dev` - A reference to the wgpu device.
/// * `uniform_buffer` - A reference to the uniform buffer.
/// * `svo_buffer` - A reference to the SVO buffer.
///
/// # Returns
///
/// A tuple containing the bind group layout and bind group.
pub fn create_buffer_descriptors(
    dev: &Device,
    uniform_buffer: &Buffer,
) -> (BindGroupLayout, BindGroup) {
    let bind_group_layout = dev.create_bind_group_layout(&BindGroupLayoutDescriptor {
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::all(),
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("bind_group_layout"),
    });

    let bind_group = dev.create_bind_group(&BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        }],
        label: Some("bind_group"),
    });

    (bind_group_layout, bind_group)
}
