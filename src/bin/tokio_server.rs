use clap::Parser;
use rust_hpc_communication_test::communicator::TokioCommunicator;
use rust_hpc_communication_test::test_execution::{
    BasicArguments, TestExecution,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = BasicArguments::parse();
    let communicator = TokioCommunicator::create_n_2_n(2, 1);
    let test_execution = TestExecution::new(communicator, args);
    test_execution.ping_pong_server();
    Ok(())
}
