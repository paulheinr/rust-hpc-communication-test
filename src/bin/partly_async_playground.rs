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
    let (grpc_tx, grpc_rx) = mpsc::channel::<MyGrpcRequest>();

    // gRPC handler task (simulated with delay)
    let grpc_handler = thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async move {
            // Use a Vec to keep track of JoinHandles if you want to await them later (optional)
            let mut tasks = Vec::new();
            while let Ok(req) = grpc_rx.try_recv() {
                println!("[gRPC Handler] Received payload: {:?}", req.payload);
                // Spawn a new task for each request
                let task = tokio::spawn(async move {
                    let resp = simulate_grpc_call(req.payload).await;
                    println!("[gRPC Handler] Sending response: {:?}", resp);
                    let _ = req.response_tx.send(resp);
                });
                tasks.push(task);
            }
            // Optionally, wait for all tasks to finish before exiting
            for t in tasks {
                let _ = t.await;
            }
        })
    });

    // Simulate starting multiple threads
    let mut handles = Vec::new();
    for i in 0..3 {
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

            let mut recv = false;
            while !recv {
                heavy_work(i);
                // Now ready to receive the result
                match resp_rx.try_recv() {
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
    println!("Status of grpc handler: {:?}", grpc_handler.is_finished());
}

fn heavy_work(i: u32) {
    println!("[Thread {}] Doing heavy work...", i);
    thread::sleep(Duration::from_millis(500));
    println!("[Thread {}] Finished heavy work, waiting for response...", i);
}

// Simulate an async gRPC call
async fn simulate_grpc_call(payload: Payload) -> GrpcResponse {
    tokio::time::sleep(Duration::from_millis((payload.0 * 100) as u64)).await;
    GrpcResponse(format!("Hello from gRPC for id {}", payload.0))
}
