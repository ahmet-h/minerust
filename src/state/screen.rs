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

    pub fn physics_update(&mut self) {
        if let Some(w) = &mut self.world {
            w.physics_update();
        }
    }

    pub fn update(&mut self, delta: f32) {
        if let Some(w) = &mut self.world {
            w.update(delta);
        }
    }

    pub fn draw(&mut self, renderer: &Renderer) {
        if let Some(w) = &mut self.world {
            w.draw(renderer);
        }
    }

    pub fn handle_input(&mut self, event: Event) {
        if let Some(w) = &mut self.world {
            w.handle_input(event);
        }
    }
}
