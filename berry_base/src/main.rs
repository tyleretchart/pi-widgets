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
    // let data = communication::connect(data.to_string());
    println!("result: {}", data);

    let light_args: light::LightArguments = serde_json::from_str(&data).unwrap();

	  loop {
        let code_change_pressed = button::poll(26);
        if code_change_pressed {
				    light::blink_led(light_args.pin, light_args.duration_ms, light_args.period_ms);
						code_change_pressed = false;
				}
				let light_activate_pressed = button::poll(27);
				if light_activate_pressed {
						light::blink_led(light_args.pin, light_args.duration_ms, light_args.period_ms);
						light_activate_pressed = false;
			  }
    }
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

// ========================================================================================

// Thread Stuff

// use std::sync::Arc;
// use std::sync::Mutex;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let mut foo: Vec<i32> = Vec::new();
//     foo.push(34);
//     foo.push(56);

//     let data = Arc::new(Mutex::new(foo));

//     for i in 0..5 {
//         let dd = data.clone();
//         let ddd = data.clone();
//         let index = i;

//         thread::spawn(move || {
//             println!("spawned consumer thread {}", index);

//             loop {
//                 let mut d = dd.lock().unwrap();

//                 if d.len() == 0 {
//                     println!("no work for thread {}, sleeping", i);
//                     thread::sleep(Duration::from_secs(1));
//                 } else {
//                     let x: i32 = d.pop().unwrap();

//                     println!("thread {} has work!  {}", index, x);
//                     thread::sleep(Duration::from_secs(x as u64));
//                     println!("thread {} work complete!", index);
//                 }
//             }
//         });

//         thread::spawn(move || {
//             println!("spawned producer thread {}", index);

//             loop {
//                 let mut d = ddd.lock().unwrap();

//                 if d.len() == 0 {
//                     println!("thread {} has work!", index);
//                     d.push(34);
//                     println!("thread {} work complete!", index);
//                 } else {
//                     println!("no work for thread {}, sleeping", i);
//                     thread::sleep(Duration::from_secs(1));
//                 }
//             }
//         });
//     }
// }
