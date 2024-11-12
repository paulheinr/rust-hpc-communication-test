use std::net::{IpAddr, SocketAddr};
use std::sync::mpsc::{Receiver, Sender};

use tokio::net::UdpSocket;

pub trait Communicator {
    fn rank(&self) -> u32;
    fn send(&self, buffer: &[u8], dest: u32);
    fn recv(&self, buffer: &mut [u8], source: u32);
}

pub struct TokioCommunicator {
    rank: u32,
    socket: UdpSocket,
    receiver: Vec<SocketAddr>,
}

impl Communicator for TokioCommunicator {
    fn rank(&self) -> u32 {
        self.rank
    }

    fn send(&self, buffer: &[u8], dest: u32) {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            self.socket
                .send_to(buffer, self.receiver[dest as usize]).await
        }).expect("TODO: panic message");
    }

    fn recv(&self, buffer: &mut [u8], _source: u32) {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            self.socket.recv_from(buffer).await
        }).expect("TODO: panic message");
        //let (len, _) = self.socket.recv_from(buffer).await.unwrap();
        //buffer[len..].iter_mut().for_each(|x| *x = 0);
    }
}

impl TokioCommunicator {
    pub async fn create_n_2_n(n: u32, rank: u32) -> TokioCommunicator {
        let socket = UdpSocket::bind(SocketAddr::new(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)), (8080 + rank) as u16)).await.unwrap();
        let receiver = (0..n)
            .map(|i| SocketAddr::new(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)), (8080 + i) as u16))
            .collect();

        TokioCommunicator {
            rank,
            socket,
            receiver,
        }
    }
}

pub struct ChannelSimCommunicator {
    rank: u32,
    senders: Vec<Sender<Vec<u8>>>,
    receivers: Receiver<Vec<u8>>,
}

impl Communicator for ChannelSimCommunicator {
    fn rank(&self) -> u32 {
        self.rank
    }

    fn send(&self, buffer: &[u8], dest: u32) {
        println!("Sending from {} to {}", self.rank, dest);
        self.senders[dest as usize].send(buffer.to_vec()).unwrap();
    }

    fn recv(&self, buffer: &mut [u8], _source: u32) {
        println!("Receiving at {}", self.rank);
        buffer.copy_from_slice(&self.receivers.recv().unwrap());
    }
}

impl ChannelSimCommunicator {
    pub fn create_n_2_n(n: u32) -> Vec<ChannelSimCommunicator> {
        let mut senders = Vec::new();
        let mut comms = Vec::new();

        //create n communicators with a receiver. Temporarily store the senders in a vector.
        for n in 0..n {
            let (sender, receiver) = std::sync::mpsc::channel();
            senders.push(sender);
            let comm = ChannelSimCommunicator {
                rank: n,
                senders: vec![],
                receivers: receiver,
            };
            comms.push(comm);
        }

        //clone the senders and assign them to the communicators.
        for comm in &mut comms {
            let mut cloned_senders = Vec::new();
            for sender in senders.iter() {
                cloned_senders.push(sender.clone());
            }
            comm.senders = cloned_senders;
        }

        comms
    }
}
