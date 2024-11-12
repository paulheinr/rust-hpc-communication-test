use crate::communicator::Communicator;

pub fn ping_pong_client<C: Communicator>(iter: u32, communicator: C, message: &[u8]) {
    let other = 1;

    for i in 0..iter {
        println!("=== Client in iteration {} ===", i);
        communicator.send(message, other);
        let in_buffer = &mut vec![0; message.len()];
        communicator.recv(in_buffer, other);
    }
}

pub fn server<C: Communicator>(iter: u32, communicator: C, length: usize) {
    let other = 0;

    for _ in 0..iter {
        //println!("Server in iteration {}", i);
        let in_buffer = &mut vec![0; length];
        communicator.recv(in_buffer, other);
        communicator.send(in_buffer, other);
    }
}