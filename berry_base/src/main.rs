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

use serde_json::Error;

mod light;

#[derive(Serialize, Deserialize)]
struct Arguments {
    pin: u64,
    duration_ms: u64,
    period_ms: u64,
}

fn main_old() -> Result<(), Error> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data: &str = r#"{
                    "pin": 18,
                    "duration_ms": 800,
                    "period_ms": 200
                  }"#;

    // Parse the string of data into an Argument object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for an Argument as output.
    let args: Arguments = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!(
        "Pin: {}, Duration: {}ms, Period: {}ms",
        args.pin, args.duration_ms, args.period_ms
    );

    light::blink_led(args.pin, args.duration_ms, args.period_ms);

    Ok(())
}

// use std::env;

// fn print_usage() {
//     println!("Usage: ./blinky <pin> <duration_ms> <period_ms>");
// }

// fn get_args() -> Option<Arguments> {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 4 {
//         return None;
//     }
//     let pin = match args[1].parse::<u64>() {
//         Ok(pin) => pin,
//         Err(_) => return None,
//     };
//     let duration_ms = match args[2].parse::<u64>() {
//         Ok(ms) => ms,
//         Err(_) => return None,
//     };
//     let period_ms = match args[3].parse::<u64>() {
//         Ok(ms) => ms,
//         Err(_) => return None,
//     };
//     Some(Arguments {
//         pin: pin,
//         duration_ms: duration_ms,
//         period_ms: period_ms,
//     })
// }

// Get args from commandline
// fn main() {
//     match get_args() {
//         None => print_usage(),
//         Some(args) => light::blink_led(args.pin, args.duration_ms, args.period_ms),
//     }
// }

// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

extern crate sysfs_gpio;

use std::env;
use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

fn poll(pin_num: u64, light_args: Arguments) -> sysfs_gpio::Result<()> {
    // NOTE: this currently runs forever and as such if
    // the app is stopped (Ctrl-C), no cleanup will happen
    // and the GPIO will be left exported.  Not much
    // can be done about this as Rust signal handling isn't
    // really present at the moment.  Revisit later.

    let input = Pin::new(pin_num);
    input.with_exported(|| {
        sleep(Duration::from_millis(120));
	      input.set_direction(Direction::In)?;
        let mut prev_val: u8 = 255;
        loop {
            let val = input.get_value()?;
            // println!("Value: {}", val);
						// if val == 0 && prev_val == 0 {
						// if val != prev_val {
            if val == 1 {
						    println!("Pin State: {}", if val == 0 { "Low" } else { "High" });
                light::blink_led(light_args.pin, light_args.duration_ms, light_args.period_ms);
            }
            sleep(Duration::from_millis(10));
						prev_val = val;
        }
    })
}

fn main() {
    let data: &str = r#"{
                           "pin": 18,
                           "duration_ms": 800,
                           "period_ms": 200
                        }"#;
    let light_args: Arguments = serde_json::from_str(data).unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./poll <pin>");
    } else {
        match args[1].parse::<u64>() {
            Ok(pin) => match poll(pin, light_args) {
                Ok(()) => println!("Polling Complete!"),
                Err(err) => println!("Error: {}", err),
            },
            Err(_) => println!("Usage: ./poll <pin>"),
        }
    }
}
