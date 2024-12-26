mod audio;
mod imgui_handler;
mod input_handler;
mod pipelines;
mod streaming;
mod uniform;
mod vertex;
mod wgpu_core;

use crate::imgui_handler::{base_ui, imgui_render_pass, setup_imgui};
use crate::input_handler::{handle_keyboard};
use crate::pipelines::{create_depth_texture, create_main_pipeline};
use crate::streaming::{create_buffer_descriptors, create_uniform_buffer};
use crate::vertex::{Vertex, CUBE_INDICES, CUBE_UV_COORDS, CUBE_VERTEX_POSITIONS};
use crate::wgpu_core::FrameInfo;
use ansi_term::Color::{Blue, Red, Yellow};
use ansi_term::Style;
use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use pollster::block_on;
use std::io::Write;
use std::{env, io};
use glam::Vec2;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    window::WindowBuilder,
};
use crate::audio::Audio;

async fn run(event_loop: EventLoop<()>, window: Window) {
    //
    // wgpu core
    //
    let (mut size, instance, surf, hidpi_factor, adapter, dev, queue) =
        wgpu_core::init_wgpu(&window).await;
    //
    // create buffers
    //
    let (mut uniform, uniform_buffer) = create_uniform_buffer(&dev, size);
    uniform.update_proj(Vec2::new(size.width as f32, size.height as f32));
    let (vertex_buffer, index_buffer, num_indices) = streaming::create_polygon_buffers(
        &dev,
        &CUBE_VERTEX_POSITIONS
            .iter()
            .enumerate()
            .map(|(i, &pos)| Vertex::new(pos, CUBE_UV_COORDS[i]))
            .collect::<Vec<Vertex>>(),
        &Vec::from(CUBE_INDICES),
    );
    //
    // depth texture
    //
    let (_, mut depth_view) = create_depth_texture(&dev, size);
    //
    // bind group
    //
    let (bind_group_layout, bind_group) = create_buffer_descriptors(&dev, &uniform_buffer);
    //
    // create pipelines
    //
    let swapchain_capabilities = surf.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let main_pipeline = create_main_pipeline(&dev, &bind_group_layout, swapchain_format);

    let mut surf_cfg = surf
        .get_default_config(&adapter, size.width, size.height)
        .unwrap();
    surf.configure(&dev, &surf_cfg);
    //
    // imgui setup
    //
    let (mut imgui, mut platform, mut renderer) =
        setup_imgui(&window, &dev, &queue, &surf_cfg, hidpi_factor);
    //
    // audio setup
    //
    let audio = Audio::new();
    //
    // main loop, window event handling
    //
    let mut frame_info = FrameInfo::default();

    let mut last_cursor = None;

    let clear_color = wgpu::Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.0,
    };

    event_loop
        .run(|event, elwt| {
            let _ = (&instance, &adapter);
            elwt.set_control_flow(ControlFlow::Poll);

            match event {
                Event::AboutToWait => window.request_redraw(),
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(new_size) => {
                        surf_cfg.width = new_size.width.max(1);
                        surf_cfg.height = new_size.height.max(1);
                        size = *new_size;
                        surf.configure(&dev, &surf_cfg);
                        depth_view = create_depth_texture(&dev, size).1;
                        uniform.update_proj(Vec2::new(size.width as f32, size.height as f32));
                        queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniform]));
                        window.request_redraw();
                    }
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::KeyboardInput { event, .. } => {
                        handle_keyboard(
                            event,
                            elwt,
                            &window,
                            &queue,
                            &mut uniform,
                            &uniform_buffer,
                        );
                    }
                    WindowEvent::RedrawRequested => {
                        let delta_time = frame_info.fetch();

                        imgui.io_mut().update_delta_time(delta_time);
                        let frame = surf
                            .get_current_texture()
                            .expect("failed to acquire next swapchain texture.");

                        uniform.time += 1;
                        queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniform]));

                        platform
                            .prepare_frame(imgui.io_mut(), &window)
                            .expect("failed to prepare frame.");
                        let ui = imgui.frame();
                        base_ui(&ui, frame_info.frame_time, frame_info.fps);

                        let mut encoder: wgpu::CommandEncoder =
                            dev.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: None,
                            });
                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        if last_cursor != Some(ui.mouse_cursor()) {
                            last_cursor = Some(ui.mouse_cursor());
                            platform.prepare_render(ui, &window);
                        }

                        {
                            let mut render_pass =
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: None,
                                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                        view: &view,
                                        resolve_target: None,
                                        ops: wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(clear_color),
                                            store: wgpu::StoreOp::Store,
                                        },
                                    })],
                                    depth_stencil_attachment: Some(
                                        wgpu::RenderPassDepthStencilAttachment {
                                            view: &depth_view,
                                            depth_ops: Some(wgpu::Operations {
                                                load: wgpu::LoadOp::Clear(1.0),
                                                store: wgpu::StoreOp::Store,
                                            }),
                                            stencil_ops: None,
                                        },
                                    ),
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                });

                            render_pass.set_pipeline(&main_pipeline);
                            render_pass.set_bind_group(0, &bind_group, &[]);
                            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                            render_pass.set_index_buffer(
                                index_buffer.slice(..),
                                wgpu::IndexFormat::Uint32,
                            );
                            render_pass.draw_indexed(0..num_indices, 0, 0..1);
                        }

                        imgui_render_pass(
                            &dev,
                            &queue,
                            &mut encoder,
                            &mut imgui,
                            &mut renderer,
                            &view,
                        );

                        queue.submit(Some(encoder.finish()));
                        frame.present();
                    }
                    _ => {}
                },
                _ => {}
            }

            platform.handle_event(imgui.io_mut(), &window, &event);
        })
        .unwrap();
}

fn main() {
    env::set_var("RUST_LOG", "info");
    let mut builder = Builder::from_default_env();

    builder
        .target(Target::Stdout)
        .format(|buf, record| {
            let level = record.level();
            let style = match level {
                log::Level::Error => Style::new().bold().fg(Red),
                log::Level::Warn => Style::new().bold().fg(Yellow),
                log::Level::Info => Style::new().bold().fg(Blue),
                _ => return Ok(()),
            };

            buf.write_fmt(format_args!(
                "{}: {}\n",
                style.paint(record.level().to_string()),
                record.args()
            ))
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "error writing log."))
        })
        .filter(None, LevelFilter::Trace)
        .init();

    println!();
    info!("logger initialized.");

    let event_loop = EventLoop::new().unwrap();

    let window = {
        let size = LogicalSize::new(1280.0, 720.0);
        WindowBuilder::new()
            .with_inner_size(size)
            .with_title(env!("CARGO_PKG_NAME"))
            .build(&event_loop)
            .unwrap()
    };

    // window.set_cursor_grab(CursorGrabMode::Locked).unwrap();

    block_on(run(event_loop, window));
}
