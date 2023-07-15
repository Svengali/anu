

#![allow(dead_code)]

use std::time::Instant;

use tracing::{span, Level, field::{Visit}, info};


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

    info!("{}", str);

    //println!("Hello, world!");
}


use tracing_subscriber::Layer;


#[derive(Default)]
pub struct PrintVisitor {
    pub visited:bool
}


impl Visit for PrintVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        //println!( "{} = {}", field.name(), value. );

        if self.visited {
            print!(" ");
        } else {
            self.visited = true;
        }

        if field.name() == "message" {
            let v = format!("[{value:?}]");
            print!( "{}", v );
        } else {
            let v = format!("{}: {value:?}", field.name());
            print!( "{}", v );
        };
        
    }
}



pub struct CustomLayer {
    pub depth: u32,
}

impl Default for CustomLayer {
    fn default() -> Self {
        Self { depth: Default::default() }
    }
}


#[derive(Debug)]
struct SpanInfo {
    pub depth: usize,
    pub new_time: Instant,
    pub start_time: Instant,
    pub end_time: Instant,
}

impl Default for SpanInfo {
    fn default() -> Self {
        Self { depth: Default::default(), new_time: Instant::now(), start_time: Instant::now(), end_time: Instant::now() }
    }
}

impl CustomLayer {
    

    pub fn prefix<S>(
        ctx: &tracing_subscriber::layer::Context<'_, S>,
        level: &str,
        prefix: &str
    ) -> String
    where
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
    {
        let cur_count = if let Some(cur_span_id) = ctx.current_span().id() {
            let cur = ctx.span( cur_span_id ).unwrap();
            let cur_ext = cur.extensions();
            let cur_span_data = cur_ext.get::<SpanInfo>().unwrap();
            cur_span_data.depth
        } else {
            0
        };

        let bars = format!("{:|<1$}", "", (cur_count + 1));
        let full_prefix = format!("{:12}", prefix);

        format!("{full_prefix}{level}{bars}")
    }

}

impl<S> Layer<S> for CustomLayer
where
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{

    //*
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        //let prefix = CustomLayer::prefix(event, event.metadata().target().to_string());

        /*
        let cur_count = if let Some(cur_span_id) = ctx.current_span().id() {
            let cur = ctx.span( cur_span_id ).unwrap();
            let cur_ext = cur.extensions();
            let cur_span_data = cur_ext.get::<SpanInfo>().unwrap();
            cur_span_data.0
        } else {
            0
        };

        let bars = format!("{:|<1$}", "", (cur_count + 1));

        print!("{:8}>{}", event.metadata().target(), bars );

        let mut visitor = PrintVisitor { visited: false };
        event.record( &mut visitor );

        println!("");
        */

        let sym = match *event.metadata().level() {
            Level::TRACE => ".",
            Level::DEBUG => ":",
            Level::INFO  => " ",
            Level::WARN  => "+",
            Level::ERROR => "*",
        };

        let prefix = CustomLayer::prefix(&ctx, sym, event.metadata().target());
        print!("{}-", prefix );

        let mut visitor = PrintVisitor { visited: false };
        event.record( &mut visitor );

        println!("");


        //println!("Got event!");

        /*
        println!("  level={:?}", event.metadata().level());
        println!("  target={:?}", event.metadata().target());
        println!("  name={:?}", event.metadata().name());
        for field in event.fields() {
            println!("  field {}", field);
        }

        //let mut visitor = PrintVisitor::new( &mut event.fields() );

        let mut visitor = PrintVisitor {};
        event.record( &mut visitor );
        // */

        //println!("DEBUG");
        //println!( "  Debug={:?}", event )

    }

    fn on_register_dispatch(&self, collector: &tracing::Dispatch) {
        let _ = collector;
        //println!("on_register_dispatch");
    }

    fn on_layer(&mut self, subscriber: &mut S) {
        let _ = subscriber;
        //println!("on_layer");
    }

    fn enabled(&self, metadata: &tracing::Metadata<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) -> bool {
        let _ = (metadata, ctx);
        //println!("enabled");
        true
    }

    fn on_new_span(&self, _attrs: &span::Attributes<'_>, id: &span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        //let _ = (attrs, id, ctx);

        //println!("on_new_span");

        let cur_count = if let Some(cur_span_id) = ctx.current_span().id() {
            let cur = ctx.span( cur_span_id ).unwrap();
            let cur_ext = cur.extensions();
            let cur_span_data = cur_ext.get::<SpanInfo>().unwrap();
            cur_span_data.depth
        } else {
            0
        };


        // Get a reference to the internal span data
        let span = ctx.span(id).unwrap();

        // Get the special place where tracing stores custom data
        let mut extensions = span.extensions_mut();


        // And store our data
        

        let storage = SpanInfo { 
            depth: cur_count + 1, 
            new_time: Instant::now(), 
            start_time: Instant::now(),
            end_time: Instant::now(),
        };

        extensions.insert::<SpanInfo>(storage);

    }

    fn max_level_hint(&self) -> Option<tracing::metadata::LevelFilter> {
        //println!("max_level_hint");
        None
    }

    fn on_record(&self, _span: &span::Id, _values: &span::Record<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        println!("on_record");

    }

    fn on_follows_from(&self, _span: &span::Id, _follows: &span::Id, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        println!("on_follows_from");

    }

    fn event_enabled(&self, _event: &tracing::Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) -> bool {
        //println!("event_enabled");
        true
    }

    fn on_enter(&self, id: &span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {

        let prefix = CustomLayer::prefix(&ctx, ">", "" );


        let cur = ctx.span( id ).unwrap();
        let mut cur_ext = cur.extensions_mut();
        let cur_span_data = cur_ext.get_mut::<SpanInfo>().unwrap();

        cur_span_data.start_time = Instant::now();

        println!("{}-------->>>{}>>>----", prefix, cur.name() );

        //println!("{}{:|<1$}", "", (cur_count + 1));

    }

    fn on_exit(&self, id: &span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        //println!("on_exit");

        //let cur_span_id = ctx.current_span().id().clone();

        let prefix = CustomLayer::prefix(&ctx, "<", "");

        let dur_usec: u128;

        let Some(cur) = ctx.span( id ) else { todo!() };
        let mut cur_ext = cur.extensions_mut();
        let cur_span_data = cur_ext.get_mut::<SpanInfo>().unwrap();

        cur_span_data.end_time = Instant::now();

        let duration = cur_span_data.end_time - cur_span_data.start_time;

        dur_usec = duration.as_micros();

        //cur_span_data.


        let dur_100usec = (dur_usec / 10) % 100;
        let dur_ms = dur_usec / 1000;

        println!("{}|-({:?}.{:02?})-<<<{}<<<---", prefix, dur_ms, dur_100usec, cur.name() )

    }

    fn on_close(&self, _id: span::Id, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        //println!("on_close");
        //Dont need anything here
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



        //println!("Hello, world!");
    }
}
 