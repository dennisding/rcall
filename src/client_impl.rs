
use tokio::{self, io::AsyncWriteExt};

use crate::{bichannel, network, Bichannel};

pub enum Message {
    Test,
    Connect(String, i32), // ip, port
    Connected,
    Disconnect,
    Disconnected,
    SendPacket(crate::packer::Packet),
}

pub struct Client<T: network::Client> {
    channel: Bichannel<Message>,
    processor: Option<T>,
}

pub struct Sender {
    sender: bichannel::Sender<Message>
}

impl Sender {
    pub fn new(sender: bichannel::Sender<Message>) -> Self {
        Sender {
            sender
        }
    }

    pub fn send(&self, packet: crate::packer::Packet) {
        println!("send packet message!!!!");
        self.sender.send(Message::SendPacket(packet));
    }
}

async fn processor(channel: Bichannel<Message>) {
    let mut client = InnerClient::new(channel);
    loop {
        if let Some(message) = client.channel.recv().await {
            match message {
                Message::Connect(ip, port) => {
                    println!("connect: {}: {}", ip, port);
                    client.connect(ip, port).await;
                }
                Message::SendPacket(packet) => {
                    println!("send packet!!!");
                    client.send_packet(packet).await;
                }
                Message::Test => {
                    println!("test");
                }
                _ => {
                    println!("unhandle msg!");
                }
            }
        }
    }
}

impl<T: network::Client> Client<T> {
    pub fn new() -> Self {
        let (channel1, channel2) = crate::Bichannel::<Message>::new(512);
        tokio::spawn(processor(channel2));
        Client {
            channel: channel1,
            processor: None
        }
    }

    pub fn new_sender(&self) -> Sender {
        Sender::new(self.channel.clone_sender())
    }

    pub fn set_processor(&mut self, processor: T) {
        self.processor = Some(processor);
    }

    pub fn connect(&self, ip: String, port: i32) {
        if let Err(err) = self.channel.send(Message::Connect(ip, port)) {
            println!("error in connect:{}", err);
        }
    }

    pub fn disconnect(&self) {
        // blocking reading
        if let Err(err) = self.channel.send(Message::Disconnect) {
            println!("error in disconnect:{}", err);
        }
    }

    pub fn send(&self, packet: super::packer::Packet) {
        let _ = self.channel.send(Message::SendPacket(packet));
    }

    pub async fn tick(&mut self) {
        match self.channel.recv().await {
            Some(Message::Disconnected) => {
                println!("disconnected");
            }
            Some(Message::Test) => {
                println!("test");
            }
            Some(Message::Connected) => {
                if let Some(processor) = &mut self.processor {
                    processor.on_connected();
                }
            }
            _ => {
                println!("unhandle message");
//                println!("unhandle");
            }
        }
    }
}

struct InnerClient {
    pub channel: Bichannel<Message>,
    stream: Option<tokio::net::TcpStream>,
}

impl InnerClient {
    pub fn new(channel: Bichannel<Message>) -> Self {
        InnerClient {
            channel,
            stream: None
        }
    }

    pub async fn connect(&mut self, ip: String, port: i32) {
        let addr = format!("{}:{}", ip, port);
        let stream_result = tokio::net::TcpStream::connect(addr).await;
        if let Ok(stream) = stream_result {
            self.stream = Some(stream);
            let _ = self.channel.send(Message::Connected);
        } else {
            println!("unable to connect to: {}", format!("{}:{}", ip, port));
            self.stream = None;
        }
    }

    pub async fn send_packet(&mut self, packet: crate::packer::Packet) {
        println!("send packet: len: {}", packet.buffer.len() as i32);
        if let Some(stream) = &mut self.stream {
            let _ = stream.write_i32(packet.buffer.len() as i32).await;
            let _ = stream.write_all(&packet.buffer[..]).await;
        }
    }
}