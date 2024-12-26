use crate::uniform::Uniform;
use wgpu::{Buffer, Queue};
use winit::event::{ElementState, KeyEvent};
use winit::event_loop::EventLoopWindowTarget;
use winit::keyboard::{Key, NamedKey};
use winit::platform::modifier_supplement::KeyEventExtModifierSupplement;
use winit::window::{Fullscreen, Window};

pub fn handle_keyboard(
    event: &KeyEvent,
    elwt: &EventLoopWindowTarget<()>,
    window: &Window,
    queue: &Queue,
    uniform: &mut Uniform,
    uniform_buffer: &Buffer,
) {
    if event.state == ElementState::Pressed {
        match event.key_without_modifiers().as_ref() {
            Key::Named(NamedKey::F12) => {
                elwt.exit();
            },
            Key::Named(NamedKey::F11) => {
                if window.fullscreen().is_some() {
                    window.set_fullscreen(None);
                } else {
                    window.set_fullscreen(Some(Fullscreen::Borderless(window.current_monitor())));
                }
            }

            _ => {}
        }

        queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniform.clone()]));
    }
}