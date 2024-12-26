use wgpu::{RenderPipeline, Texture, TextureView};
use winit::dpi::PhysicalSize;
use crate::vertex::Vertex;

pub fn create_depth_texture(dev: &wgpu::Device, size: PhysicalSize<u32>) -> (Texture, TextureView) {
    let depth_texture = dev.create_texture(&wgpu::TextureDescriptor {
        label: Some("depth_texture"),
        size: wgpu::Extent3d {
            width: size.width,
            height: size.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });

    let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

    (depth_texture, depth_view)
}

pub fn create_main_pipeline(dev: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout, swapchain_format: wgpu::TextureFormat) -> RenderPipeline {
    let vert_shader = unsafe { dev.create_shader_module_spirv(&wgpu::include_spirv_raw!("shader/glsl/vert.spv")) };
    let frag_shader = unsafe { dev.create_shader_module_spirv(&wgpu::include_spirv_raw!("shader/glsl/frag.spv")) };

    let pipeline_layout = dev.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("render_pipeline_layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    dev.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vert_shader,
            entry_point: "main",
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &frag_shader,
            entry_point: "main",
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState {
            cull_mode: Some(wgpu::Face::Back),
            front_face: wgpu::FrontFace::Ccw,

            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    })
}