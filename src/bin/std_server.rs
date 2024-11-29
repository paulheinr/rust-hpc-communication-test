use clap::Parser;
use rust_hpc_communication_test::communicator::StdCommunicator;
use rust_hpc_communication_test::test_execution::{BasicArguments, TestExecution};

fn main() {
    let args = BasicArguments::parse();
    let communicator = StdCommunicator::create_n_2_n(2, 1);
    let test_execution = TestExecution::new(communicator, args);
    test_execution.ping_pong_server();
}
