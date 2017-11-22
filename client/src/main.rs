extern crate ancestor;
extern crate env_logger;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use ancestor::Client;
use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[cfg_attr(rustfmt, rustfmt_skip)]
struct Arguments {
    #[structopt(short = "a", default_value = "127.0.0.1:1234")]
    /// IP address and port of the server.
    address: SocketAddr,
    #[structopt(short = "n", default_value = "10")]
    /// Amount of requests to generate.
    num_requests: u32,
    #[structopt(short = "m", default_value = "100000")]
    /// Minimum border for generated numbers.
    range_min: usize,
    #[structopt(short = "x", default_value = "1000000")]
    /// Maximum border for generated numbers.
    range_max: usize,
}

fn main() {
    env_logger::init().unwrap();
    let arguments = Arguments::from_args();

    let client = Client::new(
        arguments.address,
        arguments.num_requests,
        (arguments.range_min, arguments.range_max),
    );
    client.start();
}
