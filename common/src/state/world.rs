use hecs::World;
use sdl2::{event::Event, keyboard::Scancode, mouse::MouseButton};

use super::input::InputState;

pub struct GameWorld {
    input: InputState,
    world: World,
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            input: Default::default(),
            world: World::new(),
        }
    }

    pub fn handle_input(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                scancode: Some(Scancode::W),
                ..
            } => self.input.forward = true,
            Event::KeyDown {
                scancode: Some(Scancode::S),
                ..
            } => self.input.backward = true,
            Event::KeyDown {
                scancode: Some(Scancode::D),
                ..
            } => self.input.right = true,
            Event::KeyDown {
                scancode: Some(Scancode::A),
                ..
            } => self.input.left = true,
            Event::KeyUp {
                scancode: Some(Scancode::W),
                ..
            } => self.input.forward = false,
            Event::KeyUp {
                scancode: Some(Scancode::S),
                ..
            } => self.input.backward = false,
            Event::KeyUp {
                scancode: Some(Scancode::D),
                ..
            } => self.input.right = false,
            Event::KeyUp {
                scancode: Some(Scancode::A),
                ..
            } => self.input.left = false,
            Event::MouseMotion { xrel, yrel, .. } => {
                // self.camera.add_yaw(xrel as f32 * 0.2);
                // self.camera.add_pitch(-yrel as f32 * 0.2);
                // self.camera.update();
            }
            Event::KeyDown {
                scancode: Some(Scancode::Space),
                ..
            } => self.input.space_toggle = !self.input.space_toggle,
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                ..
            } => {
                self.input.grab_mouse = !self.input.grab_mouse;
            }
            _ => {}
        }
    }

    pub fn is_mouse_grabbed(&self) -> bool {
        self.input.grab_mouse
    }
}

impl Default for GameWorld {
    fn default() -> Self {
        Self::new()
    }
}
