

#![allow(dead_code)]

use tracing::{span, Level, field::{AsField, Visit}};


mod com;
//mod ent;
mod sys;



struct Tracer {
    
}

impl Tracer {
    pub fn span() {

    }
}

impl Drop for Tracer {
    fn drop(&mut self) {
        println!("Dropping Tracer!");
    }
    
}



pub fn print( str: String ) {
    let main_span = span!(Level::INFO, "it_works()");
    let _span = main_span.enter();

    println!("Hello, world!");
}


use tracing_subscriber::Layer;


pub struct PrintVisitor {
}

impl Visit for PrintVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        //println!( "{} = {}", field.name(), value. );
        let name = field.name();
        let v = format!("HEYA {name:?}: {value:?}");
        println!( "{}", v );
    }
}



pub struct CustomLayer {
    depth: u32,
}

impl Default for CustomLayer {
    fn default() -> Self {
        Self { depth: Default::default() }
    }
}

impl<S> Layer<S> for CustomLayer
where
    S: tracing::Subscriber,
{

    //*
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        println!("Got event!");
        println!("  level={:?}", event.metadata().level());
        println!("  target={:?}", event.metadata().target());
        println!("  name={:?}", event.metadata().name());
        for field in event.fields() {
            println!("  field {}", field);
        }

        //let mut visitor = PrintVisitor::new( &mut event.fields() );

        let mut visitor = PrintVisitor {};

        event.record( &mut visitor );

        println!("DEBUG");
        println!( "  Debug={:?}", event.metadata())

    }

    fn on_register_dispatch(&self, collector: &tracing::Dispatch) {
        let _ = collector;
        println!("on_register_dispatch");
    }

    fn on_layer(&mut self, subscriber: &mut S) {
        let _ = subscriber;
        println!("on_layer");
    }

    fn enabled(&self, metadata: &tracing::Metadata<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) -> bool {
        let _ = (metadata, ctx);
        println!("enabled");
        true
    }

    fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let _ = (attrs, id, ctx);
        println!("on_new_span");
    }

    fn max_level_hint(&self) -> Option<tracing::metadata::LevelFilter> {
        println!("max_level_hint");
        None
    }

    fn on_record(&self, _span: &span::Id, _values: &span::Record<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        println!("on_record");

    }

    fn on_follows_from(&self, _span: &span::Id, _follows: &span::Id, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        println!("on_follows_from");

    }

    fn event_enabled(&self, _event: &tracing::Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) -> bool {
        println!("event_enabled");
        true
    }

    fn on_enter(&self, _id: &span::Id, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        println!("on_enter");

    }

    fn on_exit(&self, _id: &span::Id, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        println!("on_exit");

    }

    fn on_close(&self, _id: span::Id, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        println!("on_close");

    }

    fn on_id_change(&self, _old: &span::Id, _new: &span::Id, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        println!("on_id_change");

    }
    // */



    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        //tracing_subscriber::fmt().init();

        /*
        let fmt = tracing_subscriber::fmt()

        .with_span_events(FmtSpawn::FULL)
        .with_max_level(Level::DEBUG)

        .pretty()
        .compact()
    
        .init();
        // */

    


        let main_span = span!(Level::INFO, "it_works()");
        let _span = main_span.enter();
    
        println!("Hello, world!");
    }
}
 