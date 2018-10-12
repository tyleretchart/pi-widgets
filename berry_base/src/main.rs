// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate tokio_codec;

#[macro_use]
extern crate serde_derive;

use tokio::codec::Decoder;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio_codec::BytesCodec;

use std::env;
use std::net::SocketAddr;
use std::thread;

mod button;
mod communication;
mod light;

fn main() {
    // collect addresses
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./run <ip:port>");
    }
    let gui_address = env::args().nth(1).unwrap();
    let self_address = env::args().nth(2).unwrap_or("0.0.0.0:5002".to_string());
    let self_address = self_address.parse::<SocketAddr>().unwrap();

    // default data for button
    let mut data = String::from(
        "{\"host\": \"boysenberry:5002\", \"pin\": 18, \"duration_ms\": 800, \"period_ms\": 200}",
    );

    // set up tcp listener
    let socket = TcpListener::bind(&self_address).unwrap();
    println!("Listening on: {}", self_address);

    // poll buttons
    thread::spawn(move || loop {
        // poll buttons
        let code_change_pressed = button::poll(26);
        println!("CODE CHANGE BUTTON: {}", code_change_pressed);
        if code_change_pressed {
            let data_tmp = communication::connect_with_gui(&gui_address, data.to_string());
            data = String::from(data_tmp);
        }
        let light_activate_pressed = button::poll(27);
        println!("LIGHT ACTIVATE BUTTON: {}", light_activate_pressed);
        if light_activate_pressed {
            let light_args: light::LightArguments = serde_json::from_str(&data).unwrap();
            communication::connect_with_light(&light_args.host, data.to_string());
            // light::blink_led(light_args);
        }
        println!("");
    });

    // set up tokio socket function
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
                    light::blink_led(light_args);
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
