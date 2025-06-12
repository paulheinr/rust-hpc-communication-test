// Cargo.toml dependencies:
// tokio = { version = "1", features = ["full"] }
// tonic = "0.10"
// futures = "0.3"

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::sink;
use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::channel;
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

#[derive(Debug)]
struct MyGrpcArrivedRequest {
    payload: Payload,
}

fn main() {
    let thread_count = 3;

    let (grpc_req_send, grpc_req_recv) = mpsc::channel::<MyGrpcRequest>();
    let (grpc_arr_req_send, grpc_arr_req_recv) = mpsc::channel::<MyGrpcArrivedRequest>();

    let mut grpc_res_send_vec = Vec::new();
    let mut grpc_res_recv_vec = HashMap::new();
    for i in 0..thread_count {
        let (grpc_res_send, grpc_res_recv) = mpsc::channel::<GrpcResponse>();
        grpc_res_send_vec.push(grpc_res_send);
        grpc_res_recv_vec.insert(i, grpc_res_recv);
    }
    
    // gRPC handler task (simulated with delay)
    let grpc_handler = thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async move {
            let responses = Arc::new(Mutex::new(HashMap::new()));
            loop {
                // Use a Vec to keep track of JoinHandles if you want to await them later (optional)
                let mut tasks = Vec::new();
                
                while let Ok(req) = grpc_req_recv.try_recv() {
                    println!("[gRPC Handler] Received payload: {:?}", req.payload);

                    let res = Arc::clone(&responses);
                    // Spawn a new task for each request
                    let task = tokio::spawn(async move {
                        let from = req.payload.0;
                        let resp = simulate_grpc_call(req.payload).await;
                        println!("[gRPC Handler] Storing response: {:?}", resp);
                        res.lock().unwrap().insert(from, resp);
                        // let _ = req.response_tx.send(resp);
                    });
                    tasks.push(task);
                }

                while let Ok(arr_req) = grpc_arr_req_recv.try_recv() {
                    println!("[gRPC Handler] Received arrived request: {:?}", arr_req.payload);
                    let res = Arc::clone(&responses);
                    let to = arr_req.payload.0;
                    if let Some(r) = res.lock().unwrap().remove(&to) {
                        println!("[gRPC Handler] Found response for arrived request: {:?}", r);
                        grpc_res_send_vec.get(to as usize).unwrap().send(r).unwrap();
                    } else {
                        println!("[gRPC Handler] No response found for arrived request: {:?}", arr_req.payload);
                    };
                }

                // Optionally, wait for all tasks to finish before exiting
                for t in tasks {
                    let _ = t.await;
                }
            }            
        })
    });

    // Simulate starting multiple threads
    let mut handles = Vec::new();
    for i in 0..thread_count {
        let tx = grpc_req_send.clone();
        let grpc_arr_req_send = grpc_arr_req_send.clone();
        let x = grpc_res_recv_vec.remove(&i).unwrap();
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
            
            let response = loop {
                println!("[Thread {}] Sending ArrivedRequest", i);
                grpc_arr_req_send.send(MyGrpcArrivedRequest { payload: Payload(i) }).unwrap();
                heavy_work(i);
                let result = x.try_recv();
                match &result {
                    Ok(response) => { println!("[Thread {}] Received response: {:?}", i, response); break result.unwrap() },
                    Err(e) => println!("[Thread {}] Error receiving response: {:?}", i, e),
                }
            };
            
            println!("[Thread {}] Got response: {:?}", i, response);
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
    println!("[gRPC Handler] Simulating gRPC call for payload: {:?}", payload);
    tokio::time::sleep(Duration::from_millis((payload.0 * 100) as u64)).await;
    println!("[gRPC Handler] Simulated gRPC call completed for payload: {:?}", payload);
    GrpcResponse(format!("Hello from gRPC for id {}", payload.0))
}
