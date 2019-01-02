/// Use this example to test auto coloring functionality
///
/// `$ cargo run --example auto` should result with colored output (when using
/// decent terminal).
///
/// `$ TERM=dump cargo run --example auto` should result with output without
/// colors.
///
/// `$ cargo run --example auto 2>&1 | cat` should also result with decolorized
/// output as stderr redirection is detected internally.
///
/// `$ cargo run --example auto &> out.txt` redirecting to file also suppresses
/// using colors.

use log::{error, warn, info, debug, trace};
use colored_logger::{ColorChoice, FormatterBuilder};

fn main() {
    let formatter = FormatterBuilder::new()
        .with_color(ColorChoice::Auto)
        .build();

    flexi_logger::Logger::with_str("auto=trace")
        .format(formatter)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    error!("This is an error example");
    warn!("This is a warning example");
    info!("This is an info example example");
    debug!("This is a debug example");
    trace!("This is a trace example");
}
