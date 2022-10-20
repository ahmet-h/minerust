use glam::{vec2, vec3, Vec3};
use hecs::World;
use sdl2::{event::Event, keyboard::Scancode};

use crate::render::{
    camera::Camera,
    mesh::{Mesh, Quad, Vertex},
    model::Model,
    renderer::Renderer,
    texture::GameTexture,
};

use super::input::InputState;

pub struct GameWorld {
    camera: Camera,
    input: InputState,
    light_dir: Vec3,
    world: World,
}

impl GameWorld {
    pub fn new(renderer: &Renderer) -> Self {
        let camera = Camera::new(vec3(0., 1., 0.));

        let mut world = World::new();

        let mut mesh = Mesh::new();
        mesh.push_quad(Quad::new(
            Vertex::new(vec3(-5., 0., -5.), vec3(0., 1., 0.), vec2(0., 0.)),
            Vertex::new(vec3(-5., 0., 5.), vec3(0., 1., 0.), vec2(5., 0.)),
            Vertex::new(vec3(5., 0., 5.), vec3(0., 1., 0.), vec2(5., 5.)),
            Vertex::new(vec3(5., 0., -5.), vec3(0., 1., 0.), vec2(0., 5.)),
        ));
        let model = renderer.create_model(&mesh);

        let texture = renderer.create_texture("assets/wood.png");

        let floor = world.spawn((1, model, texture));

        Self {
            camera,
            input: Default::default(),
            light_dir: vec3(0.5, -1., -0.8).normalize(),
            world,
        }
    }

    pub fn physics_update(&mut self) {}

    pub fn update(&mut self, delta: f32) {
        self.camera.update_movement(&self.input, delta);
    }

    pub fn draw(&mut self, renderer: &Renderer) {
        renderer.prepare(&mut self.camera);

        for (entity, (num, model, texture)) in
            self.world.query::<(&i32, &Model, &GameTexture)>().iter()
        {
            renderer.bind_texture(texture);
            renderer.render(model);
        }

        renderer.end(&self.camera);
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
            _ => {}
        }
    }
}
