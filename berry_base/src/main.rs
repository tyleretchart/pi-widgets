// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

// extern crate serde;
// extern crate serde_json;

// #[macro_use]
// extern crate serde_derive;

// use std::env;

// mod button;
// mod communication;
// mod light;

// fn main() {
//     let data = "{\"pin\": 18, \"duration_ms\": 800, \"period_ms\": 200}";
//     let data = communication::connect(data.to_string());
//     println!("result: {}", data);

//     let light_args: light::LightArguments = serde_json::from_str(&data).unwrap();
// }

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

// ========================================================================================

#[cfg(feature = "tokio")]
extern crate futures;
#[cfg(feature = "tokio")]
extern crate sysfs_gpio;
#[cfg(feature = "tokio")]
extern crate tokio_core;

#[cfg(feature = "tokio")]
use futures::{Future, Stream};
#[cfg(feature = "tokio")]
use sysfs_gpio::{Direction, Edge, Pin};
#[cfg(feature = "tokio")]
use std::env;
#[cfg(feature = "tokio")]
use tokio_core::reactor::Core;

#[cfg(feature = "tokio")]
fn stream(pin_nums: Vec<u64>) -> sysfs_gpio::Result<()> {
    // NOTE: this currently runs forever and as such if
    // the app is stopped (Ctrl-C), no cleanup will happen
    // and the GPIO will be left exported.  Not much
    // can be done about this as Rust signal handling isn't
    // really present at the moment.  Revisit later.
    let pins: Vec<_> = pin_nums.iter().map(|&p| (p, Pin::new(p))).collect();
    let mut l = Core::new()?;
    let handle = l.handle();
    for &(i, ref pin) in pins.iter() {
        pin.export()?;
        pin.set_direction(Direction::In)?;
        pin.set_edge(Edge::BothEdges)?;
        handle.spawn(pin.get_value_stream(&handle)?
                         .for_each(move |val| {
                                       println!("Pin {} changed value to {}", i, val);
                                       Ok(())
                                   })
                         .map_err(|_| ()));
    }
    // Wait forever for events
    loop {
        l.turn(None)
    }
}

#[cfg(feature = "tokio")]
fn main() {
    let pins: Vec<u64> = env::args()
        .skip(1)
        .map(|a| a.parse().expect("Pins must be specified as integers"))
        .collect();
    if pins.is_empty() {
        println!("Usage: ./tokio <pin> [pin ...]");
    } else {
        stream(pins).unwrap();
    }
}

#[cfg(not(feature = "tokio"))]
fn main() {
    println!("This example requires the `tokio` feature to be enabled.");
}