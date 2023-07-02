use dlt_tracing_subscriber::DltLayer;
use tracing::{error, info, warn};
use tracing::{error_span, info_span, trace_span};
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Layer;

pub struct CustomLayer;

impl<S> Layer<S> for CustomLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        println!("Got event!");
        println!("  meatadata={:?}", event.metadata());
        println!("  level={:?}", event.metadata().level());
        println!("  target={:?}", event.metadata().target());
        println!("  name={:?}", event.metadata().name());
        for field in event.fields() {
            println!("  field={}", field.name());
        }
    }
}

fn main() {
    let layer = DltLayer::new("APP", "An example application");
    let fmt_layer = fmt::layer();
    let fmt_layer2 = fmt::layer();
    tracing_subscriber::registry()
        .with(layer)
        .with(fmt_layer)
        .with(fmt_layer2)
        .with(CustomLayer)
        .init();

    {
        let outer_span = info_span!("outer", level = 0);
        let _outer_entered = outer_span.entered();

        {
            let inner_span = error_span!("inner", level = 1);
            let _inner_entered = inner_span.entered();

            error!(a_bool = true, answer = 42, message = "first example");
            info!(a_bool = true, answer = 42, message = "first example");
            warn!(a_bool = true, answer = 42, message = "first example");
        }

        {
            let inner_span = trace_span!("inner", level = 1);
            let _inner_entered = inner_span.entered();

            error!(a_bool = true, answer = 43, message = "second example");
            info!(a_bool = true, answer = 43, message = "second example");
            warn!(a_bool = true, answer = 43, message = "second example");
        }
    }

    error!(a_bool = true, answer = 44, message = "third example");
    info!(a_bool = true, answer = 44, message = "third example");
    warn!(a_bool = true, answer = 44, message = "third example");

    println!("Hello, world!");
}
