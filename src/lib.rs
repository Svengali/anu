#![allow(dead_code)]

use tracing::{span, info, Level};

pub mod com;
//mod ent;
pub mod sys;
pub mod layer;

struct Tracer {}

impl Tracer {
    pub fn span() {}
}

impl Drop for Tracer {
    fn drop(&mut self) {
        println!("Dropping Tracer!");
    }
}

pub fn print(str: String) {
    let main_span = span!(Level::INFO, "it_works()");
    let _span = main_span.enter();

    info!("{}", str);

    //println!("Hello, world!");
}
