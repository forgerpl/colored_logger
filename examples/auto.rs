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
///
/// Use `--color` option for experimenting with forcing/blocking colored output.
use clap::{App, Arg};
use colored_logger::FormatterBuilder;
use failure::Error;
use log::{debug, error, info, trace, warn};

fn main() -> Result<(), Error> {
    let matches = App::new("Colored Logger example")
        .arg(
            Arg::with_name("color")
                .long("color")
                .required(false)
                .takes_value(true)
                .possible_values(&["auto", "always", "never"])
                .default_value("auto")
                .help(
                    "Decides wheteher to use colors in log messages or not. \
                     `auto` should result with colors in terminal and no colors \
                     anywhere else",
                ),
        )
        .get_matches();
    let color_choice = matches.value_of("color").unwrap().parse()?;

    let formatter = FormatterBuilder::new().with_color(color_choice).build();

    flexi_logger::Logger::with_str("auto=trace")
        .format(formatter)
        .start()?;

    error!("This is an error example");
    warn!("This is a warning example");
    info!("This is an info example example");
    debug!("This is a debug example");
    trace!("This is a trace example");

    Ok(())
}
