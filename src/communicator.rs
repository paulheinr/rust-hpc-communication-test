use clap::Parser;
use mpi::collective::CommunicatorCollectives;
use mpi::point_to_point::{Destination, Source};
use mpi::topology::{Communicator, SimpleCommunicator};
use mpi::Rank;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Barrier};
use tokio::runtime::Runtime;

#[derive(Parser, Debug, Clone, Default)]
pub struct UdpArguments {
    #[arg(short, long)]
    pub server_address: String,
}

pub trait TestCommunicator {
    fn rank(&self) -> u32;
    fn size(&self) -> u32;
    fn send(&self, buffer: &[u8], dest: u32);
    fn recv(&self, buffer: &mut [u8], source: u32);
    fn barrier(&self);
}

pub struct MpiCommunicator {
    comm: SimpleCommunicator,
}

impl TestCommunicator for MpiCommunicator {
    fn rank(&self) -> u32 {
        self.comm.rank() as u32
    }

    fn size(&self) -> u32 {
        self.comm.size() as u32
    }

    fn send(&self, buffer: &[u8], dest: u32) {
        self.comm.process_at_rank(dest as Rank).send(buffer);
    }

    fn recv(&self, buffer: &mut [u8], source: u32) {
        self.comm
            .process_at_rank(source as Rank)
            .receive_into(buffer);
    }

    fn barrier(&self) {
        self.comm.barrier();
    }
}

impl MpiCommunicator {
    pub fn create(comm: SimpleCommunicator) -> MpiCommunicator {
        MpiCommunicator { comm }
    }
}

pub struct TokioCommunicator {
    rank: u32,
    socket: tokio::net::UdpSocket,
    receiver: Vec<SocketAddr>,
    runtime: Runtime,
}

impl TestCommunicator for TokioCommunicator {
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

    fn barrier(&self) {
        unimplemented!()
    }
}

impl TokioCommunicator {
    pub fn create_n_2_n(n: u32, rank: u32) -> TokioCommunicator {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let socket = runtime
            .block_on(tokio::net::UdpSocket::bind(SocketAddr::new(
                IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
                (8080 + rank) as u16,
            )))
            .unwrap();

        let receiver = (0..n)
            .map(|i| {
                SocketAddr::new(
                    IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
                    (8080 + i) as u16,
                )
            })
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
            .send_to(buffer, self.receiver[dest as usize])
            .await
            .unwrap();
    }
}

pub struct StdCommunicator {
    rank: u32,
    socket: UdpSocket,
    receiver: Vec<SocketAddr>,
}

impl TestCommunicator for StdCommunicator {
    fn rank(&self) -> u32 {
        self.rank
    }

    fn size(&self) -> u32 {
        self.receiver.len() as u32
    }

    fn send(&self, buffer: &[u8], dest: u32) {
        self.socket
            .send_to(buffer, self.receiver[dest as usize])
            .unwrap();
    }

    fn recv(&self, buffer: &mut [u8], _source: u32) {
        let (_, _addr) = self.socket.recv_from(buffer).unwrap();
    }

    fn barrier(&self) {
        unimplemented!()
    }
}

impl StdCommunicator {
    pub fn create_n_2_n(n: u32, rank: u32) -> StdCommunicator {
        let socket = UdpSocket::bind(SocketAddr::new(
            IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
            (8080 + rank) as u16,
        ))
        .unwrap();
        let receiver = (0..n)
            .map(|i| {
                SocketAddr::new(
                    IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
                    (8080 + i) as u16,
                )
            })
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
    barrier: Arc<Barrier>,
}

impl TestCommunicator for ChannelSimCommunicator {
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

    fn barrier(&self) {
        self.barrier.wait();
    }
}

impl ChannelSimCommunicator {
    pub fn create_n_2_n(n: u32) -> Vec<ChannelSimCommunicator> {
        let mut senders = Vec::new();
        let mut comms = Vec::new();
        let barrier = Arc::new(Barrier::new(n as usize));

        //create n communicators with a receiver. Temporarily store the senders in a vector.
        for n in 0..n {
            let (sender, receiver) = std::sync::mpsc::channel();
            senders.push(sender);
            let comm = ChannelSimCommunicator {
                rank: n,
                senders: vec![],
                receivers: receiver,
                barrier: barrier.clone(),
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
