use glow::*;
use sdl2::{
    event::Event,
    video::{GLContext, Window},
};

use crate::render::renderer::Renderer;

use self::screen::Screen;

pub mod ecs;
pub mod input;
pub mod screen;
pub mod world;

pub struct GameState {
    renderer: Renderer,
    screen: Screen,
}

impl GameState {
    pub fn new(gl: Context, gl_context: GLContext, window: &Window) -> Self {
        let renderer = Renderer::new(gl, gl_context, window);
        let screen = Screen::new(&renderer);
        Self { renderer, screen }
    }

    pub fn physics_update(&mut self) {
        self.screen.physics_update();
    }

    pub fn update(&mut self, delta: f32) {
        self.screen.update(delta);
    }

    pub fn draw(&mut self) {
        self.screen.draw(&mut self.renderer);
    }

    pub fn handle_input(&mut self, event: Event) {
        self.screen.handle_input(event);
    }

    pub fn handle_resize(&mut self, window: &Window) {
        let (w, h) = window.drawable_size();
        self.renderer.handle_resize(w as i32, h as i32);
    }
}
