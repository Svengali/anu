use std::borrow::BorrowMut;
use std::{cell::RefCell, borrow::Borrow};
use std::rc::Rc;

use tracing::{span};

pub mod mov;

pub enum Status {
    Invalid,
    Starting,
    Running,
    Stopping,
}

pub trait Info {
}

pub trait System {
    fn startup(&mut self);
    fn shutdown(&mut self);
}

struct SystemDetail {
    system: Rc<RefCell<dyn System>>,
    status: Status,
}

pub struct Systems {
    systems: Vec<SystemDetail>,
    render_systems: Vec<Rc<RefCell<dyn Render>>>,
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

    pub fn add<T: 'static + System + Info>(&mut self, system: T) -> Rc<RefCell<T>> {
        let rc_system = Rc::new(RefCell::new(system));
        rc_system.borrow_mut().startup();
        self.systems.push(SystemDetail {
            system: Rc::clone(&rc_system) as Rc<RefCell<dyn System>>,
            status: Status::Starting, // system is now in the "Starting" state
        });
        rc_system
    }

    /*
    pub fn update(&mut self) {
        for system in &mut self.systems {
            system.update();
        }
    }
    */

    pub fn render(&self) {
        for system in &self.render_systems {

            //let sys = system.get_mut();
            //sys.render();

        }
    }

    pub fn apply_physics(&mut self) {
        for system in &mut self.physics_systems {
            //let mut thing = system.borrow_mut();
            //thing.apply_physics();
        }
    }
}



trait Render {
    fn render(&mut self);
}

trait Physics {
    fn apply_physics(&mut self);
}

