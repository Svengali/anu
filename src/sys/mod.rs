use std::{borrow::Borrow};

use tracing::{span};

pub mod mov;

#[derive(Copy, Clone)]
pub enum Status {
    Invalid,
    Starting,
    Running,
    Stopping,
}


pub trait System {
    fn startup(&mut self);
    fn shutdown(&mut self);
}


pub trait Ticks {
    fn tick(&mut self);
}


struct SystemDetail {
    priority: u32,
    system: Box<dyn System>,
    status: Status,
}

/*
impl PartialOrd for SystemDetail {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl Ord for SystemDetail {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialEq for SystemDetail {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}
impl Eq for SystemDetail {}
*/


pub struct Systems {
    systems: Vec<SystemDetail>,
}

pub struct Tickables {
    tickables: Vec<Box<dyn Ticks>>,
}

impl Tickables {
    pub fn add( &mut self, tick: Box<dyn Ticks> ) {
        self.tickables.push( tick );
    }
}

impl Systems {
    pub fn new() -> Self {
        Systems {
            systems: Vec::new(),
        }
    }

    pub fn add( &mut self, priority: u32, mut system: Box<dyn System> ) {

        system.startup();

        let detail = SystemDetail {
            priority,
            system: system,
            status: Status::Starting,
        };

        self.systems.push( detail );

        self.systems.sort_unstable_by_key( |v| v.priority );


        //detail.system.startup();
    }


}


