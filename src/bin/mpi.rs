use clap::Parser;
use rust_hpc_communication_test::communicator::{MpiCommunicator, TestCommunicator};
use rust_hpc_communication_test::test_execution::{TestArguments, TestExecution};

fn main() {
    let args = TestArguments::parse();
    let universe = mpi::initialize().unwrap();
    let comm = universe.world();

    let communicator = MpiCommunicator::create(comm);
    let rank = communicator.rank();
    let test_execution = TestExecution::new(communicator, args);

    test_execution.barrier();
    if rank == 0 {
        test_execution.ping_pong_client();
    } else {
        test_execution.ping_pong_server();
    }
}
