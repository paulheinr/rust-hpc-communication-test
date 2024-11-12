use rust_hpc_communication_test::communicator::TokioCommunicator;
use rust_hpc_communication_test::constants::MESSAGE;
use rust_hpc_communication_test::test_execution::TestExecution;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let communicator = TokioCommunicator::create_n_2_n(2, 0);
    let test_execution = TestExecution::new(communicator, 10_000, 1000);
    test_execution.ping_pong_client(MESSAGE);
    Ok(())
}
