use nu_plugin::{JsonSerializer, serve_plugin};
use nu_plugin_units::nu::Units;

fn main() {
    serve_plugin(&mut Units {}, JsonSerializer {})
}
