use std::str;

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind to the server address and port
    let server_addr = "127.0.0.1:8080";
    let socket = UdpSocket::bind(server_addr).await?;
    println!("Server listening on {}", server_addr);

    let mut buf = [0; 1024];
    loop {
        // Receive a message from a client
        let (len, client_addr) = socket.recv_from(&mut buf).await?;
        let received_msg = str::from_utf8(&buf[..len]).unwrap();

        println!("Received '{}' from {}", received_msg, client_addr);

        // If the message is "ping", respond with "pong"
        if received_msg == "ping" {
            let response = b"pong";
            socket.send_to(response, client_addr).await?;
            println!("Sent 'pong' to {}", client_addr);
        }
    }
}
