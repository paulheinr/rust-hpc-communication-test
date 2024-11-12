use crate::communicator::Communicator;

pub fn ping_pong_client<C: Communicator>(iter: u32, communicator: C, message: &[u8]) {
    check_ping_pong(&communicator);
    let other = 1;

    //Measure elapsed time
    let start = std::time::Instant::now();
    for i in 0..iter {
        println!("=== Client in iteration {} ===", i);
        communicator.send(message, other);
        let in_buffer = &mut vec![0; message.len()];
        communicator.recv(in_buffer, other);
    }
    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
}

pub fn ping_pong_server<C: Communicator>(iter: u32, communicator: C, length: usize) {
    check_ping_pong(&communicator);
    let other = 0;

    for i in 0..iter {
        println!("=== Server in iteration {} ===", i);
        let in_buffer = &mut vec![0; length];
        communicator.recv(in_buffer, other);
        communicator.send(in_buffer, other);
    }
}

fn check_ping_pong<C: Communicator>(communicator: &C) {
    if communicator.size() {
        panic!("For ping pong, the communicator should have exactly 2 ranks");
    }
}