use tracing::{info, error, warn};
use tracing_subscriber::prelude::*;
use dlt_tracing_subscriber::DltLayer;
use tracing::{info_span, error_span};

fn main() {
    let layer = DltLayer::new("APP","An example application");
    tracing_subscriber::registry().with(layer).init();
        
    let outer_span = info_span!("outer", level = 0);
    let _outer_entered = outer_span.entered();

    let inner_span = error_span!("inner", level = 1);
    let _inner_entered = inner_span.entered();

    error!(a_bool = true, answer = 42, message = "first example");
    info!(a_bool = true, answer = 42, message = "first example");
    warn!(a_bool = true, answer = 42, message = "first example");
    println!("Hello, world!");
}
