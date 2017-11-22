use std::io;
use std::time::{Duration, Instant};

use futures::Future;
use futures_cpupool::CpuPool;
use std::net::SocketAddr;
use tokio_proto::TcpServer;
use tokio_service::Service;
use tokio_timer::Timer;

use {Request, Response};

mod protocol;

/// Async server. It gets [`Request`]s from the client, processes them in thread pool
/// and returns [`Response`]s.
///
/// [`Request`]: ../struct.Request.html
/// [`Response`]: ../enum.Response.html
pub struct Server {
    cpu_pool: CpuPool,
    timeout: Duration,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            cpu_pool: CpuPool::new_num_cpus(),
            timeout: Duration::from_secs(5),
        }
    }
}

impl Server {
    /// Create new server with desired number of threads in the pool and timeout per task.
    pub fn new(num_cpus: usize, timeout_secs: usize) -> Self {
        Self {
            cpu_pool: if num_cpus == 0 {
                CpuPool::new_num_cpus()
            } else {
                CpuPool::new(num_cpus)
            },
            timeout: Duration::from_secs(timeout_secs as u64),
        }
    }

    /// Start server. Will listen for incoming connections and process them.
    pub fn start(self, address: SocketAddr) {
        let tcp_server = TcpServer::new(protocol::Protocol, address);
        tcp_server.serve(move || {
            info!("New connection received, processing...");
            Ok(Worker {
                cpu_pool: self.cpu_pool.clone(),
                timeout: self.timeout,
                timer: Timer::default(),
            })
        });
    }
}

// Our internal service
struct Worker {
    cpu_pool: CpuPool,
    timeout: Duration,
    timer: Timer,
}

impl Service for Worker {
    type Request = Request;
    type Response = Response;

    type Error = io::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        debug!("New request received: {:?}", req);

        // Create timeout future that return [`Response::Failure`](enum.Response.html)
        // when timer expires.
        // We need to box it in order to provide type information for compiler.
        let timeout: Self::Future = Box::new(
            self.timer
                .sleep(self.timeout)
                .map_err(|err| {
                    error!("Timer error: {:?}", err);
                    io::Error::new(io::ErrorKind::Other, "Timer error")
                })
                .then(|_| {
                    debug!("Timeout triggered, sending response back");
                    Ok(Response::Failure)
                }),
        );

        // Save current time
        let start_time = Instant::now();

        // Main future, will count from 0 to N representing some computation.
        let counter = self.cpu_pool.spawn_fn(move || {
            let mut _i = 0 as u64;
            for _ in 0 .. req.number {
                _i += 1;
            }

            let elapsed = start_time.elapsed();
            debug!(
                "Successfully processed, sending response back: {:?}",
                elapsed
            );

            // return elapsed time in seconds
            Ok(Response::Success(
                elapsed.as_secs() as f64 + f64::from(elapsed.subsec_nanos()) * 1e-9,
            ))
        });

        // combine counter future with timeout future via select.
        // will return the first finished future
        let winner = timeout
            .select(counter)
            .map(|(win, _)| win)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Timeout error"));

        Box::new(winner)
    }
}
