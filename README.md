# Colored logging output for flexi_logger using colored

## How to use

```rust

#[macro_use]
extern crate log;
extern crate flexi_logger;
extern crate colored_logger;

use colored_logger::formatter;

fn main() {
   flexi_logger::LogOptions::new()
       .format(formatter)
       .init(Some("info".to_string()))
       .unwrap();

   info!("This is a test message");
   error!("Error!");
}

```
