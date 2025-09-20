
use crate::ClientServices;
use crate::RpcDispatcher;

use tokio;
use tokio::io::{AsyncReadExt};
use tokio::sync::mpsc::{self, Sender};
use tokio::net::tcp::OwnedReadHalf;

pub enum Message {
    Connect(String, i32), // ip, port
    Connected,
    Disconnect,
    Disconnected,
    SendPacket(crate::Packet),
    ReceivePacket(crate::Packet)
}

pub struct ClientSender {
    sender: mpsc::Sender<Message>
}

impl ClientSender {
    pub fn new(sender: mpsc::Sender<Message>) -> Self {
        ClientSender {
            sender,
        }
    }
}

impl crate::Sender for ClientSender {
    fn send(&mut self, packet: crate::Packet) {
        if let Err(err) = self.sender.try_send(Message::SendPacket(packet)) {
            println!("error in client send: err = {}", err);
        }
    }

    fn close(&mut self) {
        if let Err(err) = self.sender.try_send(Message::Disconnect) {
            println!("error in cient try_send: err = {}", err);
        }
    }
}

pub struct Client<T: ClientServices> {
    sender: mpsc::Sender<Message>,
    receiver: mpsc::Receiver<Message>,
    services: T,
    dispatcher: T::DispatcherType,
    runtime: tokio::runtime::Runtime,
    writer: Option<tokio::net::tcp::OwnedWriteHalf>
}

impl<T: ClientServices> Client<T> {
    pub fn new(mut services: T) -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let (sender, receiver) = mpsc::channel::<Message>(crate::CHANNEL_SIZE);
        let dispatcher = services.new_dispatcher(ClientSender::new(sender.clone()));
        Client {
            sender,
            receiver,
            services,
            dispatcher,
            runtime,
            writer: None
        }
    }

    pub fn new_sender(&mut self) -> ClientSender {
        ClientSender::new(self.sender.clone())
    }

    pub fn connect(&mut self, ip: String, port: i32) {
        self.clean_connection();

        let addr = format!("{}:{}", ip, port);
        let stream = self.runtime.block_on(async {
            tokio::net::TcpStream::connect(addr).await.unwrap()
        });
        let (reader, writer) = stream.into_split();
        self.writer = Some(writer);

        self.runtime.spawn(client_reader(self.sender.clone(), reader));
    }

    pub fn poll(&mut self) {
        // not connected!!!
        if let None = self.writer {
            return;
        }

        loop {
            if let Ok(msg) = self.receiver.try_recv() {
                self.dispatch_message(msg);
            }
            else {
                break;
            }
        }
    }

    pub fn block_poll(&mut self) {
        if let None = self.writer {
            return;
        }

        loop {
            if let Some(msg) = self.receiver.blocking_recv() {
                self.dispatch_message(msg);
            }
            else {
                break;
            }
        }
    }

    fn clean_connection(&mut self) {
        self.writer = None;
    }

    fn dispatch_message(&mut self, msg: Message) {
        match msg {
            Message::Connected => {
                self.services.on_connected(&mut self.dispatcher);
            }
            Message::SendPacket(packet) => {
                self.send_packet(packet);
            }
            Message::Disconnect => {
                self.on_disconnect();
            }
            Message::Disconnected => {
                self.on_disconnected();
            }
            Message::ReceivePacket(packet) => {
                self.dispatcher.dispatch_rpc(packet);
            }
            _ => {
                println!("unhandle message");
            }
        }
    }

    /// 主动关闭连接
    fn on_disconnect(&mut self) {
        self.services.on_disconnected(&mut self.dispatcher);
        self.writer = None;
        self.receiver.close();
    }

    fn on_disconnected(&mut self) {
        self.services.on_disconnected(&mut self.dispatcher);
        self.receiver.close();
        self.writer = None;
//        self.receiver.close();
    }

    fn send_packet(&mut self, packet: crate::Packet) {
        if let Some(writer) = &self.writer {
            let size = packet.buffer.len() as crate::PacketLenType;
            if let Err(err) = writer.try_write(&size.to_ne_bytes()) {
                println!("error in sending packet: err={}", err);
                return;
            }

            if let Err(err) = writer.try_write(&packet.buffer) {
                println!("err in send packet data: {}", err);
            }
        }
    }
}

async fn client_reader(sender: Sender<Message>, mut reader: OwnedReadHalf) {
    if let Err(err) = sender.send(Message::Connected).await {
        println!("error in send Message::Connected: {}", err);
    }

    loop {
        let mut buffer = [0u8; std::mem::size_of::<crate::PacketLenType>()];
        if let Err(_err) = reader.read_exact(&mut buffer).await {
            break;
        }
        let len = <crate::PacketLenType>::from_ne_bytes(buffer);
        let mut packet = crate::Packet::new(len as usize);

        if let Err(_err) = reader.read_exact(&mut packet.buffer).await {
            break;
        }

        if let Err(err) = sender.send(Message::ReceivePacket(packet)).await {
            println!("error in send Message::ReceivePacket: err = {}", err);
            break;
        }
    }
    if let Err(_err) = sender.send(Message::Disconnected).await {
        // 主动关闭会出现发送失败的情况.
    }
}