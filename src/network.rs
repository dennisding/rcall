use crate::packer;
use std::net::SocketAddr;
use std::collections::HashMap;

use tokio;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc;

use crate::packer::{PackTo, UnpackFrom};

pub trait Server {
    type ConnectType : RpcDispatcher;
    fn on_connected(&self, connection: &Self::ConnectType);
    fn on_disconnected(&self, connection: &Self::ConnectType);
    fn new_connection(&self) -> Self::ConnectType;
}

pub trait Client {
    fn on_connected(&mut self);
    fn on_disconnected(&mut self);
}

enum Message {
    Accept,
    Quit,
    Connected(Connection),
    Disconnected(usize),
    PacketReceived(usize, packer::Packet)
}

struct ConnectionInfo<T: RpcDispatcher> {
    processor: T,
    connection: Connection
}

impl<T: RpcDispatcher> ConnectionInfo<T> {
    pub fn new(processor: T, connection: Connection) -> Self {
        ConnectionInfo {
            processor,
            connection
        }
    }
}

pub struct Services<T: Server> {
    sender: mpsc::Sender<Message>,
    receiver: mpsc::Receiver<Message>,
    connections: HashMap<usize, ConnectionInfo<T::ConnectType>>,
    server: Option<T>
}

impl<T: Server + 'static> Services<T> {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<Message>(1024);
        Services {
            sender,
            receiver,
            connections: HashMap::new(),
            server: None
        }
    }

    pub fn serve_at(&mut self, port: i32) {
        tokio::spawn(Self::listen(port, self.sender.clone()));
    }

    async fn do_serve(&mut self, port: i32, services: T ) {
        loop {
            let result = self.receiver.blocking_recv();
            match result {
                Some(Message::Connected(connection)) => {
                    self.new_connection(connection);
                },
                Some(Message::PacketReceived(connect_id, packet)) => {
                    self.on_packet_received(connect_id, packet).await;
                },
                Some(Message::Disconnected(connect_id)) => {
                },
                _ => {
                    println!("unhandle message!");
                }
            }
        }
    }

    pub async fn serve_forever(&mut self, port: i32, services: T) {
        self.server = Some(services);

        // let runtime = tokio::runtime::Runtime::new().unwrap();

        // runtime.spawn(Self::listen(port, self.sender.clone()));
        tokio::spawn(Self::listen(port, self.sender.clone()));

        loop {
            let result = self.receiver.recv().await;
            match result {
                Some(Message::Connected(connection)) => {
                    self.new_connection(connection);
                },
                Some(Message::PacketReceived(connect_id, packet)) => {
                    self.on_packet_received(connect_id, packet).await;
                },
                Some(Message::Disconnected(connect_id)) => {
                },
                _ => {
                    println!("unhandle message!");
                }
            }
        }
    }

    fn new_connection(&mut self, connection: Connection) {
        println!("new_connection: [{}]{}", connection.id, connection.addr);
        let processor = self.server.as_ref().unwrap().new_connection();
        let info = ConnectionInfo::new(processor, connection);

        self.connections.insert(info.connection.id, info);
        // self.connections.insert(connection.id, connection);
    }

    async fn on_packet_received(&mut self, connect_id: usize, packet: packer::Packet) {
        println!("on_packet_received: {}, length: {}", connect_id, packet.buffer.len());
        if let Some(info) = self.connections.get_mut(&connect_id) {
            info.processor.dispatch_rpc(1, packet).await;
            //info.connection.on_packet_received(packet);
//            connection.on_packet_received(packet);
        } else {
            println!("invalid connect id: {}", connect_id);
        }
    }

    async fn listen(port: i32, sender: mpsc::Sender<Message>) {
        let addr = format!("127.0.0.1:{}", port);
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

        let mut index: usize = 100;
        loop {
            let id = index;
            index = index + 1;

            let (stream, addr) = listener.accept().await.unwrap();
            let (reader, writer) = stream.into_split();

            let connection = Connection::new(id, addr, writer);
            let _ = sender.send(Message::Connected(connection)).await;

            tokio::spawn(Self::process_read(id, reader, sender.clone()));
        }
    }

    async fn process_read(connect_id: usize, mut reader: OwnedReadHalf, sender: mpsc::Sender<Message>) {
        loop {
            let len_result = reader.read_i32().await;
            if let Ok(len) = len_result {
                let mut packet = packer::Packet::new(len as usize);

                let read_result = reader.read_exact(packet.buffer.as_mut_slice()).await;
                if let Ok(_) = read_result {
                    let _ = sender.send(Message::PacketReceived(connect_id, packet)).await;
                } else {
                    let _ = sender.send(Message::Disconnected(connect_id)).await;
                }
            }
            else {
                break;
            }
        }
    }
}

pub struct Connection {
    id: usize,
    writer: OwnedWriteHalf,
    addr: SocketAddr,
}

impl Connection {
    pub fn new(id: usize, addr: SocketAddr, writer: OwnedWriteHalf) -> Self {
        Connection {
            id,
            addr,
            writer,
        }
    }
    
    pub fn disconnect(&mut self) {
    }
}

pub trait RpcDispatcher {
    async fn dispatch_rpc(&mut self, rpc_id: i32, packet: crate::packer::Packet);
}

#[macro_export]
macro_rules! services {
    ($client: expr, $server: path) => {{
        let services = rcall::network::Services::new();

        services
    }};
}

macro_rules! client {
    () => {

    }
}
