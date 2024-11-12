use rust_hpc_communication_test::communicator::TokioCommunicator;
use rust_hpc_communication_test::constants::MESSAGE;
use rust_hpc_communication_test::test_execution::ping_pong_client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let communicator = TokioCommunicator::create_n_2_n(2, 0);
    ping_pong_client(10_000, communicator, MESSAGE);
    Ok(())
}
