use std::thread;
use std::thread::JoinHandle;

use rust_hpc_communication_test::communicator::ChannelSimCommunicator;
use rust_hpc_communication_test::test_execution::TestExecution;

fn main() {
    let iter = 10_000;
    let message = b"Hello, World!";

    let comms = ChannelSimCommunicator::create_n_2_n(2);

    let handles: Vec<JoinHandle<()>> = comms.into_iter().enumerate().map(|(i, comm)| {
        if i == 0 {
            thread::Builder::new().name(i.to_string()).spawn(move || {
                TestExecution::new(comm, iter, 1000).ping_pong_client(message);
            }).expect("Failed to spawn thread.")
        } else {
            thread::Builder::new().name(i.to_string()).spawn(move || {
                TestExecution::new(comm, iter, 1000).ping_pong_server(message.len());
            }).expect("Failed to spawn thread.")
        }
    }).collect();

    for handle in handles {
        handle.join().expect("Failed to join thread.");
    }
}