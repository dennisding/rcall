

use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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

pub trait ClientServices {
    fn on_connected(&mut self);
    fn on_disconnected(&mut self);
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

    pub fn send(&mut self, packet: crate::Packet) {
        if let Err(err) = self.sender.try_send(Message::SendPacket(packet)) {
            println!("error in client send: err = {}", err);
        }
    }
}

pub struct Client<T: ClientServices + crate::RpcDispatcher> {
    sender: mpsc::Sender<Message>,
    receiver: mpsc::Receiver<Message>,
    dispatcher: Option<T>,
    runtime: tokio::runtime::Runtime,
    writer: Option<tokio::net::tcp::OwnedWriteHalf>
}

impl<T: ClientServices + crate::RpcDispatcher> Client<T> {
    pub fn new() -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let (sender, receiver) = mpsc::channel::<Message>(crate::CHANNEL_SIZE);
        Client {
            sender,
            receiver,
            dispatcher: None,
            runtime,
            writer: None
        }
    }

    pub fn set_dispatcher(&mut self, dispatcher: T) {
        self.dispatcher = Some(dispatcher);
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

    fn clean_connection(&mut self) {
        if let Some(writer) = &mut self.writer {
            self.writer = None;
        }
    }

    fn dispatch_message(&mut self, msg: Message) {
        match msg {
            Message::Connected => {
                self.dispatcher.as_mut().unwrap().on_connected();
            }
            Message::SendPacket(packet) => {
                self.send_packet(packet);
            }
            Message::ReceivePacket(packet) => {
                self.dispatcher.as_mut().unwrap().dispatch_rpc(packet);
            }
            _ => {
                println!("unhandle message");
            }
        }
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

        if let Err(err) = sender.send(Message::ReceivePacket(packet)).await {
            println!("error in send Message::ReceivePacket: err = {}", err);
        }
    }
    if let Err(err) = sender.send(Message::Disconnected).await {
        println!("error in send Message::Disconnect: err = {}", err);
    }
}