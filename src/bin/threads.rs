use clap::Parser;
use rust_hpc_communication_test::communicator::ChannelSimCommunicator;
use rust_hpc_communication_test::test_execution::{BasicArguments, TestExecution};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let args = BasicArguments::parse();

    let comms = ChannelSimCommunicator::create_n_2_n(2);

    let handles: Vec<JoinHandle<()>> = comms
        .into_iter()
        .enumerate()
        .map(|(i, comm)| {
            if i == 0 {
                thread::Builder::new()
                    .name(i.to_string())
                    .spawn({
                        let a = args.clone();
                        move || {
                            TestExecution::new(comm, a).ping_pong_client();
                        }
                    })
                    .expect("Failed to spawn thread.")
            } else {
                thread::Builder::new()
                    .name(i.to_string())
                    .spawn({
                        let a = args.clone();
                        move || {
                            TestExecution::new(comm, a).ping_pong_server();
                        }
                    })
                    .expect("Failed to spawn thread.")
            }
        })
        .collect();

    for handle in handles {
        handle.join().expect("Failed to join thread.");
    }
}
