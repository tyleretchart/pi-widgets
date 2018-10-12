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

// fn main() {
//     // collect address
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 2 {
//         println!("Usage: ./run <ip:port>");
//     }
//     let address: String = args[1].parse().unwrap();

//     // default data
//     let mut data = String::from("{\"pin\": 18, \"duration_ms\": 800, \"period_ms\": 200}");

//     // poll buttons
//     loop {
//         let code_change_pressed = button::poll(26);
//         println!("CODE CHANGE BUTTON: {}", code_change_pressed);
//         if code_change_pressed {
//             let data_tmp = communication::connect(&address, data.to_string());
// 			data = String::from(data_tmp);
//         }
//         let light_activate_pressed = button::poll(27);
// 				println!("LIGHT ACTIVATE BUTTON: {}", light_activate_pressed);
//         if light_activate_pressed {
//             let light_args: light::LightArguments = serde_json::from_str(&data).unwrap();
//             light::blink_led(light_args.pin, light_args.duration_ms, light_args.period_ms);
//         }
// 				println!("");
//     }
// }

// ========================================================================================

// #![deny(warnings)]

extern crate tokio;
extern crate tokio_codec;

use tokio_codec::BytesCodec;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::codec::Decoder;

// use std::env;
use std::net::SocketAddr;

fn main() {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    let addr = env::args().nth(2).unwrap_or("0.0.0.0:5002".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop, so we pass in a handle
    // to our event loop. After the socket's created we inform that we're ready
    // to go and start accepting connections.
    let socket = TcpListener::bind(&addr).unwrap();
    println!("Listening on: {}", addr);

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method. We then define how to process each element in
    // the stream with the `for_each` method.
    //
    // This combinator, defined on the `Stream` trait, will allow us to define a
    // computation to happen for all items on the stream (in this case TCP
    // connections made to the server).  The return value of the `for_each`
    // method is itself a future representing processing the entire stream of
    // connections, and ends up being our server.
    let done = socket
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            // Once we're inside this closure this represents an accepted client
            // from our server. The `socket` is the client connection (similar to
            // how the standard library operates).
            //
            // We're parsing each socket with the `BytesCodec` included in `tokio_io`,
            // and then we `split` each codec into the reader/writer halves.
            //
            // See https://docs.rs/tokio-codec/0.1/src/tokio_codec/bytes_codec.rs.html
            let framed = BytesCodec::new().framed(socket);
            let (_writer, reader) = framed.split();

            let processor = reader
                .for_each(|bytes| {
                    println!("bytes: {:?}", bytes);
                    println!("string: {:?}", String::from_utf8(bytes.to_vec()).expect("Found invalid UTF-8").trim());

                    let data = String::from_utf8(bytes.to_vec()).expect("Found invalid UTF-8");
                    let light_args: light::LightArguments = serde_json::from_str(&data.trim()).unwrap();
                    light::blink_led(light_args.pin, light_args.duration_ms, light_args.period_ms);
                    Ok(())
                })
                // After our copy operation is complete we just print out some helpful
                // information.
                .and_then(|()| {
                    println!("Socket received FIN packet and closed connection");
                    Ok(())
                })
                .or_else(|err| {
                    println!("Socket closed with error: {:?}", err);
                    // We have to return the error to catch it in the next ``.then` call
                    Err(err)
                })
                .then(|result| {
                    println!("Socket closed with result: {:?}", result);
                    Ok(())
                });

            // And this is where much of the magic of this server happens. We
            // crucially want all clients to make progress concurrently, rather than
            // blocking one on completion of another. To achieve this we use the
            // `tokio::spawn` function to execute the work in the background.
            //
            // This function will transfer ownership of the future (`msg` in this
            // case) to the Tokio runtime thread pool that. The thread pool will
            // drive the future to completion.
            //
            // Essentially here we're executing a new task to run concurrently,
            // which will allow all of our clients to be processed concurrently.
            tokio::spawn(processor)
        });

    // And finally now that we've define what our server is, we run it!
    //
    // This starts the Tokio runtime, spawns the server task, and blocks the
    // current thread until all tasks complete execution. Since the `done` task
    // never completes (it just keeps accepting sockets), `tokio::run` blocks
    // forever (until ctrl-c is pressed).
    tokio::run(done);
}

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
