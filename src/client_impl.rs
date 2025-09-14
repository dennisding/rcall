
use tokio;

use crate::Bichannel;

enum Message {
    Connect(String, i32), // ip, port
    Connected,
    Disconnect,
    Disconnected,
    SendPacket(crate::packer::Packet),
}

pub struct Client {
    channel: Bichannel<Message>
}

impl Client {
    pub fn new() -> Client {
        let (channel1, channel2) = crate::bichannel::Bichannel::<Message>::new();
        tokio::spawn(Self::processor(channel2));
        Client{
            channel: channel1
        }
    }

    pub fn connect(&mut self, ip: String, port: i32) {
//        tokio::spawn(Self::processor(ip, port));
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
                        client.connect(ip, port);
                    }
                    Message::SendPacket(_packet) => {
                    }
                    _ => {
                    }
                }
            }
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.disconnect();
    }
}

struct InnerClient {
    pub channel: Bichannel<Message>
}

impl InnerClient {
    pub fn new(channel: Bichannel<Message>) -> Self {
        InnerClient {
            channel
        }
    }

    pub fn connect(&mut self, ip: String, port: i32) {
    }
}