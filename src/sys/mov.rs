

//use super::*;

use super::System;

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
        todo!()
    }

    fn shutdown(&mut self) {
        todo!()
    }
}
