
use paste;

use crate::ServerServices;
use crate::RpcDispatcher;

use std::net::SocketAddr;
use std::collections::HashMap;

use tokio::sync::mpsc;
use tokio::io::{AsyncReadExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub enum Message {
    Connected(Connection),
    Disconnected(crate::ConnectId),
    ReceivePacket(crate::ConnectId, crate::Packet),
    SendPacket(crate::ConnectId, crate::Packet)
}

pub struct Server<T: ServerServices> {
    sender: mpsc::Sender<Message>,
    receiver: mpsc::Receiver<Message>,
    services: T,
    connections: HashMap<crate::ConnectId, ConnectionInfo<T::ConnectionType>>
}

impl<T: ServerServices> Server<T> {
    pub fn new(services: T) -> Self {
        let (sender, receiver) = mpsc::channel::<Message>(crate::CHANNEL_SIZE);
        Server {
            sender,
            receiver,
            services,
            connections: HashMap::new()
        }
    }

    pub fn serve_forever_at(&mut self, port: i32) {
        let runtime = tokio::runtime::Runtime::new().unwrap();

        runtime.spawn(listen_at(port, self.sender.clone()));

        loop { // loop forever
            if let Some(msg) = self.receiver.blocking_recv() {
                self.dispatch_message(msg);
            }
        }
    }

    fn dispatch_message(&mut self, message: Message) {
        match message {
            Message::Connected(connection) => {
                self.on_connected(connection);
            }
            Message::Disconnected(connect_id) => {
                println!("disconnected: {}", connect_id);
            }
            Message::ReceivePacket(connect_id, packet) => {
                self.on_receive_packet(connect_id, packet);
            }
            Message::SendPacket(connect_id, packet) => {
                self.on_send_packet(connect_id, packet);
            }
        }
    }

    fn on_connected(&mut self, connection: Connection) {
        let dispatcher = self.services.new_connection(&connection);
        let connect_id = connection.id;
        let info = ConnectionInfo::new(connection, dispatcher);
        self.connections.insert(connect_id, info);

        let info = self.connections.get_mut(&connect_id).unwrap();
        self.services.on_connected(info);
    }

    fn on_send_packet(&mut self, connect_id: crate::ConnectId, packet: crate::Packet) {
        match self.connections.get_mut(&connect_id) {
            Some(info) => {
                let size = packet.buffer.len() as crate::PacketLenType;
                if let Err(err) = info.connection.writer.try_write(&size.to_ne_bytes()) {
                    println!("erro in sending packet len: {}", err);
                    return;
                }
                if let Err(err) = info.connection.writer.try_write(&packet.buffer) {
                    println!("error in sending packet: {}", err);
                }
            }
            None => {

            }
        }
    }

    fn on_receive_packet(&mut self, connect_id: crate::ConnectId, packet: crate::Packet) {
        match self.connections.get_mut(&connect_id) {
            Some(info) => {
                info.dispatcher.dispatch_rpc(packet);
            }
            None => {
                println!("invalid connect id: {}", connect_id);
            }
        }
    }
}

pub struct ServerSender {
    id: crate::ConnectId,
    sender: mpsc::Sender<Message>
}

impl ServerSender {
    pub fn new(id: crate::ConnectId, sender: mpsc::Sender<Message>) -> Self {
        ServerSender {
            id,
            sender
        }
    }
}

impl crate::Sender for ServerSender {
    fn send(&mut self, packet: crate::Packet) {
        if let Err(err) = self.sender.try_send(Message::SendPacket(self.id, packet)) {
            println!("error in ServerSender::send {}", err);
        }
    }
}

pub struct Connection {
    pub id: crate::ConnectId,
    writer: OwnedWriteHalf,
    pub addr: SocketAddr,
    sender: mpsc::Sender<Message>
}

impl Connection {
    pub fn new(id: crate::ConnectId, 
        addr: SocketAddr, 
        writer: OwnedWriteHalf, 
        sender: mpsc::Sender<Message>) -> Self 
    {
        Connection {
            id,
            addr,
            writer,
            sender
        }
    }
    
    pub fn disconnect(&self) {
    }

    pub fn new_sender(&self) -> ServerSender {
        ServerSender::new(self.id, self.sender.clone())
    }
}

pub struct ConnectionInfo<T: crate::RpcDispatcher> {
    pub connection: Connection,
    pub dispatcher: T
}

impl<T: crate::RpcDispatcher> ConnectionInfo<T> {
    pub fn new(connection: Connection, dispatcher: T) -> Self {
        ConnectionInfo {
            connection,
            dispatcher
        }
    }
}

async fn listen_at(port: i32, sender: mpsc::Sender<Message>) {
    let addr = format!("127.0.0.1:{}", port);
    println!("server listen at:{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let mut index: usize = crate::CONNECT_ID_BEGIN;
    loop {
        let id = index;
        index = index + 1;

        let (stream, addr) = listener.accept().await.unwrap();
        let (reader, writer) = stream.into_split();

        let connection = Connection::new(id, addr, writer, sender.clone());
        let _ = sender.send(Message::Connected(connection)).await;

        tokio::spawn(process_read(id, reader, sender.clone()));
    }
}

async fn process_read(connect_id: usize, mut reader: OwnedReadHalf, sender: mpsc::Sender<Message>) {
    loop {
        let mut buffer = [0u8; std::mem::size_of::<crate::PacketLenType>()];
        if let Err(err) = reader.read_exact(&mut buffer).await {
            println!("error in read packet len: {}", err);
            break;
        }
        let len = <crate::PacketLenType>::from_ne_bytes(buffer);
        let mut packet = crate::Packet::new(len as usize);

        if let Err(err) = reader.read_exact(&mut packet.buffer).await {
            println!("error in read packet data:{}", err);
            break;
        }

        if let Err(err) = sender.send(Message::ReceivePacket(connect_id, packet)).await {
            println!("error in send Message::ReceivedPacket: connect_id = {}, err = {}", connect_id, err);
        }
    }
    if let Err(err) = sender.send(Message::Disconnected(connect_id)).await {
        println!("error in send Message::Disconnected: connect_id = {}, err = {}", connect_id, err);
    }
    paste::paste!()
}

// protocols::ImplInServer_Remote<rcall::ClientSender>
// #[macro_export]
// macro_rules! client_remote {
//     ($path:path) => {
// //        paste::paste!{[<protocols::ImplInServer _Remote>]<rcall::ClientSender>}
//         // paste::paste!(
//         //      [<$path _Remote>]<rcall::ClientSender>
//         // )
//     };
//     () => {

//     }
// }