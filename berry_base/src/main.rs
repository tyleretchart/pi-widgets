// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;

mod light;

struct Arguments {
    pin: u64,
    duration_ms: u64,
    period_ms: u64,
}

fn print_usage() {
    println!("Usage: ./blinky <pin> <duration_ms> <period_ms>");
}

fn get_args() -> Option<Arguments> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        return None;
    }
    let pin = match args[1].parse::<u64>() {
        Ok(pin) => pin,
        Err(_) => return None,
    };
    let duration_ms = match args[2].parse::<u64>() {
        Ok(ms) => ms,
        Err(_) => return None,
    };
    let period_ms = match args[3].parse::<u64>() {
        Ok(ms) => ms,
        Err(_) => return None,
    };
    Some(Arguments {
        pin: pin,
        duration_ms: duration_ms,
        period_ms: period_ms,
    })
}

fn main() {
    match get_args() {
        None => print_usage(),
        Some(args) => light::blink_led(args.pin, args.duration_ms, args.period_ms),
    }
}
