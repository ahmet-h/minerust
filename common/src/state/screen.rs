use sdl2::event::Event;

use crate::render::renderer::Renderer;

use super::world::GameWorld;

pub struct Screen {
    world: Option<GameWorld>,
}

impl Screen {
    pub fn new(renderer: &Renderer) -> Self {
        Self {
            world: Some(GameWorld::new(renderer)),
        }
    }

    pub fn handle_input(&mut self, event: Event) {
        if let Some(w) = &mut self.world {
            w.handle_input(event);
        }
    }

    pub fn is_mouse_grabbed(&self) -> bool {
        if let Some(w) = &self.world {
            return w.is_mouse_grabbed();
        }

        false
    }
}
