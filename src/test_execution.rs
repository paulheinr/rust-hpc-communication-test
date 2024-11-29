use crate::communicator::TestCommunicator;
use clap::Parser;
use derive_builder::Builder;
use rand::{Rng, SeedableRng};

#[derive(Parser, Debug, Clone, Default)]
pub struct BasicArguments {
    #[arg(short, long, default_value_t = 90_000)]
    pub iterations: u32,
    #[arg(short, long, default_value_t = 5_000)]
    pub log_interval: u32,
    #[arg(short, long, default_value_t = 1024)]
    pub message_len: u32,
    #[arg(short, long)]
    pub reporting_file: Option<String>,
}

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct TestExecution<C> {
    communicator: C,
    arguments: BasicArguments,
}

impl<C: TestCommunicator> TestExecution<C> {
    pub fn new(communicator: C, arguments: BasicArguments) -> Self {
        TestExecution {
            communicator,
            arguments,
        }
    }

    pub fn ping_pong_client(&self) {
        self.check_ping_pong();
        let other = 1;

        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let message: Vec<u8> = (0..self.arguments.message_len)
            .map(|_| rng.random::<u8>())
            .collect();

        let mut reporting = Vec::with_capacity(self.arguments.iterations as usize);

        //Measure elapsed time
        let start = std::time::Instant::now();
        for i in 0..self.arguments.iterations {
            if i % self.arguments.log_interval == 0 {
                println!("=== Client in iteration {} ===", i);
            }
            let start_i = std::time::Instant::now();
            self.communicator.send(&message, other);
            let in_buffer = &mut vec![0; message.len()];
            self.communicator.recv(in_buffer, other);
            let elapsed_i = start_i.elapsed();

            if let Some(ref _reporting_file) = self.arguments.reporting_file {
                reporting.push(elapsed_i.as_nanos());
            }
        }

        let elapsed = start.elapsed();
        println!("Elapsed time: {:?}", elapsed);

        if let Some(ref _reporting_file) = self.arguments.reporting_file {
            self.write_reporting_csv(&mut reporting);
        }
    }

    pub fn ping_pong_server(&self) {
        self.check_ping_pong();
        let other = 0;

        for i in 0..self.arguments.iterations {
            if i % self.arguments.log_interval == 0 {
                println!("=== Server in iteration {} ===", i);
            }
            let in_buffer = &mut vec![0; self.arguments.message_len as usize];
            self.communicator.recv(in_buffer, other);
            self.communicator.send(in_buffer, other);
        }
    }

    pub fn barrier(&self) {
        self.communicator.barrier();
    }

    fn check_ping_pong(&self) {
        if self.communicator.size() != 2 {
            panic!("For ping pong, the communicator should have exactly 2 ranks");
        }
    }

    //save reporting as csv with header: index, elapsed time
    fn write_reporting_csv(&self, reporting: &mut Vec<u128>) {
        let mut wtr =
            csv::Writer::from_path(self.arguments.reporting_file.as_ref().unwrap()).unwrap();
        wtr.write_record(&["index", "elapsed time"]).unwrap();
        for (i, elapsed_i) in reporting.iter().enumerate() {
            wtr.write_record(&[i.to_string(), elapsed_i.to_string()])
                .unwrap();
        }
    }
}
