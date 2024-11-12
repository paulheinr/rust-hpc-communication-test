use rust_hpc_communication_test::communicator::StdCommunicator;
use rust_hpc_communication_test::constants::MESSAGE;
use rust_hpc_communication_test::test_execution::ping_pong_client;

fn main() {
    let communicator = StdCommunicator::create_n_2_n(2, 0);
    ping_pong_client(10_000, communicator, MESSAGE);
}