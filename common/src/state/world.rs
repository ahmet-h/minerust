use glam::{vec2, vec3};
use hecs::World;
use sdl2::{event::Event, keyboard::Scancode, mouse::MouseButton};

use crate::render::{camera::Camera, mesh::Mesh, pipelines::sky::SkyVertex, renderer::Renderer};

use super::input::InputState;

pub struct GameWorld {
    input: InputState,
    world: World,
    camera: Camera,
}

impl GameWorld {
    pub fn new(renderer: &Renderer) -> Self {
        let sky = vec![
            SkyVertex::new(vec2(-1.0, 1.0)),
            SkyVertex::new(vec2(1.0, 1.0)),
            SkyVertex::new(vec2(1.0, -1.0)),
            SkyVertex::new(vec2(-1.0, -1.0)),
        ];

        let mut sky_mesh = Mesh::new();
        for s in sky {
            sky_mesh.push(s);
        }

        let sky_model = renderer.create_model(&sky_mesh);

        Self {
            input: Default::default(),
            world: World::new(),
            camera: Camera::new(vec3(0., 1., 0.)),
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
                self.camera.add_yaw(xrel as f32 * 0.2);
                self.camera.add_pitch(-yrel as f32 * 0.2);
                self.camera.update();
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
