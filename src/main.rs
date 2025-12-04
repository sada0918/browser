#![no_std]
#![no_main]

use noli::prelude::*;

fn main() {
    Api::write_string("Hello World\n");
    println!("Hello from println!");
    Api::exit(42);
}

entry_point!(main);
