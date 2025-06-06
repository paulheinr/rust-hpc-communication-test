use std::thread;
use std::time::Duration;

fn main() {
    let mut v = Vec::new();
    for u in 0..10 {
        let h = thread::spawn(move || {
            println!("Hi From Thread {}", u);
            thread::sleep(Duration::from_millis(1000));
            println!("Bye From Thread {}", u);
        });
        v.push(h);
    }
    for h in v {
        h.join().unwrap();
    }
}