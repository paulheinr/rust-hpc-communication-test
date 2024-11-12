use crate::communicator::Communicator;

pub fn ping_pong_client<C: Communicator>(iter: u32, communicator: C, message: &[u8]) {
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
    let other = 0;

    for i in 0..iter {
        println!("=== Server in iteration {} ===", i);
        let in_buffer = &mut vec![0; length];
        communicator.recv(in_buffer, other);
        communicator.send(in_buffer, other);
    }
}