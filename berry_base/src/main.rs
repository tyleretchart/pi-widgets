// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::env;

mod button;
mod communication;
mod light;

fn main() {
    let data = "{\"pin\": 18, \"duration_ms\": 800, \"period_ms\": 200}";
    let data = communication::connect(data.to_string());
    println!("result: {}", data);

    let light_args: light::LightArguments = serde_json::from_str(&data).unwrap();
}

// fn main() {
//     let data: &str = r#"{
//                            "pin": 18,
//                            "duration_ms": 800,
//                            "period_ms": 200
//                         }"#;
//     let light_args: light::LightArguments = serde_json::from_str(data).unwrap();

//     let args: Vec<String> = env::args().collect();
//     if args.len() != 2 {
//         println!("Usage: ./poll <pin>");
//     } else {
//         match args[1].parse::<u64>() {
//             Ok(pin) => match button::poll(pin, light_args) {
//                 Ok(()) => println!("Polling Complete!"),
//                 Err(err) => println!("Error: {}", err),
//             },
//             Err(_) => println!("Usage: ./poll <pin>"),
//         }
//     }
// }
