
use tokio::{self, io::AsyncWriteExt};

use crate::{Bichannel, network};

enum Message {
    Connect(String, i32), // ip, port
    Connected,
    Disconnect,
    Disconnected,
    SendPacket(crate::packer::Packet),
}

pub struct Client<T: network::Client> {
    channel: Bichannel<Message>,
    client: T,
}

impl<T: 'static + network::Client> Client<T> {
    pub fn new(client: T) -> Self {
        let (channel1, channel2) = crate::Bichannel::<Message>::new();
        tokio::spawn(Self::processor(channel2));
        Client{
            channel: channel1,
            client
        }
    }

    pub fn connect(&mut self, ip: String, port: i32) {
        let _ = self.channel.send(Message::Connect(ip, port));
    }

    pub fn disconnect(&self) {
        // blocking reading
        let _ = self.channel.send(Message::Disconnect);
    }

    pub fn send(&self, packet: super::packer::Packet) {
        let _ = self.channel.send(Message::SendPacket(packet));
    }

    async fn processor(channel: Bichannel<Message>) {
        let mut client = InnerClient::new(channel);
        loop {
            if let Some(message) = client.channel.recv().await {
                match message {
                    Message::Connect(ip, port) => {
                        client.connect(ip, port).await;
                    }
                    Message::SendPacket(packet) => {
                        client.send_packet(packet).await;
                    }
                    _ => {
                    }
                }
            }
        }
    }
}

// impl<T: network::Client> Drop for Client<T> {
//     fn drop(&mut self) {
//         self.disconnect();
//     }
// }

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
        } else {
            self.stream = None;
        }
    }

    pub async fn send_packet(&mut self, packet: crate::packer::Packet) {
        if let Some(stream) = &mut self.stream {
            let _ = stream.write_all(&packet.buffer[..]).await;
        }
    }
}