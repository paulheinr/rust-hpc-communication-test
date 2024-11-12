use rust_hpc_communication_test::communicator::TokioCommunicator;
use rust_hpc_communication_test::constants::MESSAGE;
use rust_hpc_communication_test::test_execution::ping_pong_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let communicator = TokioCommunicator::create_n_2_n(2, 0).await;
    ping_pong_client(10, communicator, MESSAGE);
    Ok(())
    //
    // // Bind to any available address and port for the client
    // let client_socket = UdpSocket::bind("127.0.0.1:8081").await?;
    //
    // // Define the server address to send "ping" to
    // let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    //
    // // Send "ping" to the server
    // let message = b"ping";
    // client_socket.send_to(message, server_addr).await?;
    // println!("Sent 'ping' to server at {}", server_addr);
    //
    // let mut buf = [0; 1024];
    // // Wait for "pong" response from the server
    // let (len, _) = tokio::time::timeout(Duration::from_secs(1), client_socket.recv_from(&mut buf)).await??;
    // let response = str::from_utf8(&buf[..len]).unwrap();
    //
    // println!("Received '{}' from server", response);
    //
    // Ok(())
}
