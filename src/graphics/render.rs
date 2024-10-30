use std::collections::HashMap;

use super::{Entity, GameWorld};

pub trait RenderSystem {
    fn draw(&self, world: &GameWorld, entiy: &Vec<Entity>);
}

pub trait UpdateSystem {
    fn update(&self, world: &mut GameWorld, dt: f32);
}

pub struct DefaultUpdateSystem;

impl UpdateSystem for DefaultUpdateSystem {
    fn update(&self, world: &mut GameWorld, dt: f32) {
        for entity in &mut world.entity {
            // Обрабатывайте логику обновления сущности
        }
    }
}

pub struct GameSystems {
    render_system: Box<dyn RenderSystem>,
    update_system: Box<dyn UpdateSystem>,
}

impl GameSystems {
    pub fn new(render_system: Box<dyn RenderSystem>, update_system: Box<dyn UpdateSystem>) -> Self {
        Self {
            render_system,
            update_system
        }
    }

    pub fn update(&self, world: &mut GameWorld, dt: f32) {
        self.update_system.update(world, dt);
    }

    pub fn draw(&self, world: &GameWorld, entities: &Vec<Entity>) {
        self.render_system.draw(world, entities);
    }
}