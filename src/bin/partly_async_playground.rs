// Cargo.toml dependencies:
// tokio = { version = "1", features = ["full"] }
// tonic = "0.10"
// futures = "0.3"
// chrono = "0.4"

use std::sync::{mpsc, Arc, Barrier};
use std::thread;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::TryRecvError;
use chrono::Local;

// Dummy payload and response types
#[derive(Debug)]
struct Payload(pub u32);
#[derive(Debug)]
struct GrpcResponse(pub String);

#[derive(Debug)]
struct MyGrpcRequest {
    payload: Payload,
    response_tx: oneshot::Sender<GrpcResponse>,
}

fn main() {
    let (grpc_tx, mut grpc_rx) = tokio::sync::mpsc::channel::<MyGrpcRequest>(100);

    let (shutdown_send, mut shutdown_recv) = tokio::sync::watch::channel(false);

    let barrier = Arc::new(Barrier::new(4)); // 1 for main thread, 3 for worker threads

    // gRPC handler task (simulated with delay)
    let grpc_handler = thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let adapter = Adapter{};

        rt.block_on(async move {
            // Inside the gRPC handler's async block:
            loop {
                tokio::select! {
                    _ = shutdown_recv.changed() => {
                        if *shutdown_recv.borrow() {
                            println!("{} gRPC handler shutdown signal received, exiting.", Local::now().format("[%Y-%m-%d %H:%M:%S%.6f]"));
                            break;
                        }
                    }
                    maybe_req = grpc_rx.recv() => {
                        if let Some(req) = maybe_req {
                            let mut clone = adapter.clone();
                            tokio::spawn(async move {
                                let resp = clone.simulate_grpc_call(req.payload).await;
                                let _ = req.response_tx.send(resp);
                            });
                        }
                    }
                }
            }
        })
    });

    // Simulate starting multiple threads
    let mut handles = Vec::new();
    for i in 0..3 {
        let tx = grpc_tx.clone();
        let barrier = barrier.clone();
        handles.push(thread::spawn(move || {
            heavy_work(i);

            // Each thread creates a request
            let payload = Payload(i);
            let (resp_tx, mut resp_rx) = oneshot::channel();

            println!("{} [Thread {}] Sending payload: {:?}", Local::now().format("[%Y-%m-%d %H:%M:%S%.6f]"), i, payload);
            tx.blocking_send(MyGrpcRequest {
                payload,
                response_tx: resp_tx,
            })
                .unwrap();
            println!("{} [Thread {}] Sent payload.", Local::now().format("[%Y-%m-%d %H:%M:%S%.6f]"), i);

            let mut recv = false;
            while !recv {
                heavy_work(i);
                // Now ready to receive the result
                match resp_rx.try_recv() {
                    // _ => {}
                    Ok(r) => {
                        println!("{} [Thread {}] Got response: {:?}", Local::now().format("[%Y-%m-%d %H:%M:%S%.6f]"), i, r);
                        recv = true;
                    }
                    Err(e) => match e {
                        TryRecvError::Empty => {}, //println!("[Thread {}] Error: Empty", i),
                        TryRecvError::Closed => {}, //println!("[Thread {}] Error: Closed", i),
                    },
                }
            }
            barrier.wait();
        }));
    }

    barrier.wait(); // Simulate some initial setup
    shutdown_send.send(true).unwrap();

    for handle in handles {
        handle.join().unwrap();
    }
    grpc_handler.join().unwrap();
    // println!("Status of grpc handler: {:?}", grpc_handler.is_finished());
}

fn heavy_work(i: u32) {
    println!("{} [Thread {}] Doing heavy work...", Local::now().format("[%Y-%m-%d %H:%M:%S%.6f]"), i);
    thread::sleep(Duration::from_millis(500));
    println!("{} [Thread {}] Finished heavy work, waiting for response...", Local::now().format("[%Y-%m-%d %H:%M:%S%.6f]"), i);
}

#[derive(Clone)]
struct Adapter {}

impl Adapter {
    // Simulate an async gRPC call
    async fn simulate_grpc_call(&mut self, payload: Payload) -> GrpcResponse {
        println!("{} [gRPC Handler] Received payload: {:?}", Local::now().format("[%Y-%m-%d %H:%M:%S%.6f]"), payload);
        tokio::time::sleep(Duration::from_millis((payload.0 * 400) as u64)).await;
        let resp = GrpcResponse(format!("Hello from gRPC for id {}", payload.0));
        println!("{} [gRPC Handler] Sending response: {:?}", Local::now().format("[%Y-%m-%d %H:%M:%S%.6f]"), resp);
        resp
    }
}