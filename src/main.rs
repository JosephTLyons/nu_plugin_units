use nu_plugin::{serve_plugin, JsonSerializer};
use nu_plugin_units::nu::Units;

fn main() {
    serve_plugin(&mut Units {}, JsonSerializer {})
}
