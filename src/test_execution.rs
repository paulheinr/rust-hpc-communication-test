use derive_builder::Builder;

use crate::communicator::Communicator;

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct TestExecution<C> {
    communicator: C,
    iterations: u32,
    log_interval: u32,
}

impl<C: Communicator> TestExecution<C> {
    pub fn new(communicator: C, iterations: u32, log_interval: u32) -> Self {
        TestExecution {
            communicator,
            iterations,
            log_interval,
        }
    }

    pub fn ping_pong_client(&self, message: &[u8]) {
        self.check_ping_pong();
        let other = 1;

        //Measure elapsed time
        let start = std::time::Instant::now();
        for i in 0..self.iterations {
            if i % 1000 == 0 {
                println!("=== Client in iteration {} ===", i);
            }
            self.communicator.send(message, other);
            let in_buffer = &mut vec![0; message.len()];
            self.communicator.recv(in_buffer, other);
        }
        let elapsed = start.elapsed();
        println!("Elapsed time: {:?}", elapsed);
    }

    pub fn ping_pong_server(&self, length: usize) {
        self.check_ping_pong();
        let other = 0;

        for i in 0..self.iterations {
            if i % 1000 == 0 {
                println!("=== Server in iteration {} ===", i);
            }
            let in_buffer = &mut vec![0; length];
            self.communicator.recv(in_buffer, other);
            self.communicator.send(in_buffer, other);
        }
    }

    fn check_ping_pong(&self) {
        if self.communicator.size() != 2 {
            panic!("For ping pong, the communicator should have exactly 2 ranks");
        }
    }
}