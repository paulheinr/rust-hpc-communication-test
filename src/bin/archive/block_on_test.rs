#[tokio::main]
async fn main() {
    for i in 0..1000 {
        async { println!("From async: {}", i) }.await;
    }
}