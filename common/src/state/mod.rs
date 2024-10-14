use glow::Context;
use screen::Screen;
use sdl2::{
    event::Event,
    video::{GLContext, Window},
};

use crate::render::renderer::Renderer;

pub mod input;
pub mod screen;
pub mod world;

pub struct AppState {
    renderer: Renderer,
    screens: Vec<Screen>,
}

impl AppState {
    pub fn new(gl: Context, gl_context: GLContext, window: &Window) -> Self {
        let renderer = Renderer::new(gl, gl_context, window);
        let screen = Screen::new(&renderer);

        let screens = vec![screen];

        Self { renderer, screens }
    }

    pub fn handle_input(&mut self, event: Event) {
        if let Some(screen) = self.screens.last_mut() {
            screen.handle_input(event);
        }
    }

    pub fn is_mouse_grabbed(&self) -> bool {
        if let Some(screen) = self.screens.last() {
            return screen.is_mouse_grabbed();
        }

        false
    }

    pub fn handle_resize(&mut self, window: &Window) {
        let (w, h) = window.drawable_size();
        self.renderer.handle_resize(w as i32, h as i32);
    }
}
