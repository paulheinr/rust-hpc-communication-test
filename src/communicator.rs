use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::sync::mpsc::{Receiver, Sender};

use tokio::runtime::Runtime;

pub trait Communicator {
    fn rank(&self) -> u32;
    fn size(&self) -> u32;
    fn send(&self, buffer: &[u8], dest: u32);
    fn recv(&self, buffer: &mut [u8], source: u32);
}

pub struct TokioCommunicator {
    rank: u32,
    socket: tokio::net::UdpSocket,
    receiver: Vec<SocketAddr>,
    runtime: Runtime,
}

impl Communicator for TokioCommunicator {
    fn rank(&self) -> u32 {
        self.rank
    }

    fn size(&self) -> u32 {
        self.receiver.len() as u32
    }

    fn send(&self, buffer: &[u8], dest: u32) {
        self.runtime.block_on(self.send(buffer, dest));
    }

    fn recv(&self, buffer: &mut [u8], _source: u32) {
        self.runtime.block_on(self.recv(buffer, _source));
    }
}

impl TokioCommunicator {
    pub fn create_n_2_n(n: u32, rank: u32) -> TokioCommunicator {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        let socket = runtime.block_on(tokio::net::UdpSocket::bind(SocketAddr::new(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)), (8080 + rank) as u16))).unwrap();

        let receiver = (0..n)
            .map(|i| SocketAddr::new(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)), (8080 + i) as u16))
            .collect();

        TokioCommunicator {
            rank,
            socket,
            receiver,
            runtime,
        }
    }

    async fn recv(&self, buffer: &mut [u8], _source: u32) {
        self.socket.recv_from(buffer).await.unwrap();
    }

    async fn send(&self, buffer: &[u8], dest: u32) {
        self.socket
            .send_to(buffer, self.receiver[dest as usize]).await.unwrap();
    }
}

pub struct StdCommunicator {
    rank: u32,
    socket: UdpSocket,
    receiver: Vec<SocketAddr>,
}

impl Communicator for StdCommunicator {
    fn rank(&self) -> u32 {
        self.rank
    }

    fn size(&self) -> u32 {
        self.receiver.len() as u32
    }

    fn send(&self, buffer: &[u8], dest: u32) {
        self.socket.send_to(buffer, self.receiver[dest as usize]).unwrap();
    }

    fn recv(&self, buffer: &mut [u8], _source: u32) {
        self.socket.recv_from(buffer).unwrap();
    }
}

impl StdCommunicator {
    pub fn create_n_2_n(n: u32, rank: u32) -> StdCommunicator {
        let socket = UdpSocket::bind(SocketAddr::new(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)), (8080 + rank) as u16)).unwrap();
        let receiver = (0..n)
            .map(|i| SocketAddr::new(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)), (8080 + i) as u16))
            .collect();

        StdCommunicator {
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

    fn size(&self) -> u32 {
        self.senders.len() as u32
    }

    fn send(&self, buffer: &[u8], dest: u32) {
        self.senders[dest as usize].send(buffer.to_vec()).unwrap();
    }

    fn recv(&self, buffer: &mut [u8], _source: u32) {
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
