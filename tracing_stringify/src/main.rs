use tracing::metadata::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::Layer;
use tracing_subscriber::prelude::*;


struct Point {
    x: i32,
    y: i32,
}

impl ::std::fmt::Debug for Point {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        eprintln!("fmt is called!");
        f.debug_struct("Point")
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}

fn main() {
    let fmt_layer = fmt::layer();

    tracing_subscriber::registry()
    // .with(fmt_layer.with_filter(LevelFilter::TRACE))
    .with(fmt_layer.with_filter(LevelFilter::INFO))
    .init();

    let origin = Point { x: 0, y: 0 };
    tracing::trace!("{:?}", origin);
}
