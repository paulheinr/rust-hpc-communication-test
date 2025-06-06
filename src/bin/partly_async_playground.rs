// Cargo.toml dependencies:
// tokio = { version = "1", features = ["full"] }
// tonic = "0.10"
// futures = "0.3"

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::TryRecvError;

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
    let (grpc_tx, mut grpc_rx) = mpsc::channel::<MyGrpcRequest>();

    // gRPC handler task (simulated with delay)
    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async move {while let Ok(req) = grpc_rx.recv() {
            println!("[gRPC Handler] Received payload: {:?}", req.payload);
            let resp = simulate_grpc_call(req.payload).await;
            let _ = req.response_tx.send(resp);
        }})
    });

    // Simulate starting multiple threads
    let mut handles = Vec::new();
    for i in 0..4 {
        let tx = grpc_tx.clone();
        handles.push(thread::spawn(move || {
            // Each thread creates a request
            let payload = Payload(i);
            let (resp_tx, mut resp_rx) = oneshot::channel();

            println!("[Thread {}] Sending payload: {:?}", i, payload);
            tx.send(MyGrpcRequest {
                payload,
                response_tx: resp_tx,
            })
                .unwrap();
            println!("[Thread {}] Sent payload.", i);

            // Do some heavy work
            println!("[Thread {}] Doing heavy work...", i);
            thread::sleep(Duration::from_millis(500));

            let mut recv = false;
            while !recv {
                // Now ready to receive the result
                match resp_rx.try_recv() {
                    // Ok(Some(response)) => println!("[Thread {}] Got response: {:?}", i, response),
                    // Ok(None) => println!("[Thread {}] Got response: {:?}", i, response)
                    // Err(e) => println!("[Thread {}] Error receiving response: {}", i, e),
                    // _ => {}
                    Ok(r) => {
                        println!("[Thread {}] Got response: {:?}", i, r);
                        recv = true;
                    }
                    Err(e) => match e {
                        TryRecvError::Empty => {}, //println!("[Thread {}] Error: Empty", i),
                        TryRecvError::Closed => {}, //println!("[Thread {}] Error: Closed", i),
                    },
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// Simulate an async gRPC call
async fn simulate_grpc_call(payload: Payload) -> GrpcResponse {
    tokio::time::sleep(Duration::from_millis(300)).await;
    GrpcResponse(format!("Hello from gRPC for id {}", payload.0))
}
