use std::io;

use futures::Sink;
use futures::Stream;
use futures::future::Future;
use futures::stream;
use rand::{self, Rng};
use std::net::SocketAddr;
use tokio_core::reactor::Core;
use tokio_io::AsyncRead;

use tokio_core::net::TcpStream;

use Request;

mod protocol;

/// Async client. It sends to the server bunch of random-generated [`Request`]s in simple json
/// format and then reads and prints responses.
///
/// [`Request`]: ../struct.Request.html
pub struct Client {
    address: SocketAddr,
    num_requests: u32,
    range: (usize, usize),
}

impl Client {
    /// Create new `Client` with desired settings.
    pub fn new(address: SocketAddr, num_requests: u32, range: (usize, usize)) -> Self {
        Self {
            address,
            num_requests,
            range,
        }
    }

    /// Starts `Client`, sends random-generated requests to the server, reads and prints all the
    /// responses.
    pub fn start(self) {
        let mut core = Core::new().unwrap();

        let mut rng = rand::thread_rng();

        let mut responses = self.num_requests;
        let requests = self.num_requests;

        let client_future = TcpStream::connect(&self.address, &core.handle())
            .and_then(|socket| {
                debug!("Connection established, sending requests");

                let transport = socket.framed(protocol::Codec);

                // generate requests
                let requests = (0 .. requests).map(|_| {
                    Ok(Request {
                        number: rng.gen_range(self.range.0 as u64, self.range.1 as u64),
                    }) as Result<_, io::Error>
                });

                // send them
                transport.send_all(stream::iter_result(requests))
            })
            .and_then(|(socket, _)| {
                debug!("Processing responses");

                socket.for_each(|response| {
                    // print both in log and in stdout
                    info!("Response #{}: {:?}", requests - responses, response);
                    println!("Response #{}: {:?}", requests - responses, response);

                    // we're counting number of responses in order to exit on the last one
                    responses -= 1;
                    if responses == 0 {
                        info!("All responses processed");
                        // dirty hack: we send Err when all responses acquired in order to stop
                        // listening for the stream
                        // TODO: fix it somehow?
                        Err(io::Error::new(
                            io::ErrorKind::Other,
                            "All responses processed, it's not an error",
                        ))
                    } else {
                        Ok(())
                    }
                })
            });

        // run our future and shadow fake error
        core.run(client_future)
            .or_else(|err| match err.kind() {
                io::ErrorKind::Other => Ok(()),
                _ => Err(err),
            })
            .unwrap();
    }
}
