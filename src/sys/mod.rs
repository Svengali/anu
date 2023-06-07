use std::cell::RefCell;
use std::rc::Rc;

pub mod mov;

pub trait System {
    fn update(&self);
}

trait Render {
    fn render(&self);
}

trait Physics {
    fn apply_physics(&mut self);
}

pub struct Systems {
    systems: Vec<Rc<dyn System>>,
    render_systems: Vec<Rc<dyn Render>>,
    physics_systems: Vec<Rc<RefCell<dyn Physics>>>,
}

impl Systems {
    pub fn new() -> Self {
        Systems {
            systems: Vec::new(),
            render_systems: Vec::new(),
            physics_systems: Vec::new(),
        }
    }

    pub fn add<T: 'static + System>(&mut self, system: T) -> Rc<T> {
        let system = Rc::new(system);

        self.systems.push( system.clone() );

        system
    }

    pub fn update(&mut self) {
        for system in &mut self.systems {
            system.update();
        }
    }

    pub fn render(&self) {
        for system in &self.render_systems {
            system.render();
        }
    }

    pub fn apply_physics(&mut self) {
        for system in &mut self.physics_systems {
            let mut thing = system.borrow_mut();
            thing.apply_physics();
        }
    }
}
