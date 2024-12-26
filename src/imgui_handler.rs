use imgui::{Condition, Context, FontSource};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::WinitPlatform;
use wgpu::{CommandEncoder, Device, LoadOp, Operations, Queue, RenderPassColorAttachment, RenderPassDescriptor, StoreOp, SurfaceConfiguration, TextureView};
use winit::window::Window;

pub fn setup_imgui(window: &Window, dev: &Device, queue: &Queue, surf_cfg: &SurfaceConfiguration, hidpi_factor: f64) -> (Context, WinitPlatform, Renderer) {
    let mut imgui = Context::create();
    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        &window,
        imgui_winit_support::HiDpiMode::Default,
    );
    imgui.set_ini_filename(None);

    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(imgui::FontConfig {
            oversample_h: 1,
            pixel_snap_h: true,
            size_pixels: font_size,
            ..Default::default()
        }),
    }]);

    let renderer_cfg = RendererConfig {
        texture_format: surf_cfg.format,
        ..Default::default()
    };

    let renderer = Renderer::new(&mut imgui, &dev, &queue, renderer_cfg);

    (imgui, platform, renderer)
}

pub fn base_ui(ui: &imgui::Ui, frame_time: f32, fps: f32) {
    ui.window("info").size([400.0, 200.0], Condition::FirstUseEver).position([10.0, 10.0], Condition::FirstUseEver).build(|| {
        ui.text(format!("frame_time: {frame_time}"));
        ui.text(format!("fps: {fps}"));

        let mouse_pos = ui.io().mouse_pos;
        ui.text(format!(
            "mouse_pos: ({:.1},{:.1})",
            mouse_pos[0], mouse_pos[1]
        ));
    });
}

pub fn imgui_render_pass(dev: &Device, queue: &Queue, encoder: &mut CommandEncoder, imgui: &mut Context, renderer: &mut Renderer, view: &TextureView) {
    let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some("imgui_render_pass"),
        color_attachments: &[Some(RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Load,
                store: StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
    });

    renderer.render(imgui.render(), &queue, &dev, &mut render_pass).expect("rendering imgui failed.");
}