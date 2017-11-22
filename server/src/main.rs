extern crate ancestor;
extern crate env_logger;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use ancestor::Server;
use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[cfg_attr(rustfmt, rustfmt_skip)]
struct Arguments {
    #[structopt(short = "a", default_value = "127.0.0.1:1234")]
    /// IP address and port for listening to.
    address: SocketAddr,
    #[structopt(short = "n", default_value = "0")]
    /// Number of threads to create (0 for number of CPUs).
    num_threads: usize,
    #[structopt(short = "t", default_value = "5")]
    /// Timeout per task.
    timeout: usize,
}

fn main() {
    env_logger::init().unwrap();
    let arguments = Arguments::from_args();

    let server = Server::new(arguments.num_threads, arguments.timeout);

    server.start(arguments.address);
}
