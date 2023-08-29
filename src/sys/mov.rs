

//use super::*;

use super::{System, Ticks};

#[derive(Debug)]
pub struct Move {
    pub v: i32,
}

impl Move {
    pub fn new() -> Move {
        Move {
            v: 1024,
        }
    }
}

impl System for Move {
    fn startup(&mut self) {
    }

    fn shutdown(&mut self) {
    }

}

impl Ticks for Move {
    fn tick(&mut self) {
    }
}