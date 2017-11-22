//! Experimental library that provides ready-to-use async `Server` and `Client` written with
//! `tokio-rs`.
//!
//! Created for education purposes only, it will never be published as a crate.
//!
//! #### Wat? `Ancestor`? What does it mean?
//! I love to use acronym generator for my libs' names, so in `Ancestor` you can probably find
//! some letters from words "rust", "server", "client" and "async".


extern crate bytes;
extern crate futures;
extern crate futures_cpupool;
#[macro_use]
extern crate log;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_timer;

use bytes::BytesMut;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::io;

pub mod server;
pub mod client;

/// Response type.
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    /// Computation has been finished successfully in given time (in seconds).
    Success(f64),
    /// Computation has not been finished before the timeout fired.
    Failure,
}

/// Request type.
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    /// Some number that represents the complexity of fake computation.
    pub number: u64,
}

pub use client::Client;
pub use server::Server;

fn decode_message<T: DeserializeOwned>(bytes: &mut BytesMut) -> Result<Option<T>, io::Error> {
    if let Some(i) = bytes.iter().position(|&b| b == b'\n') {
        // remove the serialized frame from the buffer.
        let line = bytes.split_to(i);

        // Also remove the '\n'
        bytes.split_to(1);

        // Deserialize data from binary format
        serde_json::from_slice(&line).map(Some).map_err(|err| {
            error!("Deserialize error: {:?}", err);
            io::Error::new(io::ErrorKind::Other, "Deserialize error")
        })
    } else {
        Ok(None)
    }
}

fn encode_message<T: Serialize>(msg: T, bytes: &mut BytesMut) -> Result<(), io::Error> {
    let string = serde_json::to_vec(&msg).map_err(|err| {
        error!("Serialize error: {:?}", err);
        io::Error::new(io::ErrorKind::Other, "Serialize error")
    })?;
    bytes.extend(string);
    bytes.extend(b"\n");
    Ok(())
}
