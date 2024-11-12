use tokio::net::UdpSocket;

//#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let iter = 10000;

    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

    // Bind to the server address and port
    let server_addr = "127.0.0.1:8080";
    // let socket = UdpSocket::bind(server_addr).await?;
    let socket = runtime.block_on(UdpSocket::bind(server_addr)).unwrap();
    println!("Server listening on {}", server_addr);

    let mut buf = [0; 1024];
    let mut count = 0;
    loop {
        println!("Iteration: {}", count);

        // Receive a message from a client
        // let (len, client_addr) = socket.recv_from(&mut buf).await?;
        let (len, client_addr) = runtime.block_on(socket.recv_from(&mut buf))?;
        let received_msg = std::str::from_utf8(&buf[..len]).unwrap();

        // println!("Received '{}' from {}", received_msg, client_addr);

        // If the message is "ping", respond with "pong"
        if received_msg == "ping" {
            let response = b"pong";
            // socket.send_to(response, client_addr).await?;
            runtime.block_on(socket.send_to(response, client_addr))?;
            // println!("Sent 'pong' to {}", client_addr);
        }
        count += 1;
        if count == iter { return Ok(()); }
    }
}