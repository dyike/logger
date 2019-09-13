#[macro_use]
extern crate logger;

pub fn main() {
    println!("A simple demo of the logger");

    logger::Logger::new()
        .label("demo")
        .level(logger::Level::Trace)
        .init()
        .expect("Failed to initialize logger");
    trace!("Some tracing message");
    debug!("Some debugging message");
    info!("Just some general info");
    warn!("You might want to know this");
    error!("You need to know this");
    fatal!("Something really bad happened! Terminating program");
    // code below would be unreachable
}
