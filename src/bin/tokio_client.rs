use clap::Parser;
use rust_hpc_communication_test::communicator::TokioCommunicator;
use rust_hpc_communication_test::test_execution::{TestArguments, TestExecution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = TestArguments::parse();
    let communicator = TokioCommunicator::create_n_2_n(2, 0);
    let test_execution = TestExecution::new(communicator, args);
    test_execution.ping_pong_client();
    Ok(())
}
