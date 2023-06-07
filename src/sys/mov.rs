

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
    fn update(&self) {
        todo!()
    }
}
