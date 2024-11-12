use rust_hpc_communication_test::communicator::StdCommunicator;
use rust_hpc_communication_test::constants::MESSAGE;
use rust_hpc_communication_test::test_execution::ping_pong_server;

fn main() {
    let communicator = StdCommunicator::create_n_2_n(2, 1);
    ping_pong_server(10_000, communicator, MESSAGE.len());
}