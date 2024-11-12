use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tokio::net::UdpSocket;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

    let iter = 10000;
    // Bind to any available address and port for the client
    let client_socket = runtime.block_on(UdpSocket::bind("127.0.0.1:8081")).unwrap(); //.await?;

    // Define the server address to send "ping" to
    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    // Send "ping" to the server
    for i in 0..iter {
        println!("Iteration: {}", i);
        let message = b"ping";
        // client_socket.send_to(message, server_addr).await?;
        runtime.block_on(client_socket.send_to(message, server_addr))?;
        // println!("Sent 'ping' to server at {}", server_addr);

        let mut buf = [0; 1024];
        // Wait for "pong" response from the server
        // let (len, _) = tokio::time::timeout(Duration::from_secs(1), client_socket.recv_from(&mut buf)).await??;
        let (len, _) = runtime.block_on(async {
            //tokio::time::sleep(Duration::from_secs(1)).await;
            client_socket.recv_from(&mut buf).await
        })?;
        let response = std::str::from_utf8(&buf[..len]).unwrap();

        // println!("Received '{}' from server", response);
    }

    Ok(())
}